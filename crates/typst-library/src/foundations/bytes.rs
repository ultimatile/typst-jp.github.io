use std::any::Any;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Deref};
use std::str::Utf8Error;
use std::sync::Arc;

use ecow::{EcoString, eco_format};
use serde::{Serialize, Serializer};
use typst_syntax::Lines;
use typst_utils::LazyHash;

use crate::diag::{StrResult, bail};
use crate::foundations::{Array, Reflect, Repr, Str, Value, cast, func, scope, ty};

/// バイト列。
///
/// これは概念的には`{0}`から`{255}`までの[整数]($int)の配列に似ていますが、
/// はるかに効率的に表現されています。[forループ]($scripting/#loops)で
/// 反復処理できます。
///
/// 以下の変換ができます。
/// - [`bytes`]コンストラクターを用いて、[文字列]($str)または整数の[配列]($array)を
///   バイト列に変換
/// - [`str`]コンストラクターを用いて、UTF-8エンコーディングでバイト列を
///   文字列に変換
/// - [`array`]コンストラクターを用いて、バイト列を整数の配列に変換
///
/// ファイルから[データを読み込む]($read)際に、文字列として読み込むか
/// 生のバイト列として読み込むかを選択できます。
///
/// ```example
/// #bytes((123, 160, 22, 0)) \
/// #bytes("Hello 😃")
///
/// #let data = read(
///   "rhino.png",
///   encoding: none,
/// )
///
/// // Magic bytes.
/// #array(data.slice(0, 4)) \
/// #str(data.slice(1, 4))
/// ```
#[ty(scope, cast)]
#[derive(Clone, Hash)]
#[allow(clippy::derived_hash_with_manual_eq)]
pub struct Bytes(Arc<LazyHash<dyn Bytelike>>);

impl Bytes {
    /// Create `Bytes` from anything byte-like.
    ///
    /// The `data` type will directly back this bytes object. This means you can
    /// e.g. pass `&'static [u8]` or `[u8; 8]` and no extra vector will be
    /// allocated.
    ///
    /// If the type is `Vec<u8>` and the `Bytes` are unique (i.e. not cloned),
    /// the vector will be reused when mutating to the `Bytes`.
    ///
    /// If your source type is a string, prefer [`Bytes::from_string`] to
    /// directly use the UTF-8 encoded string data without any copying.
    pub fn new<T>(data: T) -> Self
    where
        T: AsRef<[u8]> + Send + Sync + 'static,
    {
        Self(Arc::new(LazyHash::new(data)))
    }

    /// Create `Bytes` from anything string-like, implicitly viewing the UTF-8
    /// representation.
    ///
    /// The `data` type will directly back this bytes object. This means you can
    /// e.g. pass `String` or `EcoString` without any copying.
    pub fn from_string<T>(data: T) -> Self
    where
        T: AsRef<str> + Send + Sync + 'static,
    {
        Self(Arc::new(LazyHash::new(StrWrapper(data))))
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// Return a view into the bytes.
    pub fn as_slice(&self) -> &[u8] {
        self
    }

    /// Try to view the bytes as an UTF-8 string.
    ///
    /// If these bytes were created via `Bytes::from_string`, UTF-8 validation
    /// is skipped.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        self.inner().as_str()
    }

    /// Return a copy of the bytes as a vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    /// Try to turn the bytes into a `Str`.
    ///
    /// - If these bytes were created via `Bytes::from_string::<Str>`, the
    ///   string is cloned directly.
    /// - If these bytes were created via `Bytes::from_string`, but from a
    ///   different type of string, UTF-8 validation is still skipped.
    pub fn to_str(&self) -> Result<Str, Utf8Error> {
        match self.inner().as_any().downcast_ref::<Str>() {
            Some(string) => Ok(string.clone()),
            None => self.as_str().map(Into::into),
        }
    }

    /// Resolve an index or throw an out of bounds error.
    fn locate(&self, index: i64) -> StrResult<usize> {
        self.locate_opt(index).ok_or_else(|| out_of_bounds(index, self.len()))
    }

    /// Resolve an index, if it is within bounds.
    ///
    /// `index == len` is considered in bounds.
    fn locate_opt(&self, index: i64) -> Option<usize> {
        let len = self.as_slice().len();
        let wrapped =
            if index >= 0 { Some(index) } else { (len as i64).checked_add(index) };
        wrapped.and_then(|v| usize::try_from(v).ok()).filter(|&v| v <= len)
    }

    /// Access the inner `dyn Bytelike`.
    fn inner(&self) -> &dyn Bytelike {
        &**self.0
    }
}

