use kurbo::ParamCurveExtrema;
use typst_macros::{Cast, scope};
use typst_utils::Numeric;

use crate::diag::{HintedStrResult, HintedString, bail};
use crate::foundations::{Content, Packed, Smart, cast, elem};
use crate::layout::{Abs, Axes, Length, Point, Rect, Rel, Size};
use crate::visualize::{FillRule, Paint, Stroke};

use super::FixedStroke;

/// 移動、直線、ベジェセグメントから構成される曲線。
///
/// 任意の時点において、概念的なペンまたはカーソルが存在します。
/// - moveの要素はカーソルを描画せずに移動させます。
/// - line/quadratic/cubicの要素は、カーソルから新しい位置までセグメントを描画し、ベジェ曲線の場合は制御点をともないます。
/// - closeの要素は、曲線の始点または直前のmoveセグメントの位置まで、直線または滑らかな線を描画します。
///
/// レイアウト目的では、曲線のバウンディングボックスは全てのセグメントと点 `{(0pt, 0pt)}` を含むタイトな矩形となります。
///
/// 位置は絶対的に（すなわち `{(0pt, 0pt)}` に対する相対位置として）指定するか、現在のペン/カーソルの位置、つまり直前のセグメントが終わった位置に対する相対位置として指定できます。
///
/// ベジェ曲線の制御点は `{none}` を渡すことでスキップしたり、`{auto}` を渡すことで直前のセグメントから自動的に鏡映させたりできます。
///
/// # 例
/// ```example
/// #curve(
///   fill: blue.lighten(80%),
///   stroke: blue,
///   curve.move((0pt, 50pt)),
///   curve.line((100pt, 50pt)),
///   curve.cubic(none, (90pt, 0pt), (50pt, 0pt)),
///   curve.close(),
/// )
/// ```
#[elem(scope)]
pub struct CurveElem {
    /// 曲線の塗りつぶし方法。
    ///
    /// 塗りつぶしを設定すると、デフォルトのストロークは消えます。塗りつぶしとストロークの両方を持つ曲線を作成するには、両方を設定する必要があります。
    pub fill: Option<Paint>,

    /// 曲線の塗りつぶしに使用される描画ルール。
    ///
    /// ```example
    /// // We use `.with` to get a new
    /// // function that has the common
    /// // arguments pre-applied.
    /// #let star = curve.with(
    ///   fill: red,
    ///   curve.move((25pt, 0pt)),
    ///   curve.line((10pt, 50pt)),
    ///   curve.line((50pt, 20pt)),
    ///   curve.line((0pt, 20pt)),
    ///   curve.line((40pt, 50pt)),
    ///   curve.close(),
    /// )
    ///
    /// #star(fill-rule: "non-zero")
    /// #star(fill-rule: "even-odd")
    /// ```
    #[default]
    pub fill_rule: FillRule,

    /// 曲線を[ストローク]($stroke)する方法。
    ///
    /// `{none}` に設定するとストロークを無効化できます。`{auto}` に設定すると、塗りつぶしが指定されていない場合に限り、`{1pt}` の黒のストロークになります。
    ///
    /// ```example
    /// #let down = curve.line((40pt, 40pt), relative: true)
    /// #let up = curve.line((40pt, -40pt), relative: true)
    ///
    /// #curve(
    ///   stroke: 4pt + gradient.linear(red, blue),
    ///   down, up, down, up, down,
    /// )
    /// ```
    #[fold]
    pub stroke: Smart<Option<Stroke>>,

    /// 曲線の構成要素。move、lineおよびベジェセグメント、closeの形式で指定します。
    #[variadic]
    pub components: Vec<CurveComponent>,
}

#[scope]
impl CurveElem {
    #[elem]
    type CurveMove;

    #[elem]
    type CurveLine;

    #[elem]
    type CurveQuad;

    #[elem]
    type CurveCubic;

    #[elem]
    type CurveClose;
}

/// 曲線の作成に使用される構成要素。
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CurveComponent {
    Move(Packed<CurveMove>),
    Line(Packed<CurveLine>),
    Quad(Packed<CurveQuad>),
    Cubic(Packed<CurveCubic>),
    Close(Packed<CurveClose>),
}

