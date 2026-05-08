use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};
use std::sync::Arc;

use ecow::{EcoString, eco_format};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typst_syntax::is_ident;
use typst_utils::ArcExt;

use crate::diag::{Hint, HintedStrResult, StrResult};
use crate::foundations::{
    Array, Module, Repr, Str, Value, array, cast, func, repr, scope, ty,
};

/// Create a new [`Dict`] from key-value pairs.
#[macro_export]
#[doc(hidden)]
macro_rules! __dict {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = $crate::foundations::IndexMap::default();
        $(map.insert($key.into(), $crate::foundations::IntoValue::into_value($value));)*
        $crate::foundations::Dict::from(map)
    }};
}

#[doc(inline)]
pub use crate::__dict as dict;

/// 文字列キーから値へのマップ。
///
/// `key: value`のペアをカンマで区切って括弧で囲むことで辞書を構築できます。
/// 値は同じ型でなくても構いません。空の括弧はすでに空の配列を表すため、
/// 空の辞書を作るには専用の構文`(:)`を使う必要があります。
///
/// 辞書は概念的には[配列]($array)に似ていますが、整数の代わりに文字列でインデックスされます。
/// `.at()`メソッドで辞書のエントリにアクセスしたり作成したりできます。
/// キーが静的に分かっている場合は、代わりに[フィールドアクセス記法]($scripting/#fields)
/// （`.key`）で値にアクセスすることもできます。辞書にキーが含まれているかを
/// 確認するには、`in`キーワードを使います。
///
/// 辞書のペアは[forループ]($scripting/#loops)で反復処理できます。
/// ペアは最初に挿入または宣言された順に反復処理されます。
///
/// 辞書は`+`演算子で加算したり[結合]($scripting/#blocks)したりできます。
/// `..spread`演算子で関数呼び出しや別の辞書[^1]に
/// [スプレッド]($arguments/#spreading)することもできます。
/// いずれの場合も、キーが複数回現れた場合は、最後の値が他を上書きします。
///
/// # 例
/// ```example
/// #let dict = (
///   name: "Typst",
///   born: 2019,
/// )
///
/// #dict.name \
/// #(dict.launch = 20)
/// #dict.len() \
/// #dict.keys() \
/// #dict.values() \
/// #dict.at("born") \
/// #dict.insert("city", "Berlin")
/// #("name" in dict)
/// ```
///
/// [^1]: 辞書にスプレッドする際、括弧内の全ての項目がスプレッドである場合は、
/// 専用の構文`(:..spread)`を使う必要があります。そうでなければ配列にスプレッドされます。
#[ty(scope, cast, name = "dictionary")]
#[derive(Default, Clone, PartialEq)]
pub struct Dict(Arc<IndexMap<Str, Value, FxBuildHasher>>);

impl Dict {
    /// Create a new, empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the value at the given key.
    pub fn get(&self, key: &str) -> StrResult<&Value> {
        self.0.get(key).ok_or_else(|| missing_key(key))
    }

    /// Mutably borrow the value the given `key` maps to.
    pub fn at_mut(&mut self, key: &str) -> HintedStrResult<&mut Value> {
        Arc::make_mut(&mut self.0)
            .get_mut(key)
            .ok_or_else(|| missing_key(key))
            .hint("use `insert` to add or update values")
    }

    /// Remove the value if the dictionary contains the given key.
    pub fn take(&mut self, key: &str) -> StrResult<Value> {
        Arc::make_mut(&mut self.0)
            .shift_remove(key)
            .ok_or_else(|| missing_key(key))
    }

    /// Whether the dictionary contains a specific key.
    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Clear the dictionary.
    pub fn clear(&mut self) {
        if Arc::strong_count(&self.0) == 1 {
            Arc::make_mut(&mut self.0).clear();
        } else {
            *self = Self::new();
        }
    }

    /// Iterate over pairs of references to the contained keys and values.
    pub fn iter(&self) -> indexmap::map::Iter<'_, Str, Value> {
        self.0.iter()
    }

    /// Check if there is any remaining pair, and if so return an
    /// "unexpected key" error.
    pub fn finish(&self, expected: &[&str]) -> StrResult<()> {
        let mut iter = self.iter().peekable();
        if iter.peek().is_none() {
            return Ok(());
        }
        let unexpected: Vec<&str> = iter.map(|kv| kv.0.as_str()).collect();

        Err(Self::unexpected_keys(unexpected, Some(expected)))
    }

    // Return an "unexpected key" error string.
    pub fn unexpected_keys(
        unexpected: Vec<&str>,
        hint_expected: Option<&[&str]>,
    ) -> EcoString {
        let format_as_list = |arr: &[&str]| {
            repr::separated_list(
                &arr.iter().map(|s| eco_format!("\"{s}\"")).collect::<Vec<_>>(),
                "and",
            )
        };

        let mut msg = String::from(match unexpected.len() {
            1 => "unexpected key ",
            _ => "unexpected keys ",
        });

        msg.push_str(&format_as_list(&unexpected[..]));

        if let Some(expected) = hint_expected {
            msg.push_str(", valid keys are ");
            msg.push_str(&format_as_list(expected));
        }

        msg.into()
    }
}

