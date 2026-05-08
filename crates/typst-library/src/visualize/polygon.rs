use std::f64::consts::PI;

use typst_syntax::Span;

use crate::foundations::{Content, NativeElement, Smart, elem, func, scope};
use crate::layout::{Axes, Em, Length, Rel};
use crate::visualize::{FillRule, Paint, Stroke};

/// 閉じた多角形。
///
/// 多角形は頂点で定義され、自動的に閉じられます。
///
/// # 例
/// ```example
/// #polygon(
///   fill: blue.lighten(80%),
///   stroke: blue,
///   (20%, 0pt),
///   (60%, 0pt),
///   (80%, 2cm),
///   (0%,  2cm),
/// )
/// ```
#[elem(scope)]
pub struct PolygonElem {
    /// 多角形の塗りつぶし方法。
    ///
    /// fillを設定すると、デフォルトのストロークは消えます。塗りつぶしと
    /// ストロークの両方を持つ矩形を作るには、両方を設定する必要があります。
    pub fill: Option<Paint>,

    /// 多角形の塗りつぶしに使用する描画規則。
    ///
    /// 例については[curveのドキュメント]($curve.fill-rule)を参照してください。
    #[default]
    pub fill_rule: FillRule,

    /// 多角形の[ストローク]($stroke)の方法。
    ///
    /// ストロークを無効にするには `{none}` を設定でき、塗りつぶしが指定されて
    /// いない場合に限り `{1pt}` の黒のストロークにするには `{auto}` を設定できます。
    #[fold]
    pub stroke: Smart<Option<Stroke>>,

    /// 多角形の頂点。各点は2つの[相対長さ]($relative)からなる配列として
    /// 指定されます。
    #[variadic]
    pub vertices: Vec<Axes<Rel<Length>>>,
}

#[scope]
impl PolygonElem {
    /// 正多角形。サイズと頂点数で定義されます。
    ///
    /// ```example
    /// #polygon.regular(
    ///   fill: blue.lighten(80%),
    ///   stroke: blue,
    ///   size: 30pt,
    ///   vertices: 3,
    /// )
    /// ```
    #[func(title = "Regular Polygon")]
    pub fn regular(
        span: Span,

        /// 多角形の塗りつぶし方法。詳細は一般的な
        /// [polygonのドキュメント]($polygon.fill)を参照してください。
        #[named]
        fill: Option<Option<Paint>>,

        /// 多角形のストロークの方法。詳細は一般的な
        /// [polygonのドキュメント]($polygon.stroke)を参照してください。
        #[named]
        stroke: Option<Smart<Option<Stroke>>>,

        /// 正多角形の[外接円](https://en.wikipedia.org/wiki/Circumcircle)の直径。
        #[named]
        #[default(Em::one().into())]
        size: Length,

        /// 多角形の頂点数。
        #[named]
        #[default(3)]
        vertices: u64,
    ) -> Content {
        let radius = size / 2.0;
        let angle = |i: f64| {
            2.0 * PI * i / (vertices as f64) + PI * (1.0 / 2.0 - 1.0 / vertices as f64)
        };
        let (horizontal_offset, vertical_offset) = (0..=vertices)
            .map(|v| {
                (
                    (radius * angle(v as f64).cos()) + radius,
                    (radius * angle(v as f64).sin()) + radius,
                )
            })
            .fold((radius, radius), |(min_x, min_y), (v_x, v_y)| {
                (
                    if min_x < v_x { min_x } else { v_x },
                    if min_y < v_y { min_y } else { v_y },
                )
            });
        let vertices = (0..=vertices)
            .map(|v| {
                let x = (radius * angle(v as f64).cos()) + radius - horizontal_offset;
                let y = (radius * angle(v as f64).sin()) + radius - vertical_offset;
                Axes::new(x, y).map(Rel::from)
            })
            .collect();

        let mut elem = PolygonElem::new(vertices);
        if let Some(fill) = fill {
            elem.fill.set(fill);
        }
        if let Some(stroke) = stroke {
            elem.stroke.set(stroke);
        }
        elem.pack().spanned(span)
    }
}
