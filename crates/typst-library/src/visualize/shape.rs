use crate::foundations::{Cast, Content, Smart, elem};
use crate::layout::{Abs, Corners, Length, Point, Rect, Rel, Sides, Size, Sizing};
use crate::visualize::{Curve, FixedStroke, Paint, Stroke};

/// A rectangle with optional content.
///
/// # Example
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
    /// The rectangle's width, relative to its parent container.
    pub width: Smart<Rel<Length>>,

    /// The rectangle's height, relative to its parent container.
    pub height: Sizing,

    /// How to fill the rectangle.
    ///
    /// When setting a fill, the default stroke disappears. To create a
    /// rectangle with both fill and stroke, you have to configure both.
    ///
    /// ```example
    /// #rect(fill: blue)
    /// ```
    pub fill: Option<Paint>,

    /// How to stroke the rectangle. This can be:
    ///
    /// - `{none}` to disable stroking
    ///
    /// - `{auto}` for a stroke of `{1pt + black}` if and only if no fill is
    ///   given.
    ///
    /// - Any kind of [stroke]
    ///
    /// - A dictionary describing the stroke for each side individually. The
    ///   dictionary can contain the following keys in order of precedence:
    ///
    ///   - `top`: The top stroke.
    ///   - `right`: The right stroke.
    ///   - `bottom`: The bottom stroke.
    ///   - `left`: The left stroke.
    ///   - `x`: The horizontal stroke.
    ///   - `y`: The vertical stroke.
    ///   - `rest`: The stroke on all sides except those for which the
    ///     dictionary explicitly sets a size.
    ///
    ///   All keys are optional; omitted keys will use their previously set
    ///   value, or the default stroke if never set.
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

    /// How much to round the rectangle's corners, relative to the minimum of
    /// the width and height divided by two. This can be:
    ///
    /// - A relative length for a uniform corner radius.
    ///
    /// - A dictionary: With a dictionary, the stroke for each side can be set
    ///   individually. The dictionary can contain the following keys in order
    ///   of precedence:
    ///   - `top-left`: The top-left corner radius.
    ///   - `top-right`: The top-right corner radius.
    ///   - `bottom-right`: The bottom-right corner radius.
    ///   - `bottom-left`: The bottom-left corner radius.
    ///   - `left`: The top-left and bottom-left corner radii.
    ///   - `top`: The top-left and top-right corner radii.
    ///   - `right`: The top-right and bottom-right corner radii.
    ///   - `bottom`: The bottom-left and bottom-right corner radii.
    ///   - `rest`: The radii for all corners except those for which the
    ///     dictionary explicitly sets a size.
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

    /// How much to pad the rectangle's content.
    /// See the [box's documentation]($box.inset) for more details.
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// How much to expand the rectangle's size without affecting the layout.
    /// See the [box's documentation]($box.outset) for more details.
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// The content to place into the rectangle.
    ///
    /// When this is omitted, the rectangle takes on a default size of at most
    /// `{45pt}` by `{30pt}`.
    #[positional]
    pub body: Option<Content>,
}

/// オプションでコンテンツを含められる正方形。
///
/// # 例
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
    /// 正方形の一辺の長さ。`width`および`height`とは排他です。
    #[external]
    pub size: Smart<Length>,

    /// 正方形の幅。`size`および`height`とは排他です。
    ///
    /// `size`とは異なり、こちらは親コンテナーの幅に対する相対指定が可能です。
    #[parse(
        let size = args.named::<Smart<Length>>("size")?.map(|s| s.map(Rel::from));
        match size {
            None => args.named("width")?,
            size => size,
        }
    )]
    pub width: Smart<Rel<Length>>,

    /// 正方形の高さ。`size`および`width`とは排他です。
    ///
    /// `size`とは異なり、こちらは親コンテナーの高さに対する相対指定が可能です。
    #[parse(match size {
        None => args.named("height")?,
        size => size.map(Into::into),
    })]
    pub height: Sizing,

    /// 正方形の塗りつぶし方。
    /// 詳細は[rectangleのドキュメント]($rect.fill)を参照してください。
    pub fill: Option<Paint>,

    /// 正方形のストロークの設定方法。
    /// 詳細は[rectangleのドキュメント]($rect.stroke)を参照してください。
    #[fold]
    pub stroke: Smart<Sides<Option<Option<Stroke>>>>,

    /// 正方形の角をどの程度丸めるか。
    /// 詳細は[rectangleのドキュメント]($rect.radius)を参照してください。
    #[fold]
    pub radius: Corners<Option<Rel<Length>>>,

    /// 正方形のコンテンツのパディング量。
    /// 詳細は[boxのドキュメント]($box.inset)を参照してください。
    #[fold]
    #[default(Sides::splat(Some(Abs::pt(5.0).into())))]
    pub inset: Sides<Option<Rel<Length>>>,

    /// レイアウトに影響を与えずに正方形の大きさを拡大する量。
    /// 詳細は[boxのドキュメント]($box.outset)を参照してください。
    #[fold]
    pub outset: Sides<Option<Rel<Length>>>,

    /// 正方形に配置するコンテンツ。
    /// 正方形はこのコンテンツにあわせて、1:1のアスペクト比を保ったまま拡大します。
    ///
    /// 省略された場合、正方形は最大で`{30pt}`のデフォルトサイズになります。
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
