use crate::diag::{Hint, HintedStrResult};
use crate::foundations::{Content, Packed, ShowSet, Smart, StyleChain, Styles, elem};
use crate::introspection::{Locatable, Tagged};
use crate::layout::{BlockElem, Em};
use crate::model::DocumentElem;
use crate::text::{FontWeight, TextElem, TextSize};

/// 文書のタイトル。
///
/// 文書全体のメインタイトルを表示するために使用し、1つの文書につき1度だけ登場すべきです。
/// 一方、レベル1の[見出し]($heading)は、文書の最上位のセクションに使用することを意図しています。
///
/// タイトルとともに表示される追加のフロントマター（著者リストなど）は、本文には含めないでください。
///
/// HTMLエクスポートでは、これは`h1`要素として表示され、レベル1の見出しは`h2`要素として表示されます。
///
/// # 例
/// ```example
/// #set document(
///   title: [Interstellar Mail Delivery]
/// )
///
/// #title()
///
/// = Introduction
/// In recent years, ...
/// ```
#[elem(Locatable, Tagged, ShowSet)]
pub struct TitleElem {
    /// タイトルの内容。
    ///
    /// 省略された場合（または`{auto}`の場合）、これは[`document.title`]がデフォルトとなります。
    /// この場合、`{set document(title: [..])}`によって事前に文書のタイトルが設定されている必要があります。
    ///
    /// ```example
    /// #set document(title: "Course ABC, Homework 1")
    /// #title[Homework 1]
    ///
    /// ...
    /// ```
    #[positional]
    pub body: Smart<Content>,
}

impl TitleElem {
    pub fn resolve_body(&self, styles: StyleChain) -> HintedStrResult<Content> {
        match self.body.get_cloned(styles) {
            Smart::Auto => styles
                .get_cloned(DocumentElem::title)
                .ok_or("document title was not set")
                .hint("set the title with `set document(title: [...])`")
                .hint("or provide an explicit body with `title[..]`"),
            Smart::Custom(body) => Ok(body),
        }
    }
}

impl ShowSet for Packed<TitleElem> {
    fn show_set(&self, _styles: StyleChain) -> Styles {
        const SIZE: Em = Em::new(1.7);
        const ABOVE: Em = Em::new(1.125);
        const BELOW: Em = Em::new(0.75);

        let mut out = Styles::new();
        out.set(TextElem::size, TextSize(SIZE.into()));
        out.set(TextElem::weight, FontWeight::BOLD);
        out.set(BlockElem::above, Smart::Custom(ABOVE.into()));
        out.set(BlockElem::below, Smart::Custom(BELOW.into()));
        out.set(BlockElem::sticky, true);
        out
    }
}
