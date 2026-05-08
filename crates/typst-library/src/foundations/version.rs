use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter, Write};
use std::hash::Hash;
use std::iter::repeat;

use ecow::{EcoString, EcoVec, eco_format};

use crate::diag::{StrResult, bail};
use crate::foundations::{Repr, cast, func, repr, scope, ty};

/// 任意の数の構成要素を持つバージョン。
///
/// 最初の3つの構成要素には名前があり、フィールドとして利用できます。
/// `major`、`minor`、`patch`の3つです。それ以降の構成要素には名前がありません。
///
/// 構成要素のリストは、意味的には無限に続くゼロのリストで拡張されています。
/// 例えば、`0.8`は`0.8.0`と同じです。特殊な場合として、
/// 構成要素を全く持たない空のバージョンは`0`、`0.0`、`0.0.0`などと同じです。
///
/// Typstコンパイラーの現在のバージョンは`sys.version`として利用できます。
///
/// [`array`]コンストラクターを用いると、バージョンを明示的に与えられた構成要素の
/// 配列に変換できます。
#[ty(scope, cast)]
#[derive(Debug, Default, Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Version(EcoVec<u32>);

impl Version {
    /// The names for the first components of a version.
    pub const COMPONENTS: [&'static str; 3] = ["major", "minor", "patch"];

    /// Create a new (empty) version.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a named component of a version.
    ///
    /// Always non-negative. Returns `0` if the version isn't specified to the
    /// necessary length.
    pub fn component(&self, name: &str) -> StrResult<i64> {
        self.0
            .iter()
            .zip(Self::COMPONENTS)
            .find_map(|(&i, s)| (s == name).then_some(i as i64))
            .ok_or_else(|| "unknown version component".into())
    }

    /// Push a component to the end of this version.
    pub fn push(&mut self, component: u32) {
        self.0.push(component);
    }

    /// The values of the version
    pub fn values(&self) -> &[u32] {
        &self.0
    }
}

#[scope]
impl Version {
    /// 新しいバージョンを生成します。
    ///
    /// 構成要素はいくつでも持てます（ゼロでも構いません）。
    ///
    /// ```example
    /// #version() \
    /// #version(1) \
    /// #version(1, 2, 3, 4) \
    /// #version((1, 2, 3, 4)) \
    /// #version((1, 2), 3)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// バージョンの構成要素（配列引数は平坦化されます）。
        #[variadic]
        components: Vec<VersionComponents>,
    ) -> Version {
        let mut version = Version::new();
        for c in components {
            match c {
                VersionComponents::Single(v) => version.push(v),
                VersionComponents::Multiple(values) => {
                    for v in values {
                        version.push(v);
                    }
                }
            }
        }
        version
    }

    /// バージョンの構成要素を取得します。
    ///
    /// 返される整数は常に非負です。バージョンが必要な長さまで指定されていない場合は
    /// `0`を返します。
    #[func]
    pub fn at(
        &self,
        /// 構成要素を取得するインデックス。負の値の場合は、明示的に与えられた
        /// 構成要素の末尾からのインデックスとなります。
        index: i64,
    ) -> StrResult<i64> {
        let mut index = index;
        if index < 0 {
            match (self.0.len() as i64).checked_add(index) {
                Some(pos_index) if pos_index >= 0 => index = pos_index,
                _ => bail!(
                    "component index out of bounds (index: {index}, len: {})",
                    self.0.len()
                ),
            }
        }
        Ok(usize::try_from(index)
            .ok()
            .and_then(|i| self.0.get(i).copied())
            .unwrap_or_default() as i64)
    }
}

impl FromIterator<u32> for Version {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        Self(EcoVec::from_iter(iter))
    }
}

impl IntoIterator for Version {
    type Item = u32;
    type IntoIter = ecow::vec::IntoIter<u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_len = self.0.len().max(other.0.len());
        let tail = repeat(&0);

        let self_iter = self.0.iter().chain(tail.clone());
        let other_iter = other.0.iter().chain(tail);

        for (l, r) in self_iter.zip(other_iter).take(max_len) {
            match l.cmp(r) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Version {}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut first = true;
        for &v in &self.0 {
            if !first {
                f.write_char('.')?;
            }
            write!(f, "{v}")?;
            first = false;
        }
        Ok(())
    }
}

impl Repr for Version {
    fn repr(&self) -> EcoString {
        let parts: Vec<_> = self.0.iter().map(|v| eco_format!("{v}")).collect();
        eco_format!("version{}", &repr::pretty_array_like(&parts, false))
    }
}

/// One or multiple version components.
pub enum VersionComponents {
    Single(u32),
    Multiple(Vec<u32>),
}

cast! {
    VersionComponents,
    v: u32 => Self::Single(v),
    v: Vec<u32> => Self::Multiple(v)
}