cast! {
    CurveComponent,
    self => match self {
        Self::Move(element) => element.into_value(),
        Self::Line(element) => element.into_value(),
        Self::Quad(element) => element.into_value(),
        Self::Cubic(element) => element.into_value(),
        Self::Close(element) => element.into_value(),
    },
    v: Content => {
        v.try_into()?
    }
}

impl TryFrom<Content> for CurveComponent {
    type Error = HintedString;

    fn try_from(value: Content) -> HintedStrResult<Self> {
        value
            .into_packed::<CurveMove>()
            .map(Self::Move)
            .or_else(|value| value.into_packed::<CurveLine>().map(Self::Line))
            .or_else(|value| value.into_packed::<CurveQuad>().map(Self::Quad))
            .or_else(|value| value.into_packed::<CurveCubic>().map(Self::Cubic))
            .or_else(|value| value.into_packed::<CurveClose>().map(Self::Close))
            .or_else(|_| bail!("expecting a curve element"))
    }
}

/// 新しい曲線の構成要素を開始します。
///
/// `curve.move` 要素が渡されない場合、曲線は `{(0pt, 0pt)}` から始まります。
///
/// ```example
/// #curve(
///   fill: blue.lighten(80%),
///   fill-rule: "even-odd",
///   stroke: blue,
///   curve.line((50pt, 0pt)),
///   curve.line((50pt, 50pt)),
///   curve.line((0pt, 50pt)),
///   curve.close(),
///   curve.move((10pt, 10pt)),
///   curve.line((40pt, 10pt)),
///   curve.line((40pt, 40pt)),
///   curve.line((10pt, 40pt)),
///   curve.close(),
/// )
/// ```
#[elem(name = "move", title = "Curve Move")]
pub struct CurveMove {
    /// 新しい構成要素の始点。
    #[required]
    pub start: Axes<Rel<Length>>,

    /// 座標を直前の点に対する相対位置として扱うかどうか。
    #[default(false)]
    pub relative: bool,
}

/// 現在の点から後続の点までの直線を追加します。
///
/// ```example
/// #curve(
///   stroke: blue,
///   curve.line((50pt, 0pt)),
///   curve.line((50pt, 50pt)),
///   curve.line((100pt, 50pt)),
///   curve.line((100pt, 0pt)),
///   curve.line((150pt, 0pt)),
/// )
/// ```
#[elem(name = "line", title = "Curve Line")]
pub struct CurveLine {
    /// 直線の終点。
    #[required]
    pub end: Axes<Rel<Length>>,

    /// 座標を直前の点に対する相対位置として扱うかどうか。
    ///
    /// ```example
    /// #curve(
    ///   stroke: blue,
    ///   curve.line((50pt, 0pt), relative: true),
    ///   curve.line((0pt, 50pt), relative: true),
    ///   curve.line((50pt, 0pt), relative: true),
    ///   curve.line((0pt, -50pt), relative: true),
    ///   curve.line((50pt, 0pt), relative: true),
    /// )
    /// ```
    #[default(false)]
    pub relative: bool,
}

/// 直前の点から `end` までの2次ベジェ曲線セグメントを、`control` を制御点として追加します。
///
/// ```example
/// // Function to illustrate where the control point is.
/// #let mark((x, y)) = place(
///   dx: x - 1pt, dy: y - 1pt,
///   circle(fill: aqua, radius: 2pt),
/// )
///
/// #mark((20pt, 20pt))
///
/// #curve(
///   stroke: blue,
///   curve.move((0pt, 100pt)),
///   curve.quad((20pt, 20pt), (100pt, 0pt)),
/// )
/// ```
#[elem(name = "quad", title = "Curve Quadratic Segment")]
pub struct CurveQuad {
    /// 2次ベジェ曲線の制御点。
    ///
    /// - `{auto}` の場合で、このセグメントが別の2次ベジェ曲線に続くときは、直前の制御点が鏡映されます。
    /// - `{none}` の場合、制御点はデフォルトで `end` となり、曲線は直線になります。
    ///
    /// ```example
    /// #curve(
    ///   stroke: 2pt,
    ///   curve.quad((20pt, 40pt), (40pt, 40pt), relative: true),
    ///   curve.quad(auto, (40pt, -40pt), relative: true),
    /// )
    /// ```
    #[required]
    pub control: Smart<Option<Axes<Rel<Length>>>>,

