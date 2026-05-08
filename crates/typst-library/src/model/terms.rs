use crate::diag::bail;
use crate::foundations::{
    Array, Content, NativeElement, Packed, Smart, Styles, cast, elem, scope,
};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{Em, HElem, Length};
use crate::model::{ListItemLike, ListLike};

/// 用語とその説明のリスト。
///
/// 用語とその説明を縦方向に並べて表示します。
/// 説明が複数行にわたる場合、視覚的な階層を伝えるためにぶら下げインデントが使用されます。
///
/// # 例
/// ```example
/// / Ligature: A merged glyph.
/// / Kerning: A spacing adjustment
///   between two adjacent letters.
/// ```
///
/// # 構文
/// この関数には専用の構文もあります。スラッシュで始まる行に、用語、コロン、説明を続けて記述すると、用語リストの項目が作成されます。
#[elem(scope, title = "Term List", Locatable, Tagged)]
pub struct TermsElem {
    /// 用語リストの[spacing]($terms.spacing)のデフォルト値を定義します。
    /// `{false}`の場合、項目は[段落間隔]($par.spacing)で離れて配置されます。
    /// `{true}`の場合は、代わりに[段落の行送り]($par.leading)が使われます。
    /// これによってリストがコンパクトになり、項目が短い場合により見栄えがよくなります。
    ///
    /// マークアップモードでは、このパラメーターの値は項目間に空行があるかどうかに基づいて決定されます。
    /// 項目が直接続いている場合は`{true}`に、項目が空行で区切られている場合は`{false}`に設定されます。
    /// マークアップで定義された詰め具合は、setルールで上書きできません。
    ///
    /// ```example
    /// / Fact: If a term list has a lot
    ///   of text, and maybe other inline
    ///   content, it should not be tight
    ///   anymore.
    ///
    /// / Tip: To make it wide, simply
    ///   insert a blank line between the
    ///   items.
    /// ```
    #[default(true)]
    pub tight: bool,

    /// 項目と説明の間のセパレーター。
    ///
    /// 項目と説明を一定の量の空白で区切りたいだけの場合は、
    /// セパレーターとして`{h(2cm, weak: true)}`を使い、`{2cm}`を任意の空白の量に置き換えてください。
    ///
    /// ```example
    /// #set terms(separator: [: ])
    ///
    /// / Colon: A nice separator symbol.
    /// ```
    #[default(HElem::new(Em::new(0.6).into()).with_weak(true).pack())]
    pub separator: Content,

    /// 各項目のインデント。
    pub indent: Length,

    /// 説明のぶら下げインデント。
    ///
    /// これは項目全体の`indent`に加えて適用されます。
    ///
    /// ```example
    /// #set terms(hanging-indent: 0pt)
    /// / Term: This term list does not
    ///   make use of hanging indents.
    /// ```
    #[default(Em::new(2.0).into())]
    pub hanging_indent: Length,

    /// 用語リストの項目間の間隔。
    ///
    /// `{auto}`が設定された場合、詰めた用語リストでは段落の[`leading`]($par.leading)、
    /// 詰めていない（widerな）用語リストでは段落の[`spacing`]($par.spacing)が使用されます。
    pub spacing: Smart<Length>,

    /// 用語リストの子要素。
    ///
    /// 用語リスト構文を使用すると、forループのような構文を介しても、隣接する項目は自動的に用語リストにまとめられます。
    ///
    /// ```example
    /// #for (year, product) in (
    ///   "1978": "TeX",
    ///   "1984": "LaTeX",
    ///   "2019": "Typst",
    /// ) [/ #product: Born in #year.]
    /// ```
    #[variadic]
    pub children: Vec<Packed<TermItem>>,

    /// 現在用語リスト内にいるかどうか。
    #[internal]
    #[ghost]
    pub within: bool,
}

#[scope]
impl TermsElem {
    #[elem]
    type TermItem;
}

/// 用語リストの項目。
#[elem(name = "item", title = "Term List Item", Tagged)]
pub struct TermItem {
    /// リスト項目で説明される用語。
    #[required]
    pub term: Content,

    /// 用語の説明。
    #[required]
    pub description: Content,
}

cast! {
    TermItem,
    array: Array => {
        let mut iter = array.into_iter();
        let (term, description) = match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), None) => (a.cast()?, b.cast()?),
            _ => bail!("array must contain exactly two entries"),
        };
        Self::new(term, description)
    },
    v: Content => v.unpack::<Self>().map_err(|_| "expected term item or array")?,
}

impl ListLike for TermsElem {
    type Item = TermItem;

    fn create(children: Vec<Packed<Self::Item>>, tight: bool) -> Self {
        Self::new(children).with_tight(tight)
    }
}

impl ListItemLike for TermItem {
    fn styled(mut item: Packed<Self>, styles: Styles) -> Packed<Self> {
        item.term.style_in_place(styles.clone());
        item.description.style_in_place(styles);
        item
    }
}