#[scope]
impl Bytes {
    /// 値をバイト列に変換します。
    ///
    /// - 文字列はUTF-8でエンコードされます。
    /// - `{0}`から`{255}`までの整数の配列は直接変換されます。専用のバイト表現は
    ///   配列表現よりはるかに効率的なので、大きなバイトバッファ
    ///   （例えば画像データ）には通常こちらが使われます。
    ///
    /// ```example
    /// #bytes("Hello 😃") \
    /// #bytes((123, 160, 22, 0))
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// バイト列に変換する値。
        value: ToBytes,
    ) -> Bytes {
        value.0
    }

    /// バイト単位の長さ。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// 指定したインデックスのバイトを返します。インデックスが範囲外の場合は
    /// デフォルト値を返し、デフォルト値が指定されていない場合はエラーになります。
    #[func]
    pub fn at(
        &self,
        /// バイトを取得するインデックス。
        index: i64,
        /// インデックスが範囲外の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.locate_opt(index)
            .and_then(|i| self.as_slice().get(i).map(|&b| Value::Int(b.into())))
            .or(default)
            .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
    }

    /// バイト列のサブスライスを取り出します。開始または終了のインデックスが
    /// 範囲外の場合はエラーになります。
    #[func]
    pub fn slice(
        &self,
        /// 開始インデックス（このインデックスを含みます）。
        start: i64,
        /// 終了インデックス（このインデックスは含みません）。
        /// 省略した場合、末尾までのスライス全体が取り出されます。
        #[default]
        end: Option<i64>,
        /// 取り出す要素の数。`end`の位置として`start + count`を渡すのと等価です。
        /// `end`と同時には指定できません。
        #[named]
        count: Option<i64>,
    ) -> StrResult<Bytes> {
        let start = self.locate(start)?;
        let end = end.or(count.map(|c| start as i64 + c));
        let end = self.locate(end.unwrap_or(self.len() as i64))?.max(start);
        let slice = &self.as_slice()[start..end];

        // We could hold a view into the original bytes here instead of
        // making a copy, but it's unclear when that's worth it. Java
        // originally did that for strings, but went back on it because a
        // very small view into a very large buffer would be a sort of
        // memory leak.
        Ok(Bytes::new(slice.to_vec()))
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Bytes({})", self.len())
    }
}

impl Repr for Bytes {
    fn repr(&self) -> EcoString {
        eco_format!("bytes({})", self.len())
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.inner().as_bytes()
    }
}

impl Eq for Bytes {}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl Add for Bytes {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.is_empty() {
            // Nothing to do
        } else if self.is_empty() {
            *self = rhs;
        } else if let Some(vec) = Arc::get_mut(&mut self.0)
            .and_then(|unique| unique.as_any_mut().downcast_mut::<Vec<u8>>())
        {
            vec.extend_from_slice(&rhs);
        } else {
            *self = Self::new([self.as_slice(), rhs.as_slice()].concat());
        }
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.repr())
        } else {
            serializer.serialize_bytes(self)
        }
    }
}

impl TryFrom<&Bytes> for Lines<String> {
    type Error = Utf8Error;

    #[comemo::memoize]
    fn try_from(value: &Bytes) -> Result<Lines<String>, Utf8Error> {
        let text = value.as_str()?;
        Ok(Lines::new(text.to_string()))
    }
}

/// Any type that can back a byte buffer.
trait Bytelike: Send + Sync {
    fn as_bytes(&self) -> &[u8];
    fn as_str(&self) -> Result<&str, Utf8Error>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Bytelike for T
where
    T: AsRef<[u8]> + Send + Sync + 'static,
{
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }

    fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_ref())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Hash for dyn Bytelike {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

/// Makes string-like objects usable with `Bytes`.
struct StrWrapper<T>(T);

impl<T> Bytelike for StrWrapper<T>
where
    T: AsRef<str> + Send + Sync + 'static,
{
    fn as_bytes(&self) -> &[u8] {
        self.0.as_ref().as_bytes()
    }

    fn as_str(&self) -> Result<&str, Utf8Error> {
        Ok(self.0.as_ref())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A value that can be cast to bytes.
pub struct ToBytes(Bytes);

cast! {
    ToBytes,
    v: Str => Self(Bytes::from_string(v)),
    v: Array => Self(v.iter()
        .map(|item| match item {
            Value::Int(byte @ 0..=255) => Ok(*byte as u8),
            Value::Int(_) => bail!("number must be between 0 and 255"),
            value => Err(<u8 as Reflect>::error(value)),
        })
        .collect::<Result<Vec<u8>, _>>()
        .map(Bytes::new)?
    ),
    v: Bytes => Self(v),
}

/// The out of bounds access error message.
#[cold]
fn out_of_bounds(index: i64, len: usize) -> EcoString {
    eco_format!("byte index out of bounds (index: {index}, len: {len})")
}

/// The out of bounds access error message when no default value was given.
#[cold]
fn out_of_bounds_no_default(index: i64, len: usize) -> EcoString {
    eco_format!(
        "byte index out of bounds (index: {index}, len: {len}) \
         and no default value was specified",
    )
}
