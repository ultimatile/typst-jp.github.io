use crate::foundations::elem;
use crate::layout::{Abs, Angle, Axes, Length, Rel};
use crate::visualize::Stroke;

/// ある点から別の点までの線。
///
/// # 例
/// ```example
/// #set page(height: 100pt)
///
/// #line(length: 100%)
/// #line(end: (50%, 50%))
/// #line(
///   length: 4cm,
///   stroke: 2pt + maroon,
/// )
/// ```
#[elem]
pub struct LineElem {
    /// 線の始点。
    ///
    /// ちょうど2つの相対長さからなる配列でなければなりません。
    pub start: Axes<Rel<Length>>,

    /// 線の終点。
    pub end: Option<Axes<Rel<Length>>>,

    /// 線の長さ。これは `end` が `{none}` の場合にのみ反映されます。
    #[default(Abs::pt(30.0).into())]
    pub length: Rel<Length>,

    /// 線が原点から離れる方向の角度。これは `end` が `{none}` の場合にのみ
    /// 反映されます。
    pub angle: Angle,

    /// 線の[ストローク]($stroke)の方法。
    ///
    /// ```example
    /// #set line(length: 100%)
    /// #stack(
    ///   spacing: 1em,
    ///   line(stroke: 2pt + red),
    ///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: ("dot", 2pt, 4pt, 2pt))),
    /// )
    /// ```
    #[fold]
    pub stroke: Stroke,
}
