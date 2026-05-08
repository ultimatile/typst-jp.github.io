use std::hash::Hash;
use std::sync::Arc;

use ecow::{EcoString, eco_format};
use typst_syntax::{Span, Spanned};
use typst_utils::{LazyHash, Numeric};

use crate::World;
use crate::diag::{SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{Content, Smart, StyleChain, func, repr, scope, ty};
use crate::introspection::Locator;
use crate::layout::{Abs, Axes, Frame, Length, Region, Size};
use crate::visualize::RelativeTo;

/// 繰り返しのタイリング塗りつぶし。
///
/// Typstは、塗りつぶされたりストロークされたりする要素の領域全体を覆うように
/// パターンが格子状に繰り返される最も一般的なタイプのタイリングをサポートして
/// います。パターンはタイルサイズと、各セルの内容を定義するbodyによって
/// 定義されます。タイリングのセルの間に水平方向や垂直方向の間隔を追加する
/// こともできます。
///
/// # 例
///
/// ```example
/// #let pat = tiling(size: (30pt, 30pt))[
///   #place(line(start: (0%, 0%), end: (100%, 100%)))
///   #place(line(start: (0%, 100%), end: (100%, 0%)))
/// ]
///
/// #rect(fill: pat, width: 100%, height: 60pt, stroke: 1pt)
/// ```
///
/// タイリングは文章に対してもサポートされていますが、[相対性]($tiling.relative)
/// を `{auto}`（デフォルト値）または `{"parent"}` に設定した場合に限ります。
/// 単語ごとまたはグリフごとのタイリングを作成するには、文章の単語や文字を
/// 手動で、もしくは[showルール]($styling/#show-rules)によって
/// [ボックス]($box)で包むことができます。
///
/// ```example
/// #let pat = tiling(
///   size: (30pt, 30pt),
///   relative: "parent",
///   square(
///     size: 30pt,
///     fill: gradient
///       .conic(..color.map.rainbow),
///   )
/// )
///
/// #set text(fill: pat)
/// #lorem(10)
/// ```
///
/// タイリングの[`spacing`]($tiling.spacing)機能を使うことで、要素同士を
/// より広く、あるいはより近く配置することもできます。spacingがタイリングの
/// サイズよりも小さい場合、タイリングは重なります。大きい場合、タイリングは
/// タイリングの背景と同じ色の隙間ができます。
///
/// ```example
/// #let pat = tiling(
///   size: (30pt, 30pt),
///   spacing: (10pt, 10pt),
///   relative: "parent",
///   square(
///     size: 30pt,
///     fill: gradient
///      .conic(..color.map.rainbow),
///   ),
/// )
///
/// #rect(
///   width: 100%,
///   height: 60pt,
///   fill: pat,
/// )
/// ```
///
/// # 相対性
/// タイリングの開始点の位置はコンテナの寸法に依存します。このコンテナは、
/// 描画対象の図形そのものか、最も近い周囲のコンテナのいずれかです。これは
/// タイリングコンストラクターの `relative` 引数で制御します。デフォルトでは、
/// タイリングは描画対象の図形に対して相対的です。ただし、タイリングが文章に
/// 適用される場合は、最も近い祖先のコンテナに対して相対的になります。
///
/// Typstは祖先のコンテナを以下のように決定します。
/// - 文書のルート/最上位レベルに配置された図形の場合、最も近い祖先はページ
///   自体です。
/// - その他の図形の場合、祖先はその図形を含む最も内側の[`block`]または
///   [`box`]です。これには、showルールや要素によって暗黙的に作成された
///   ボックスやブロックも含まれます。例えば、[`rotate`]はグラデーションの
///   親に影響しませんが、[`grid`]は影響します。
///
/// # 互換性
/// この型は以前 `pattern` と呼ばれていました。エイリアスとして名前は残って
/// いますが、Typst 0.13以降は廃止予定です。
#[ty(scope, cast, keywords = ["pattern"])]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Tiling(Arc<Repr>);

/// Internal representation of [`Tiling`].
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Repr {
    /// The tiling's rendered content.
    frame: LazyHash<Frame>,
    /// The tiling's tile size.
    size: Size,
    /// The tiling's tile spacing.
    spacing: Size,
    /// The tiling's relative transform.
    relative: Smart<RelativeTo>,
}

#[scope]
impl Tiling {
    /// 新しいタイリングを構築します。
    ///
    /// ```example
    /// #let pat = tiling(
    ///   size: (20pt, 20pt),
    ///   relative: "parent",
    ///   place(
    ///     dx: 5pt,
    ///     dy: 5pt,
    ///     rotate(45deg, square(
    ///       size: 5pt,
    ///       fill: black,
    ///     )),
    ///   ),
    /// )
    ///
    /// #rect(width: 100%, height: 60pt, fill: pat)
    /// ```
    #[func(constructor)]
    pub fn construct(
        engine: &mut Engine,
        span: Span,
        /// タイリングの各セルのバウンディングボックス。
        #[named]
        #[default(Spanned::new(Smart::Auto, Span::detached()))]
        size: Spanned<Smart<Axes<Length>>>,
        /// タイリングのセル間の間隔。
        #[named]
        #[default(Spanned::new(Axes::splat(Length::zero()), Span::detached()))]
        spacing: Spanned<Axes<Length>>,
        /// タイリングの[相対配置](#relativeness)。
        ///
        /// 文書のルート/最上位レベルに配置された要素の場合、親はページ自体
        /// です。その他の要素の場合、親はその要素を含む最も内側のblock、box、
        /// column、grid、またはstackです。
        #[named]
        #[default(Smart::Auto)]
        relative: Smart<RelativeTo>,
        /// タイリングの各セルの内容。
        body: Content,
    ) -> SourceResult<Tiling> {
        let size_span = size.span;
        if let Smart::Custom(size) = size.v {
            // Ensure that sizes are absolute.
            if !size.x.em.is_zero() || !size.y.em.is_zero() {
                bail!(size_span, "tile size must be absolute");
            }

            // Ensure that sizes are non-zero and finite.
            if size.x.is_zero()
                || size.y.is_zero()
                || !size.x.is_finite()
                || !size.y.is_finite()
            {
                bail!(size_span, "tile size must be non-zero and non-infinite");
            }
        }

        // Ensure that spacing is absolute.
        if !spacing.v.x.em.is_zero() || !spacing.v.y.em.is_zero() {
            bail!(spacing.span, "tile spacing must be absolute");
        }

        // Ensure that spacing is finite.
        if !spacing.v.x.is_finite() || !spacing.v.y.is_finite() {
            bail!(spacing.span, "tile spacing must be finite");
        }

        // The size of the frame
        let size = size.v.map(|l| l.map(|a| a.abs));
        let region = size.unwrap_or_else(|| Axes::splat(Abs::inf()));

        // Layout the tiling.
        let world = engine.world;
        let library = world.library();
        let locator = Locator::root();
        let styles = StyleChain::new(&library.styles);
        let pod = Region::new(region, Axes::splat(false));
        let mut frame =
            (engine.routines.layout_frame)(engine, &body, locator, styles, pod)?;

        // Set the size of the frame if the size is enforced.
        if let Smart::Custom(size) = size {
            frame.set_size(size);
        }

        // Check that the frame is non-zero.
        if frame.width().is_zero() || frame.height().is_zero() {
            bail!(
                span, "tile size must be non-zero";
                hint: "try setting the size manually"
            );
        }

        Ok(Self(Arc::new(Repr {
            size: frame.size(),
            frame: LazyHash::new(frame),
            spacing: spacing.v.map(|l| l.abs),
            relative,
        })))
    }
}

impl Tiling {
    /// Set the relative placement of the tiling.
    pub fn with_relative(mut self, relative: RelativeTo) -> Self {
        if let Some(this) = Arc::get_mut(&mut self.0) {
            this.relative = Smart::Custom(relative);
        } else {
            self.0 = Arc::new(Repr {
                relative: Smart::Custom(relative),
                ..self.0.as_ref().clone()
            });
        }

        self
    }

    /// Return the frame of the tiling.
    pub fn frame(&self) -> &Frame {
        &self.0.frame
    }

    /// Return the size of the tiling in absolute units.
    pub fn size(&self) -> Size {
        self.0.size
    }

    /// Return the spacing of the tiling in absolute units.
    pub fn spacing(&self) -> Size {
        self.0.spacing
    }

    /// Returns the relative placement of the tiling.
    pub fn relative(&self) -> Smart<RelativeTo> {
        self.0.relative
    }

    /// Returns the relative placement of the tiling.
    pub fn unwrap_relative(&self, on_text: bool) -> RelativeTo {
        self.0.relative.unwrap_or_else(|| {
            if on_text { RelativeTo::Parent } else { RelativeTo::Self_ }
        })
    }
}

impl repr::Repr for Tiling {
    fn repr(&self) -> EcoString {
        let mut out =
            eco_format!("tiling(({}, {})", self.0.size.x.repr(), self.0.size.y.repr());

        if self.0.spacing.is_zero() {
            out.push_str(", spacing: (");
            out.push_str(&self.0.spacing.x.repr());
            out.push_str(", ");
            out.push_str(&self.0.spacing.y.repr());
            out.push(')');
        }

        out.push_str(", ..)");

        out
    }
}