    /// セグメントの終点。
    #[required]
    pub end: Axes<Rel<Length>>,

    /// `control` と `end` の座標を直前の点に対する相対位置として扱うかどうか。
    #[default(false)]
    pub relative: bool,
}

/// 直前の点から `end` までの3次ベジェ曲線セグメントを、`control-start` と `control-end` を制御点として追加します。
///
/// ```example
/// // Function to illustrate where the control points are.
/// #let handle(start, end) = place(
///   line(stroke: red, start: start, end: end)
/// )
///
/// #handle((0pt, 80pt), (10pt, 20pt))
/// #handle((90pt, 60pt), (100pt, 0pt))
///
/// #curve(
///   stroke: blue,
///   curve.move((0pt, 80pt)),
///   curve.cubic((10pt, 20pt), (90pt, 60pt), (100pt, 0pt)),
/// )
/// ```
#[elem(name = "cubic", title = "Curve Cubic Segment")]
pub struct CurveCubic {
    /// 曲線セグメントの始点から出ていく制御点。
    ///
    /// - `{auto}` の場合で、この要素が別の `curve.cubic` 要素に続くときは、直前の制御点が鏡映されます。SVGの用語で言えば、これは `curve.cubic` を `C` 演算子の代わりに `S` 演算子のように振る舞わせます。
    ///
    /// - `{none}` の場合、曲線には最初の制御点がないか、同等に、制御点はデフォルトで曲線の始点となります。
    ///
    /// ```example
    /// #curve(
    ///   stroke: blue,
    ///   curve.move((0pt, 50pt)),
    ///   // - No start control point
    ///   // - End control point at `(20pt, 0pt)`
    ///   // - End point at `(50pt, 0pt)`
    ///   curve.cubic(none, (20pt, 0pt), (50pt, 0pt)),
    ///   // - No start control point
    ///   // - No end control point
    ///   // - End point at `(50pt, 0pt)`
    ///   curve.cubic(none, none, (100pt, 50pt)),
    /// )
    ///
    /// #curve(
    ///   stroke: blue,
    ///   curve.move((0pt, 50pt)),
    ///   curve.cubic(none, (20pt, 0pt), (50pt, 0pt)),
    ///   // Passing `auto` instead of `none` means the start control point
    ///   // mirrors the end control point of the previous curve. Mirror of
    ///   // `(20pt, 0pt)` w.r.t `(50pt, 0pt)` is `(80pt, 0pt)`.
    ///   curve.cubic(auto, none, (100pt, 50pt)),
    /// )
    ///
    /// #curve(
    ///   stroke: blue,
    ///   curve.move((0pt, 50pt)),
    ///   curve.cubic(none, (20pt, 0pt), (50pt, 0pt)),
    ///   // `(80pt, 0pt)` is the same as `auto` in this case.
    ///   curve.cubic((80pt, 0pt), none, (100pt, 50pt)),
    /// )
    /// ```
    #[required]
    pub control_start: Option<Smart<Axes<Rel<Length>>>>,

    /// 曲線セグメントの終点に入る制御点。
    ///
    /// `{none}` に設定すると、曲線には終点の制御点がないか、同等に、制御点はデフォルトで曲線の終点となります。
    #[required]
    pub control_end: Option<Axes<Rel<Length>>>,

    /// 曲線セグメントの終点。
    #[required]
    pub end: Axes<Rel<Length>>,

    /// `control-start`、`control-end`、`end` の座標を直前の点に対する相対位置として扱うかどうか。
    #[default(false)]
    pub relative: bool,
}

