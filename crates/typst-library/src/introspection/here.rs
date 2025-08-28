use comemo::Tracked;

use crate::diag::HintedStrResult;
use crate::foundations::{func, Context};
use crate::introspection::Location;

/// 文書中における現在位置を提供。
///
/// `here`はアクティブな[context]から現在位置を直接取得する低レベルな構成要素と考えることができます。
/// 他のいくつかの関数は内部で使用しています。
/// 例えば、`{counter.get()}`は`{counter.at(here())}`と等価です。
///
/// Within show rules on [locatable]($location/#locatable) 要素に対するshowルールにおいて, `{here()}`will match the location of the shown element.
///
/// 現在のページ番号を表示したい場合は、[`counter`]型のドキュメントを参照してください。
/// `here`は物理的なページ番号を決定できますが、通常は、前書きの後にリセットされるような、論理的なページ番号が必要でしょう。
///
/// # 例
/// Determining the current position in the document in combination with the
/// [`position`]($location.position) method:
/// ```example
/// #context [
///   I am located at
///   #here().position()
/// ]
/// ```
///
/// Running a [query] for elements before the current position:
/// ```example
/// = Introduction
/// = Background
///
/// There are
/// #context query(
///   selector(heading).before(here())
/// ).len()
/// headings before me.
///
/// = Conclusion
/// ```
/// Refer to the [`selector`] type for more details on before/after selectors.
#[func(contextual)]
pub fn here(context: Tracked<Context>) -> HintedStrResult<Location> {
    context.location()
}
