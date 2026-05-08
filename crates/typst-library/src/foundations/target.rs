use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::foundations::{Cast, Context, elem, func};

/// The export target.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Target {
    /// The target that is used for paged, fully laid-out content.
    #[default]
    Paged,
    /// The target that is used for HTML export.
    Html,
}

impl Target {
    /// Whether this is the HTML target.
    pub fn is_html(self) -> bool {
        self == Self::Html
    }
}

/// この要素は`target`スタイルチェーンフィールドを保持するためだけに存在します。
/// 実際に構築されることはなく、ユーザーには見えません。
#[elem]
pub struct TargetElem {
    /// コンパイルターゲット。
    pub target: Target,
}

/// 現在のエクスポートターゲットを返します。
///
/// この関数は次のいずれかを返します。
/// - `{"paged"}`（PDF、PNG、SVGエクスポートの場合）
/// - `{"html"}`（HTMLエクスポートの場合）
///
/// この関数の設計はまだ確定しておらず、そのため`html`フィーチャーで保護されています。
/// 詳細は[HTMLドキュメントページ]($html)を参照してください。
///
/// # 用途
/// この関数を用いると、HTMLとページエクスポートの両ターゲットで適切に文書を整形できます。
/// 主にテンプレートやshowルールで使用し、コンテンツ内では直接使わないことが推奨されます。
/// こうすることで、文書の内容はエクスポートターゲットに依存せず、
/// PDFとHTMLのエクスポートでコンテンツを共有できます。
///
/// # ターゲットの変化
/// この関数は[コンテキスト依存]($context)です。
/// 単一のコンパイル内でもターゲットが変化することがあります。
/// HTMLにエクスポートする場合、[`html.frame`]内では`{"paged"}`になります。
///
/// # 例
/// ```example
/// #let kbd(it) = context {
///   if target() == "html" {
///     html.elem("kbd", it)
///   } else {
///     set text(fill: rgb("#1f2328"))
///     let r = 3pt
///     box(
///       fill: rgb("#f6f8fa"),
///       stroke: rgb("#d1d9e0b3"),
///       outset: (y: r),
///       inset: (x: r),
///       radius: r,
///       raw(it)
///     )
///   }
/// }
///
/// Press #kbd("F1") for help.
/// ```
#[func(contextual)]
pub fn target(context: Tracked<Context>) -> HintedStrResult<Target> {
    Ok(context.styles()?.get(TargetElem::target))
}