/// 直前の点から曲線の始点（または直前の `curve.move` の点）までのセグメントを追加して、曲線を閉じます。
///
/// ```example
/// // We define a function to show the same shape with
/// // both closing modes.
/// #let shape(mode: "smooth") = curve(
///   fill: blue.lighten(80%),
///   stroke: blue,
///   curve.move((0pt, 50pt)),
///   curve.line((100pt, 50pt)),
///   curve.cubic(auto, (90pt, 0pt), (50pt, 0pt)),
///   curve.close(mode: mode),
/// )
///
/// #shape(mode: "smooth")
/// #shape(mode: "straight")
/// ```
#[elem(name = "close", title = "Curve Close")]
pub struct CurveClose {
    /// 曲線を閉じる方法。
    pub mode: CloseMode,
}

/// 曲線を閉じる方法。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum CloseMode {
    /// 始点の反対側にある制御点を考慮した滑らかなセグメントで曲線を閉じます。
    #[default]
    Smooth,
    /// 直線で曲線を閉じます。
    Straight,
}

/// 移動、直線、ベジェセグメントから構成される曲線。
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Curve(pub Vec<CurveItem>);

/// 曲線のアイテム。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CurveItem {
    Move(Point),
    Line(Point),
    Cubic(Point, Point, Point),
    Close,
}

impl Curve {
    /// 空の曲線を作成します。
    pub const fn new() -> Self {
        Self(vec![])
    }

    /// 矩形を表す曲線を作成します。
    pub fn rect(size: Size) -> Self {
        let z = Abs::zero();
        let point = Point::new;
        let mut curve = Self::new();
        curve.move_(point(z, z));
        curve.line(point(size.x, z));
        curve.line(point(size.x, size.y));
        curve.line(point(z, size.y));
        curve.close();
        curve
    }

    /// 軸に沿った楕円を表す曲線を作成します。
    pub fn ellipse(size: Size) -> Self {
        // https://stackoverflow.com/a/2007782
        let z = Abs::zero();
        let rx = size.x / 2.0;
        let ry = size.y / 2.0;
        let m = 0.551784;
        let mx = m * rx;
        let my = m * ry;
        let point = |x, y| Point::new(x + rx, y + ry);

        let mut curve = Curve::new();
        curve.move_(point(-rx, z));
        curve.cubic(point(-rx, -my), point(-mx, -ry), point(z, -ry));
        curve.cubic(point(mx, -ry), point(rx, -my), point(rx, z));
        curve.cubic(point(rx, my), point(mx, ry), point(z, ry));
        curve.cubic(point(-mx, ry), point(-rx, my), point(-rx, z));
        curve
    }

    /// [`Move`](CurveItem::Move) アイテムを追加します。
    pub fn move_(&mut self, p: Point) {
        self.0.push(CurveItem::Move(p));
    }

    /// [`Line`](CurveItem::Line) アイテムを追加します。
    pub fn line(&mut self, p: Point) {
        self.0.push(CurveItem::Line(p));
    }

    /// [`Cubic`](CurveItem::Cubic) アイテムを追加します。
    pub fn cubic(&mut self, p1: Point, p2: Point, p3: Point) {
        self.0.push(CurveItem::Cubic(p1, p2, p3));
    }

    /// [`Close`](CurveItem::Close) アイテムを追加します。
    pub fn close(&mut self) {
        self.0.push(CurveItem::Close);
    }

    /// 曲線が空かどうかを判定します。
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// この曲線の全ての点を指定したオフセットだけ平行移動します。
    pub fn translate(&mut self, offset: Point) {
        if offset.is_zero() {
            return;
        }
        for item in self.0.iter_mut() {
            match item {
                CurveItem::Move(p) => *p += offset,
                CurveItem::Line(p) => *p += offset,
                CurveItem::Cubic(p1, p2, p3) => {
                    *p1 += offset;
                    *p2 += offset;
                    *p3 += offset;
                }
                CurveItem::Close => (),
            }
        }
    }

