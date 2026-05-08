use crate::foundations::{Cast, Content, Smart, elem};
use crate::layout::{Abs, Corners, Length, Point, Rect, Rel, Sides, Size, Sizing};
use crate::visualize::{Curve, FixedStroke, Paint, Stroke};

/// オプションでコンテンツを含められる長方形。
///
/// # 例
/// ```example
/// // Without content.
/// #rect(width: 35%, height: 30pt)
///
/// // With content.
/// #rect[
///   Automatically sized \
///   to fit the content.
/// ]
/// ```
#[elem(title = "Rectangle")]
pub struct RectElem {
    /// 長方形の幅。親コンテナーに対する相対指定です。
    pub width: Smart<Rel<Length>>,

    /// 長方形の高さ。親コンテナーに対する相対指定です。
    pub height: Sizing,

    /// 長方形の塗りつぶし方。
    ///
    /// 塗りつぶしを設定すると、デフォルトのストロークは消えます。
    /// 塗りつぶしとストロークの両方を持つ長方形を作成するには、両方を設定する必要があります。
    ///
    /// ```example
    /// #rect(fill: blue)
    /// ```
    pub fill: Option<Paint>,

    /// 長方形のストロークの設定方法。以下のいずれかを指定できます。
    ///
    /// - `{none}`：ストロークを無効化します。
    ///
    /// - `{auto}`：塗りつぶしが指定されていない場合に限り、`{1pt + black}`のストロークになります。
    ///
    /// - 任意の[ストローク]($stroke)
    ///
    /// - 各辺ごとのストロークを記述した辞書。
    ///   この辞書には、優先順位の高い順に以下のキーを含めることができます。
    ///
    ///   - `top`：上辺のストローク。
    ///   - `right`：右辺のストローク。
    ///   - `bottom`：下辺のストローク。
    ///   - `left`：左辺のストローク。
    ///   - `x`：水平方向のストローク。
    ///   - `y`：垂直方向のストローク。
    ///   - `rest`：辞書でサイズが明示的に設定されていない全ての辺のストローク。
    ///
    ///   全てのキーは任意で、省略されたキーには以前に設定された値、
    ///   一度も設定されていない場合はデフォルトのストロークが使用されます。
    ///
    /// ```example
    /// #stack(
    ///   dir: ltr,
    ///   spacing: 1fr,
    ///   rect(stroke: red),
    ///   rect(stroke: 2pt),
    ///   rect(stroke: 2pt + red),
    /// )
    /// ```
    #[fold]
    pub stroke: Smart<Sides<Option<Option<Stroke>>>>,

    /// 長方形の角をどの程度丸めるか。幅と高さの最小値の半分に対する相対値で指定します。
    /// 以下のいずれかを指定できます。
    ///
    /// - 相対長：全ての角に対して一律の角丸半径を指定します。
    ///
    /// - 辞書：辞書を用いると、各辺の角丸を個別に設定できます。
    ///   この辞書には、優先順位の高い順に以下のキーを含めることができます。
    ///   - `top-left`：左上の角丸半径。
    ///   - `top-right`：右上の角丸半径。
    ///   - `bottom-right`：右下の角丸半径。
    ///   - `bottom-left`：左下の角丸半径。
    ///   - `left`：左上と左下の角丸半径。
    ///   - `top`：左上と右上の角丸半径。
    ///   - `right`：右上と右下の角丸半径。
    ///   - `bottom`：左下と右下の角丸半径。
    ///   - `rest`：辞書でサイズが明示的に設定されていない全ての角の角丸半径。
    ///
    /// ```example
    /// #set rect(stroke: 4pt)
    /// #rect(
    ///   radius: (
    ///     left: 5pt,
    ///     top-right: 20pt,
    ///     bottom-right: 10pt,
    ///   ),
    ///   stroke: (
    ///     left: red,
    ///     top: yellow,
    ///     right: green,
    ///     bottom: blue,
    ///   ),
    /// )
    /// ```
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// 長方形のコンテンツのパディング量。
    /// 詳細は[boxのドキュメント]($box.inset)を参照してください。
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// レイアウトに影響を与えずに長方形の大きさを拡大する量。
    /// 詳細は[boxのドキュメント]($box.outset)を参照してください。
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// 長方形に配置するコンテンツ。
    ///
    /// 省略された場合、長方形は最大で`{45pt}` × `{30pt}`のデフォルトサイズになります。
    #[positional]
    pub body: Option<Content>,
}

/// A square with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #square(size: 40pt)
///
/// // With content.
/// #square[
///   Automatically \
///   sized to fit.
/// ]
/// ```
#[elem]
pub struct SquareElem {
    /// The square's side length. This is mutually exclusive with `width` and
    /// `height`.
    #[external]
    pub size: Smart<Length>,

    /// The square's width. This is mutually exclusive with `size` and `height`.
    ///
    /// In contrast to `size`, this can be relative to the parent container's
    /// width.
    #[parse(
        let size = args.named::<Smart<Length>>("size")?.map(|s| s.map(Rel::from));
        match size {
            None => args.named("width")?,
            size => size,
        }
    )]
    pub width: Smart<Rel<Length>>,

    /// The square's height. This is mutually exclusive with `size` and `width`.
    ///
    /// In contrast to `size`, this can be relative to the parent container's
    /// height.
    #[parse(match size {
        None => args.named("height")?,
        size => size.map(Into::into),
    })]
    pub height: Sizing,

    /// How to fill the square. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<Paint>,

    /// How to stroke the square. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[fold]
    pub stroke: Smart<Sides<Option<Option<Stroke>>>>,

    /// How much to round the square's corners. See the
    /// [rectangle's documentation]($rect.radius) for more details.
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// How much to pad the square's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// How much to expand the square's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// The content to place into the square. The square expands to fit this
    /// content, keeping the 1-1 aspect ratio.
    ///
    /// When this is omitted, the square takes on a default size of at most
    /// `{30pt}`.
    #[positional]
    pub body: Option<Content>,
}

