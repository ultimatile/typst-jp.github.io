use crate::foundations::{Content, elem};
use crate::introspection::Tagged;

/// レイアウトに影響を与えないコンテンツの隠蔽。
///
/// `hide`関数を用いると、レイアウトにコンテンツを「認識」させながらコンテンツを隠せます。
/// これは何らかのコンテンツと全く同じ大きさを持つ空白を作る際に便利です。
///
/// # 例
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
///
/// # 墨消し { #redaction }
/// この関数に渡した引数は視覚的に現れず、支援技術（AT）からも認識されません。
/// そのため、コンテンツの墨消しにも利用できます。
/// ただし、隠したコンテンツの痕跡が*いくらか*残ることもあります（PDFの文書アウトラインにブックマークとして残る見出しなど）。
///
/// また、状況によってはレイアウト上の大きさからコンテンツを推測されてしまう可能性があります。
/// そのため、機密性の非常に高い情報を隠す用途にこの関数を使用することは推奨しません。
#[elem(Tagged)]
pub struct HideElem {
    /// 隠したいコンテンツ。
    #[required]
    pub body: Content,

    /// This style is set on the content contained in the `hide` element.
    #[internal]
    #[ghost]
    pub hidden: bool,
}