    /// この曲線のバウンディングボックスを計算します。
    pub fn bbox(&self) -> Rect {
        let mut min = Point::splat(Abs::inf());
        let mut max = Point::splat(-Abs::inf());

        let mut cursor = Point::zero();
        for item in self.0.iter() {
            match item {
                CurveItem::Move(to) => {
                    cursor = *to;
                }
                CurveItem::Line(to) => {
                    min = min.min(cursor).min(*to);
                    max = max.max(cursor).max(*to);
                    cursor = *to;
                }
                CurveItem::Cubic(c0, c1, end) => {
                    let cubic = kurbo::CubicBez::new(
                        kurbo::Point::new(cursor.x.to_pt(), cursor.y.to_pt()),
                        kurbo::Point::new(c0.x.to_pt(), c0.y.to_pt()),
                        kurbo::Point::new(c1.x.to_pt(), c1.y.to_pt()),
                        kurbo::Point::new(end.x.to_pt(), end.y.to_pt()),
                    );

                    let bbox = cubic.bounding_box();
                    min.x = min.x.min(Abs::pt(bbox.x0)).min(Abs::pt(bbox.x1));
                    min.y = min.y.min(Abs::pt(bbox.y0)).min(Abs::pt(bbox.y1));
                    max.x = max.x.max(Abs::pt(bbox.x0)).max(Abs::pt(bbox.x1));
                    max.y = max.y.max(Abs::pt(bbox.y0)).max(Abs::pt(bbox.y1));
                    cursor = *end;
                }
                CurveItem::Close => (),
            }
        }

        Rect::new(min, max)
    }

    /// この曲線のバウンディングボックスのサイズを計算します。
    pub fn bbox_size(&self) -> Size {
        self.bbox().size()
    }
}

impl Curve {
    fn to_kurbo(&self) -> impl Iterator<Item = kurbo::PathEl> + '_ {
        use kurbo::PathEl;

        self.0.iter().map(|item| match *item {
            CurveItem::Move(point) => PathEl::MoveTo(point_to_kurbo(point)),
            CurveItem::Line(point) => PathEl::LineTo(point_to_kurbo(point)),
            CurveItem::Cubic(point, point1, point2) => PathEl::CurveTo(
                point_to_kurbo(point),
                point_to_kurbo(point1),
                point_to_kurbo(point2),
            ),
            CurveItem::Close => PathEl::ClosePath,
        })
    }

    /// この曲線をクリップマスクとして解釈した場合、`point` を含むかどうか。
    pub fn contains(&self, fill_rule: FillRule, needle: Point) -> bool {
        let kurbo = kurbo::BezPath::from_vec(self.to_kurbo().collect());
        let windings = kurbo::Shape::winding(&kurbo, point_to_kurbo(needle));
        match fill_rule {
            FillRule::NonZero => windings != 0,
            FillRule::EvenOdd => windings % 2 != 0,
        }
    }

    /// この曲線を `stroke` でストロークした場合、そのストロークが `point` を含むかどうか。
    pub fn stroke_contains(&self, stroke: &FixedStroke, needle: Point) -> bool {
        let width = stroke.thickness.to_raw();
        let cap = match stroke.cap {
            super::LineCap::Butt => kurbo::Cap::Butt,
            super::LineCap::Round => kurbo::Cap::Round,
            super::LineCap::Square => kurbo::Cap::Square,
        };
        let join = match stroke.join {
            super::LineJoin::Miter => kurbo::Join::Miter,
            super::LineJoin::Round => kurbo::Join::Round,
            super::LineJoin::Bevel => kurbo::Join::Bevel,
        };
        let miter_limit = stroke.miter_limit.get();
        let mut style = kurbo::Stroke::new(width)
            .with_caps(cap)
            .with_join(join)
            .with_miter_limit(miter_limit);
        if let Some(dash) = &stroke.dash {
            style = style.with_dashes(
                dash.phase.to_raw(),
                dash.array.iter().copied().map(Abs::to_raw),
            );
        }
        let opts = kurbo::StrokeOpts::default();
        let tolerance = 0.01;
        let expanded = kurbo::stroke(self.to_kurbo(), &style, &opts, tolerance);
        kurbo::Shape::contains(&expanded, point_to_kurbo(needle))
    }
}

fn point_to_kurbo(point: Point) -> kurbo::Point {
    kurbo::Point::new(point.x.to_raw(), point.y.to_raw())
}