/// An ellipse with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #ellipse(width: 35%, height: 30pt)
///
/// // With content.
/// #ellipse[
///   #set align(center)
///   Automatically sized \
///   to fit the content.
/// ]
/// ```
#[elem]
pub struct EllipseElem {
    /// The ellipse's width, relative to its parent container.
    pub width: Smart<Rel<Length>>,

    /// The ellipse's height, relative to its parent container.
    pub height: Sizing,

    /// How to fill the ellipse. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<Paint>,

    /// How to stroke the ellipse. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[fold]
    pub stroke: Smart<Option<Stroke>>,

    /// How much to pad the ellipse's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// How much to expand the ellipse's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// The content to place into the ellipse.
    ///
    /// When this is omitted, the ellipse takes on a default size of at most
    /// `{45pt}` by `{30pt}`.
    #[positional]
    pub body: Option<Content>,
}

/// A circle with optional content.
///
/// # Example
/// ```example
/// // Without content.
/// #circle(radius: 25pt)
///
/// // With content.
/// #circle[
///   #set align(center + horizon)
///   Automatically \
///   sized to fit.
/// ]
/// ```
#[elem]
pub struct CircleElem {
    /// The circle's radius. This is mutually exclusive with `width` and
    /// `height`.
    #[external]
    pub radius: Length,

    /// The circle's width. This is mutually exclusive with `radius` and
    /// `height`.
    ///
    /// In contrast to `radius`, this can be relative to the parent container's
    /// width.
    #[parse(
        let size = args
            .named::<Smart<Length>>("radius")?
            .map(|s| s.map(|r| 2.0 * Rel::from(r)));
        match size {
            None => args.named("width")?,
            size => size,
        }
    )]
    pub width: Smart<Rel<Length>>,

    /// The circle's height. This is mutually exclusive with `radius` and
    /// `width`.
    ///
    /// In contrast to `radius`, this can be relative to the parent container's
    /// height.
    #[parse(match size {
        None => args.named("height")?,
        size => size.map(Into::into),
    })]
    pub height: Sizing,

    /// How to fill the circle. See the [rectangle's documentation]($rect.fill)
    /// for more details.
    pub fill: Option<Paint>,

    /// How to stroke the circle. See the
    /// [rectangle's documentation]($rect.stroke) for more details.
    #[fold]
    #[default(Smart::Auto)]
    pub stroke: Smart<Option<Stroke>>,

    /// How much to pad the circle's content. See the
    /// [box's documentation]($box.inset) for more details.
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// How much to expand the circle's size without affecting the layout. See
    /// the [box's documentation]($box.outset) for more details.
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// The content to place into the circle. The circle expands to fit this
    /// content, keeping the 1-1 aspect ratio.
    #[positional]
    pub body: Option<Content>,
}

/// A geometric shape with optional fill and stroke.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Shape {
    /// The shape's geometry.
    pub geometry: Geometry,
    /// The shape's background fill.
    pub fill: Option<Paint>,
    /// The shape's fill rule.
    pub fill_rule: FillRule,
    /// The shape's border stroke.
    pub stroke: Option<FixedStroke>,
}

/// A fill rule for curve drawing.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum FillRule {
    /// Specifies that "inside" is computed by a non-zero sum of signed edge crossings.
    #[default]
    NonZero,
    /// Specifies that "inside" is computed by an odd number of edge crossings.
    EvenOdd,
}

/// A shape's geometry.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Geometry {
    /// A line to a point (relative to its position).
    Line(Point),
    /// A rectangle with its origin in the topleft corner.
    Rect(Size),
    /// A curve consisting of movements, lines, and Bézier segments.
    Curve(Curve),
}

impl Geometry {
    /// Fill the geometry without a stroke.
    pub fn filled(self, fill: impl Into<Paint>) -> Shape {
        Shape {
            geometry: self,
            fill: Some(fill.into()),
            fill_rule: FillRule::default(),
            stroke: None,
        }
    }

    /// Stroke the geometry without a fill.
    pub fn stroked(self, stroke: FixedStroke) -> Shape {
        Shape {
            geometry: self,
            fill: None,
            fill_rule: FillRule::default(),
            stroke: Some(stroke),
        }
    }

    /// The bounding box of the geometry.
    pub fn bbox(&self) -> Rect {
        match self {
            Self::Line(end) => {
                let min = end.min(Point::zero());
                let max = end.max(Point::zero());
                Rect::new(min, max)
            }
            Self::Rect(size) => {
                let p = size.to_point();
                let min = p.min(Point::zero());
                let max = p.max(Point::zero());
                Rect::new(min, max)
            }
            Self::Curve(curve) => curve.bbox(),
        }
    }

    /// The bounding box of the geometry.
    pub fn bbox_size(&self) -> Size {
        match self {
            Self::Line(line) => Size::new(line.x, line.y),
            Self::Rect(rect) => *rect,
            Self::Curve(curve) => curve.bbox_size(),
        }
    }
}
