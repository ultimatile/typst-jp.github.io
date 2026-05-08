use ecow::{EcoString, eco_format};
use typst_utils::{PicoStr, ResolvedPicoStr};

use crate::diag::StrResult;
use crate::foundations::{Repr, Str, bail, func, scope, ty};

/// 要素のラベル。
///
/// コンテンツにラベルを挿入すると、空白ではない直前の要素に紐付けられます。
/// 直前の要素はラベルと同じスコープ内になければなりません。
/// 例えば、`[Hello #[<label>]]`は機能しません。
///
/// ラベルを付けた要素は、そのラベルを通して[参照]($ref)、[クエリ]($query)、
/// [スタイル設定]($styling)が可能です。
///
/// ラベルを構築した後は、[`str`]($str/#constructor)を用いて名前を取得できます。
///
/// # 例
/// ```example
/// #show <a>: set text(blue)
/// #show label("b"): set text(red)
///
/// = Heading <a>
/// *Strong* #label("b")
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。山括弧で名前を囲むことでラベルを作成できます。
/// マークアップとコードの両方で使えます。
/// ラベル名には、英字、数字、`_`、`-`、`:`、`.`を含められます。
/// ラベルは空にはできません。
///
/// この関数で専用の構文を使う場合、構文上の差異がある点に注意してください。
/// 以下のコードでは、`[<a>]`は見出しを終端させるため、見出し自体に紐付けられますが、
/// `[#label("b")]`は見出しの一部となるため、見出しのテキストに紐付けられます。
///
/// ```typ
/// // Equivalent to `#heading[Introduction] <a>`.
/// = Introduction <a>
///
/// // Equivalent to `#heading[Conclusion #label("b")]`.
/// = Conclusion #label("b")
/// ```
///
/// 現時点では、ラベルはマークアップモードの要素にのみ付けられ、コードモードでは付けられません。
/// この仕様は将来変更されるかもしれません。
#[ty(scope, cast)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label(PicoStr);

impl Label {
    /// Creates a label from an interned string.
    ///
    /// Returns `None` if the given string is empty.
    pub fn new(name: PicoStr) -> Option<Self> {
        const EMPTY: PicoStr = PicoStr::constant("");
        (name != EMPTY).then_some(Self(name))
    }

    /// Resolves the label to a string.
    pub fn resolve(self) -> ResolvedPicoStr {
        self.0.resolve()
    }

    /// Turns this label into its inner interned string.
    pub fn into_inner(self) -> PicoStr {
        self.0
    }
}

#[scope]
impl Label {
    /// 文字列からラベルを生成します。
    #[func(constructor)]
    pub fn construct(
        /// ラベルの名前。
        ///
        /// [専用構文](#syntax)とは異なり、このコンストラクターは
        /// 特殊文字を含む名前も含めて、任意の空でない文字列を受け付けます。
        name: Str,
    ) -> StrResult<Label> {
        if name.is_empty() {
            bail!("label name must not be empty");
        }

        Ok(Self(PicoStr::intern(name.as_str())))
    }
}

impl Repr for Label {
    fn repr(&self) -> EcoString {
        let resolved = self.resolve();
        if typst_syntax::is_valid_label_literal_id(&resolved) {
            eco_format!("<{resolved}>")
        } else {
            eco_format!("label({})", resolved.repr())
        }
    }
}

impl From<Label> for PicoStr {
    fn from(value: Label) -> Self {
        value.into_inner()
    }
}

/// Indicates that an element cannot be labelled.
pub trait Unlabellable {}
