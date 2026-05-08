#[doc(inline)]
pub use typst_macros::{scope, ty};

use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::LazyLock;

use ecow::{EcoString, eco_format};
use typst_utils::Static;

use crate::diag::{DeprecationSink, StrResult, bail};
use crate::foundations::{
    AutoValue, Func, NativeFuncData, NoneValue, Repr, Scope, Value, cast, func,
};

/// 値の種類を表します。
///
/// 文書をスタイル設定するには、さまざまな種類の値を扱う必要があります。
/// 要素のサイズを指定するための長さ、テキストや図形のための色などです。
/// Typstはこれらを明確に定義された_型_に分類し、
/// どの型の値が必要とされるかを示します。
///
/// 数値の基本型や、プログラミング言語で[一般的に]($int)
/// [知られている]($float)[型]($str)（[配列]($array)、[辞書]($dictionary)など）
/// に加えて、Typstは[_コンテンツ_]($content)のための特別な型を提供します。
/// この型の値は、文書に入力できるあらゆるもの（テキスト、見出しや図形などの要素、
/// スタイル情報）を保持できます。
///
/// # 例
/// ```example
/// #let x = 10
/// #if type(x) == int [
///   #x is an integer!
/// ] else [
///   #x is another value...
/// ]
///
/// An image is of type
/// #type(image("glacier.jpg")).
/// ```
///
/// `{10}`の型は`int`です。では、`int`や`type`自体の型は何でしょうか？
/// ```example
/// #type(int) \
/// #type(type)
/// ```
///
/// `int`のような他の型と異なり、[none]と[auto]にはそれらを表す名前がありません。
/// 値がこれらのいずれかかどうかを調べるには、値を直接比較してください。例：
/// ```example
/// #let val = none
/// #if val == none [
///   Yep, it's none.
/// ]
/// ```
///
/// `type`は文書の全ての要素に対して[`content`]を返す点に注意してください。
/// 扱っているコンテンツの種類をプログラム的に判別するには、
/// [`content.func`]を参照してください。
#[ty(scope, cast)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type(Static<NativeTypeData>);

impl Type {
    /// Get the type for `T`.
    pub fn of<T: NativeType>() -> Self {
        T::ty()
    }

    /// The type's short name, how it is used in code (e.g. `str`).
    pub fn short_name(&self) -> &'static str {
        self.0.name
    }

    /// The type's long name, for use in diagnostics (e.g. `string`).
    pub fn long_name(&self) -> &'static str {
        self.0.long_name
    }

    /// The type's title case name, for use in documentation (e.g. `String`).
    pub fn title(&self) -> &'static str {
        self.0.title
    }

    /// Documentation for the type (as Markdown).
    pub fn docs(&self) -> &'static str {
        self.0.docs
    }

    /// Search keywords for the type.
    pub fn keywords(&self) -> &'static [&'static str] {
        self.0.keywords
    }

    /// This type's constructor function.
    pub fn constructor(&self) -> StrResult<Func> {
        self.0
            .constructor
            .as_ref()
            .map(|lazy| Func::from(*lazy))
            .ok_or_else(|| eco_format!("type {self} does not have a constructor"))
    }

    /// The type's associated scope that holds sub-definitions.
    pub fn scope(&self) -> &'static Scope {
        &(self.0).0.scope
    }

    /// Get a field from this type's scope, if possible.
    pub fn field(
        &self,
        field: &str,
        sink: impl DeprecationSink,
    ) -> StrResult<&'static Value> {
        match self.scope().get(field) {
            Some(binding) => Ok(binding.read_checked(sink)),
            None => bail!("type {self} does not contain field `{field}`"),
        }
    }
}

#[scope]
impl Type {
    /// 値の型を判定します。
    ///
    /// ```example
    /// #type(12) \
    /// #type(14.7) \
    /// #type("hello") \
    /// #type(<glacier>) \
    /// #type([Hi]) \
    /// #type(x => x + 1) \
    /// #type(type)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 型を判定する対象の値。
        value: Value,
    ) -> Type {
        value.ty()
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Type({})", self.long_name())
    }
}

impl Repr for Type {
    fn repr(&self) -> EcoString {
        if *self == Type::of::<AutoValue>() {
            "type(auto)"
        } else if *self == Type::of::<NoneValue>() {
            "type(none)"
        } else {
            self.short_name()
        }
        .into()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.long_name())
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.long_name().cmp(other.long_name())
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A Typst type that is defined by a native Rust type.
pub trait NativeType {
    /// The type's name.
    ///
    /// In contrast to `data()`, this is usable in const contexts.
    const NAME: &'static str;

    /// Get the type for the native Rust type.
    fn ty() -> Type {
        Type::from(Self::data())
    }

    // Get the type data for the native Rust type.
    fn data() -> &'static NativeTypeData;
}

/// Defines a native type.
#[derive(Debug)]
pub struct NativeTypeData {
    /// The type's normal name (e.g. `str`), as exposed to Typst.
    pub name: &'static str,
    /// The type's long name (e.g. `string`), for error messages.
    pub long_name: &'static str,
    /// The function's title case name (e.g. `String`).
    pub title: &'static str,
    /// The documentation for this type as a string.
    pub docs: &'static str,
    /// A list of alternate search terms for this type.
    pub keywords: &'static [&'static str],
    /// The constructor for this type.
    pub constructor: LazyLock<Option<&'static NativeFuncData>>,
    /// Definitions in the scope of the type.
    pub scope: LazyLock<Scope>,
}

impl From<&'static NativeTypeData> for Type {
    fn from(data: &'static NativeTypeData) -> Self {
        Self(Static(data))
    }
}

cast! {
    &'static NativeTypeData,
    self => Type::from(self).into_value(),
}