#[scope]
impl Dict {
    /// 値を辞書に変換します。
    ///
    /// この関数は辞書のような値を辞書に変換するためだけのものであり、
    /// 個別のペアから辞書を作成するためのものではない点に注意してください。
    /// その用途には辞書構文`(key: value)`を使ってください。
    ///
    /// ```example
    /// #dictionary(sys).at("version")
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 辞書に変換する値。
        value: ToDict,
    ) -> Dict {
        value.0
    }

    /// 辞書のペアの数。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 辞書内の指定したキーに対応付けられた値を返します。キーがすでに辞書に存在する場合は、
    /// 代入の左辺としても利用できます。キーが辞書にない場合はデフォルト値を返し、
    /// デフォルト値が指定されていなければエラーになります。
    #[func]
    pub fn at(
        &self,
        /// 項目を取得するキー。
        key: Str,
        /// キーが辞書にない場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.0
            .get(&key)
            .cloned()
            .or(default)
            .ok_or_else(|| missing_key_no_default(&key))
    }

    /// 辞書に新しいペアを挿入します。辞書がすでにこのキーを持っている場合、
    /// 値は更新されます。
    ///
    /// 複数のペアを一度に挿入するには、`+=`演算子で別の辞書を加えるだけです。
    #[func]
    pub fn insert(
        &mut self,
        /// 挿入するペアのキー。
        key: Str,
        /// 挿入するペアの値。
        value: Value,
    ) {
        Arc::make_mut(&mut self.0).insert(key, value);
    }

    /// キーで指定したペアを辞書から削除し、その値を返します。
    #[func]
    pub fn remove(
        &mut self,
        /// 削除するペアのキー。
        key: Str,
        /// キーが存在しない場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        Arc::make_mut(&mut self.0)
            .shift_remove(&key)
            .or(default)
            .ok_or_else(|| missing_key(&key))
    }

    /// 辞書のキーを挿入順の配列として返します。
    #[func]
    pub fn keys(&self) -> Array {
        self.0.keys().cloned().map(Value::Str).collect()
    }

    /// 辞書の値を挿入順の配列として返します。
    #[func]
    pub fn values(&self) -> Array {
        self.0.values().cloned().collect()
    }

    /// 辞書のキーと値をペアの配列として返します。各ペアは長さ2の配列として表されます。
    #[func]
    pub fn pairs(&self) -> Array {
        self.0
            .iter()
            .map(|(k, v)| Value::Array(array![k.clone(), v.clone()]))
            .collect()
    }
}

/// A value that can be cast to dictionary.
pub struct ToDict(Dict);

cast! {
    ToDict,
    v: Module => Self(v
        .scope()
        .iter()
        .map(|(k, b)| (Str::from(k.clone()), b.read().clone()))
        .collect()
    ),
}

impl Debug for Dict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl Repr for Dict {
    fn repr(&self) -> EcoString {
        if self.is_empty() {
            return "(:)".into();
        }

        let max = 40;
        let mut pieces: Vec<_> = self
            .iter()
            .take(max)
            .map(|(key, value)| {
                if is_ident(key) {
                    eco_format!("{key}: {}", value.repr())
                } else {
                    eco_format!("{}: {}", key.repr(), value.repr())
                }
            })
            .collect();

        if self.len() > max {
            pieces.push(eco_format!(".. ({} pairs omitted)", self.len() - max));
        }

        repr::pretty_array_like(&pieces, false).into()
    }
}

impl Add for Dict {
    type Output = Self;

    fn add(mut self, rhs: Dict) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Dict {
    fn add_assign(&mut self, rhs: Dict) {
        match Arc::try_unwrap(rhs.0) {
            Ok(map) => self.extend(map),
            Err(rc) => self.extend(rc.iter().map(|(k, v)| (k.clone(), v.clone()))),
        }
    }
}

impl Hash for Dict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.len());
        for item in self {
            item.hash(state);
        }
    }
}

impl Serialize for Dict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Dict {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(IndexMap::<Str, Value, FxBuildHasher>::deserialize(deserializer)?.into())
    }
}

impl Extend<(Str, Value)> for Dict {
    fn extend<T: IntoIterator<Item = (Str, Value)>>(&mut self, iter: T) {
        Arc::make_mut(&mut self.0).extend(iter);
    }
}

impl FromIterator<(Str, Value)> for Dict {
    fn from_iter<T: IntoIterator<Item = (Str, Value)>>(iter: T) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

impl IntoIterator for Dict {
    type Item = (Str, Value);
    type IntoIter = indexmap::map::IntoIter<Str, Value>;

    fn into_iter(self) -> Self::IntoIter {
        Arc::take(self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a Dict {
    type Item = (&'a Str, &'a Value);
    type IntoIter = indexmap::map::Iter<'a, Str, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<IndexMap<Str, Value, FxBuildHasher>> for Dict {
    fn from(map: IndexMap<Str, Value, FxBuildHasher>) -> Self {
        Self(Arc::new(map))
    }
}

/// The missing key access error message.
#[cold]
fn missing_key(key: &str) -> EcoString {
    eco_format!("dictionary does not contain key {}", key.repr())
}

/// The missing key access error message when no default was given.
#[cold]
fn missing_key_no_default(key: &str) -> EcoString {
    eco_format!(
        "dictionary does not contain key {} \
         and no default value was specified",
        key.repr()
    )
}
