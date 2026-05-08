use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::sync::Arc;

use ecow::EcoString;
use kurbo::Vec2;
use typst_syntax::{Span, Spanned};

use crate::diag::{SourceResult, bail};
use crate::foundations::{
    Args, Array, Cast, Func, IntoValue, Repr, Smart, array, cast, func, scope, ty,
};
use crate::layout::{Angle, Axes, Dir, Quadrant, Ratio};
use crate::visualize::{Color, ColorSpace, WeightedColor};

/// 色のグラデーション。
///
/// Typstは[`gradient.linear`関数]($gradient.linear)による線形グラデーション、[`gradient.radial`関数]($gradient.radial)による放射状グラデーション、[`gradient.conic`関数]($gradient.conic)による円錐状グラデーションをサポートしています。
///
/// グラデーションは以下の用途に使用できます。
/// - 図形の内部を塗りつぶす塗り：
///   `{rect(fill: gradient.linear(..))}`
/// - 図形の輪郭を描画するストローク：
///   `{rect(stroke: 1pt + gradient.linear(..))}`
/// - テキストの塗り：
///   `{set text(fill: gradient.linear(..))}`
/// - [サンプル]($gradient.sample)を取得できるカラーマップ：
///   `{gradient.linear(..).sample(50%)}`
///
/// # 例
/// ```example
/// >>> #set square(size: 50pt)
/// #stack(
///   dir: ltr,
///   spacing: 1fr,
///   square(fill: gradient.linear(..color.map.rainbow)),
///   square(fill: gradient.radial(..color.map.rainbow)),
///   square(fill: gradient.conic(..color.map.rainbow)),
/// )
/// ```
///
/// グラデーションはテキストにも適用できますが、[相対配置]($gradient.relative)が `{auto}`（デフォルト値）または `{"parent"}` に設定されている場合に限ります。単語ごとやグリフごとのグラデーションを作成するには、テキストの単語や文字を手動で、または[showルール]($styling/#show-rules)を介して[ボックス]($box)で囲むことができます。
///
/// ```example
/// >>> #set page(width: auto, height: auto, margin: 12pt)
/// >>> #set text(size: 12pt)
/// #set text(fill: gradient.linear(red, blue))
/// #let rainbow(content) = {
///   set text(fill: gradient.linear(..color.map.rainbow))
///   box(content)
/// }
///
/// This is a gradient on text, but with a #rainbow[twist]!
/// ```
///
/// # ストップ
/// グラデーションは一連のストップから構成されます。各ストップには色とオフセットがあります。オフセットは `{0%}` から `{100%}` までの[比率]($ratio)、または `{0deg}` から `{360deg}` までの角度です。オフセットは、ストップがグラデーション上のどの位置にあるかを決定する相対位置です。ストップの色は、その位置でのグラデーションの色です。グラデーションを定義する際にオフセットを省略することもできます。その場合、Typstは全てのストップを等間隔に配置します。
///
/// Typstはストップとして使用できるカラーマップを事前定義しています。詳細は[`color`]($color/#predefined-color-maps)のドキュメントを参照してください。
///
/// # 相対配置
/// `{0%}` と `{100%}` のストップの位置はコンテナの寸法に依存します。このコンテナは、塗りつぶされる図形そのものか、最も近い周囲のコンテナのいずれかになります。これはグラデーションのコンストラクターの `relative` 引数によって制御されます。デフォルトでは、グラデーションは塗りつぶされる図形に対して相対的です。ただし、グラデーションがテキストに適用されている場合は、最も近い祖先のコンテナに対して相対的になります。
///
/// Typstは祖先コンテナを以下のように決定します。
/// - 文書のルート/最上位レベルに配置された図形の場合、最も近い祖先はページ自体です。
/// - その他の図形の場合、祖先は図形を含む最も内側の[`block`]($block)または[`box`]($box)です。これにはshowルールや要素によって暗黙的に作成されたボックスやブロックも含まれます。例えば、[`rotate`]($rotate)はグラデーションの親に影響しませんが、[`grid`]($grid)は影響します。
///
/// # 色空間と補間
/// グラデーションは任意の色空間で補間できます。デフォルトでは、グラデーションは[Oklab]($color.oklab)色空間で補間されます。これは[知覚的に均一](https://programmingdesignsystems.com/color/perceptually-uniform-color-spaces/index.html)な色空間です。これは、グラデーションが滑らかな色の遷移として知覚されることを意味します。これは特にデータの視覚化に有用です。
///
/// ただし、サポートされている任意の色空間でグラデーションを補間することを選べますが、一部の色空間は色の間を知覚的に補間するのに適していないことに注意してください。補間空間を選ぶ際は、以下の表を参照してください。
///
/// |           色空間                | 知覚的に均一？        |
/// | ------------------------------- |-----------------------|
/// | [Oklab]($color.oklab)           | *はい*                |
/// | [Oklch]($color.oklch)           | *はい*                |
/// | [sRGB]($color.rgb)              | *いいえ*              |
/// | [linear-RGB]($color.linear-rgb) | *はい*                |
/// | [CMYK]($color.cmyk)             | *いいえ*              |
/// | [Grayscale]($color.luma)        | *はい*                |
/// | [HSL]($color.hsl)               | *いいえ*              |
/// | [HSV]($color.hsv)               | *いいえ*              |
///
/// ```preview
/// >>> #set text(fill: white, font: "IBM Plex Sans", 8pt)
/// >>> #set block(spacing: 0pt)
/// #let spaces = (
///   ("Oklab", color.oklab),
///   ("Oklch", color.oklch),
///   ("sRGB", color.rgb),
///   ("linear-RGB", color.linear-rgb),
///   ("CMYK", color.cmyk),
///   ("Grayscale", color.luma),
///   ("HSL", color.hsl),
///   ("HSV", color.hsv),
/// )
///
/// #for (name, space) in spaces {
///   block(
///     width: 100%,
///     inset: 4pt,
///     fill: gradient.linear(
///       red,
///       blue,
///       space: space,
///     ),
///     strong(upper(name)),
///   )
/// }
/// ```
///
/// # 方向
/// 一部のグラデーションは方向に敏感です。例えば、線形グラデーションには方向を決定する角度があります。Typstは時計回りの角度を使用します。0°は左から右、90°は上から下、180°は右から左、270°は下から上です。
///
/// ```example
/// >>> #set square(size: 50pt)
/// #stack(
///   dir: ltr,
///   spacing: 1fr,
///   square(fill: gradient.linear(red, blue, angle: 0deg)),
///   square(fill: gradient.linear(red, blue, angle: 90deg)),
///   square(fill: gradient.linear(red, blue, angle: 180deg)),
///   square(fill: gradient.linear(red, blue, angle: 270deg)),
/// )
/// ```
///
/// # ファイルサイズに関する注意
///
/// グラデーションは、特にストップが多い場合、かなり大きくなることがあります。これはグラデーションが色とオフセットのリストとして保存されるためで、多くの容量を占有することがあります。ファイルサイズが気になる場合は、以下を考慮すべきです。
/// - SVGグラデーションは現在、非効率的にエンコードされています。これは将来改善される予定です。
/// - [`color.oklab`]、[`color.hsv`]、[`color.hsl`]、[`color.oklch`]色空間のPDFグラデーションは、間に追加のストップを挿入した[`color.rgb`]色のリストとして保存されます。これによりPDFファイルでこれらの色空間をエンコードする必要がなくなりますが、グラデーションに追加のストップが加わるため、ファイルサイズが増える可能性があります。
#[ty(scope, cast)]
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Gradient {
    Linear(Arc<LinearGradient>),
    Radial(Arc<RadialGradient>),
    Conic(Arc<ConicGradient>),
}

#[scope]
#[allow(clippy::too_many_arguments)]
impl Gradient {
    /// 色が直線に沿って遷移する新しい線形グラデーションを作成します。
    ///
    /// ```example
    /// #rect(
    ///   width: 100%,
    ///   height: 20pt,
    ///   fill: gradient.linear(
    ///     ..color.map.viridis,
    ///   ),
    /// )
    /// ```
    #[func(title = "Linear Gradient")]
    pub fn linear(
        args: &mut Args,
        span: Span,
        /// グラデーションの色の[ストップ](#stops)。
        #[variadic]
        stops: Vec<Spanned<GradientStop>>,
        /// グラデーションを補間する色空間。
        ///
        /// デフォルトは[Oklab]($color.oklab)と呼ばれる知覚的に均一な色空間です。
        #[named]
        #[default(ColorSpace::Oklab)]
        space: ColorSpace,
        /// グラデーションの[相対配置](#relativeness)。
        ///
        /// 文書のルート/最上位レベルに配置された要素の場合、親はページ自体です。その他の要素の場合、親は要素を含む最も内側のblock、box、column、grid、またはstackです。
        #[named]
        #[default(Smart::Auto)]
        relative: Smart<RelativeTo>,
        /// グラデーションの方向。
        #[external]
        #[default(Dir::LTR)]
        dir: Dir,
        /// グラデーションの角度。
        #[external]
        angle: Angle,
    ) -> SourceResult<Gradient> {
        let angle = if let Some(angle) = args.named::<Angle>("angle")? {
            angle
        } else if let Some(dir) = args.named::<Dir>("dir")? {
            match dir {
                Dir::LTR => Angle::rad(0.0),
                Dir::RTL => Angle::rad(PI),
                Dir::TTB => Angle::rad(FRAC_PI_2),
                Dir::BTT => Angle::rad(3.0 * FRAC_PI_2),
            }
        } else {
            Angle::rad(0.0)
        };

        if stops.len() < 2 {
            bail!(
                span, "a gradient must have at least two stops";
                hint: "try filling the shape with a single color instead"
            );
        }

        Ok(Self::Linear(Arc::new(LinearGradient {
            stops: process_stops(&stops)?,
            angle,
            space,
            relative,
            anti_alias: true,
        })))
    }

    /// 色が原点から放射状に広がる新しい放射状グラデーションを作成します。
    ///
    /// グラデーションは2つの円、すなわち焦点円と終端円によって定義されます。焦点円は中心 `focal-center` と半径 `focal-radius` を持つ円で、グラデーションが開始する点を定義し、最初のストップの色を持ちます。終端円は中心 `center` と半径 `radius` を持つ円で、グラデーションが終了する点を定義し、最後のストップの色を持ちます。グラデーションはこれら2つの円の間で補間されます。
    ///
    /// これら4つの値（始点円の焦点と終端円の中心と半径とも呼ばれる）を使用すると、基本的な放射状グラデーションよりも興味深い特性を持つグラデーションを定義できます。
    ///
    /// ```example
    /// >>> #set circle(radius: 30pt)
    /// #stack(
    ///   dir: ltr,
    ///   spacing: 1fr,
    ///   circle(fill: gradient.radial(
    ///     ..color.map.viridis,
    ///   )),
    ///   circle(fill: gradient.radial(
    ///     ..color.map.viridis,
    ///     focal-center: (10%, 40%),
    ///     focal-radius: 5%,
    ///   )),
    /// )
    /// ```
    #[func(title = "Radial Gradient")]
    fn radial(
        span: Span,
        /// グラデーションの色の[ストップ](#stops)。
        #[variadic]
        stops: Vec<Spanned<GradientStop>>,
        /// グラデーションを補間する色空間。
        ///
        /// デフォルトは[Oklab]($color.oklab)と呼ばれる知覚的に均一な色空間です。
        #[named]
        #[default(ColorSpace::Oklab)]
        space: ColorSpace,
        /// グラデーションの[相対配置](#relativeness)。
        ///
        /// 文書のルート/最上位レベルに配置された要素の場合、親はページ自体です。その他の要素の場合、親は要素を含む最も内側のblock、box、column、grid、またはstackです。
        #[named]
        #[default(Smart::Auto)]
        relative: Smart<RelativeTo>,
        /// グラデーションの終端円の中心。
        ///
        /// 値 `{(50%, 50%)}` は、終端円がコンテナの内側中央に配置されることを意味します。
        #[named]
        #[default(Axes::splat(Ratio::new(0.5)))]
        center: Axes<Ratio>,
        /// グラデーションの終端円の半径。
        ///
        /// デフォルトでは `{50%}` に設定されています。終端半径は焦点半径より大きくなければなりません。
        #[named]
        #[default(Spanned::new(Ratio::new(0.5), Span::detached()))]
        radius: Spanned<Ratio>,
        /// グラデーションの焦点円の中心。
        ///
        /// 焦点中心は終端円の内側になければなりません。
        ///
        /// 値 `{(50%, 50%)}` は、焦点円がコンテナの内側中央に配置されることを意味します。
        ///
        /// デフォルトでは終端円の中心と同じに設定されます。
        #[named]
        #[default(Smart::Auto)]
        focal_center: Smart<Axes<Ratio>>,
        /// グラデーションの焦点円の半径。
        ///
        /// 焦点中心は終端円の内側になければなりません。
        ///
        /// デフォルトでは `{0%}` に設定されています。焦点半径は終端半径より小さくなければなりません。
        #[named]
        #[default(Spanned::new(Ratio::new(0.0), Span::detached()))]
        focal_radius: Spanned<Ratio>,
    ) -> SourceResult<Gradient> {
        if stops.len() < 2 {
            bail!(
                span, "a gradient must have at least two stops";
                hint: "try filling the shape with a single color instead"
            );
        }

        if focal_radius.v > radius.v {
            bail!(
                focal_radius.span,
                "the focal radius must be smaller than the end radius";
                hint: "try using a focal radius of `0%` instead"
            );
        }

        let focal_center = focal_center.unwrap_or(center);
        let d_center_sqr = (focal_center.x - center.x).get().powi(2)
            + (focal_center.y - center.y).get().powi(2);
        if d_center_sqr.sqrt() >= (radius.v - focal_radius.v).get() {
            bail!(
                span,
                "the focal circle must be inside of the end circle";
                hint: "try using a focal center of `auto` instead"
            );
        }

        Ok(Gradient::Radial(Arc::new(RadialGradient {
            stops: process_stops(&stops)?,
            center: center.map(From::from),
            radius: radius.v,
            focal_center,
            focal_radius: focal_radius.v,
            space,
            relative,
            anti_alias: true,
        })))
    }

    /// 色が中心点を中心に放射状に変化する新しい円錐状グラデーションを作成します。
    ///
    /// `center` 引数を使用することで、グラデーションの中心点を制御できます。デフォルトでは、中心点は図形の中心です。
    ///
    /// ```example
    /// >>> #set circle(radius: 30pt)
    /// #stack(
    ///   dir: ltr,
    ///   spacing: 1fr,
    ///   circle(fill: gradient.conic(
    ///     ..color.map.viridis,
    ///   )),
    ///   circle(fill: gradient.conic(
    ///     ..color.map.viridis,
    ///     center: (20%, 30%),
    ///   )),
    /// )
    /// ```
    #[func(title = "Conic Gradient")]
    pub fn conic(
        span: Span,
        /// グラデーションの色の[ストップ](#stops)。
        #[variadic]
        stops: Vec<Spanned<GradientStop>>,
        /// グラデーションの角度。
        #[named]
        #[default(Angle::zero())]
        angle: Angle,
        /// グラデーションを補間する色空間。
        ///
        /// デフォルトは[Oklab]($color.oklab)と呼ばれる知覚的に均一な色空間です。
        #[named]
        #[default(ColorSpace::Oklab)]
        space: ColorSpace,
        /// グラデーションの[相対配置](#relativeness)。
        ///
        /// 文書のルート/最上位レベルに配置された要素の場合、親はページ自体です。その他の要素の場合、親は要素を含む最も内側のblock、box、column、grid、またはstackです。
        #[named]
        #[default(Smart::Auto)]
        relative: Smart<RelativeTo>,
        /// グラデーションの円の中心。
        ///
        /// 値 `{(50%, 50%)}` は、円がコンテナの内側中央に配置されることを意味します。
        #[named]
        #[default(Axes::splat(Ratio::new(0.5)))]
        center: Axes<Ratio>,
    ) -> SourceResult<Gradient> {
        if stops.len() < 2 {
            bail!(
                span, "a gradient must have at least two stops";
                hint: "try filling the shape with a single color instead"
            );
        }

        Ok(Gradient::Conic(Arc::new(ConicGradient {
            stops: process_stops(&stops)?,
            angle,
            center: center.map(From::from),
            space,
            relative,
            anti_alias: true,
        })))
    }

    /// このグラデーションのシャープ版を作成します。
    ///
    /// シャープグラデーションは、滑らかな遷移ではなく色の間に離散的なジャンプがあります。プリセットグラデーション用の色リストを作成する際に特に有用です。
    ///
    /// ```example
    /// #set rect(width: 100%, height: 20pt)
    /// #let grad = gradient.linear(..color.map.rainbow)
    /// #rect(fill: grad)
    /// #rect(fill: grad.sharp(5))
    /// #rect(fill: grad.sharp(5, smoothness: 20%))
    /// ```
    #[func]
    pub fn sharp(
        &self,
        /// グラデーションのストップ数。
        steps: Spanned<usize>,
        /// グラデーションを平滑化する度合い。
        #[named]
        #[default(Spanned::new(Ratio::zero(), Span::detached()))]
        smoothness: Spanned<Ratio>,
    ) -> SourceResult<Gradient> {
        if steps.v < 2 {
            bail!(steps.span, "sharp gradients must have at least two stops");
        }

        if smoothness.v.get() < 0.0 || smoothness.v.get() > 1.0 {
            bail!(smoothness.span, "smoothness must be between 0 and 1");
        }

        let n = steps.v;
        let smoothness = smoothness.v.get();
        let colors = (0..n)
            .flat_map(|i| {
                let c = self
                    .sample(RatioOrAngle::Ratio(Ratio::new(i as f64 / (n - 1) as f64)));

                [c, c]
            })
            .collect::<Vec<_>>();

        let mut positions = Vec::with_capacity(n * 2);
        let index_to_progress = |i| i as f64 * 1.0 / n as f64;

        let progress = smoothness * 1.0 / (4.0 * n as f64);
        for i in 0..n {
            let mut j = 2 * i;
            positions.push(index_to_progress(i));
            if j > 0 {
                positions[j] += progress;
            }

            j += 1;
            positions.push(index_to_progress(i + 1));
            if j < colors.len() - 1 {
                positions[j] -= progress;
            }
        }

        let mut stops = colors
            .into_iter()
            .zip(positions)
            .map(|(c, p)| (c, Ratio::new(p)))
            .collect::<Vec<_>>();

        stops.dedup();

        Ok(match self {
            Self::Linear(linear) => Self::Linear(Arc::new(LinearGradient {
                stops,
                angle: linear.angle,
                space: linear.space,
                relative: linear.relative,
                anti_alias: false,
            })),
            Self::Radial(radial) => Self::Radial(Arc::new(RadialGradient {
                stops,
                center: radial.center,
                radius: radial.radius,
                focal_center: radial.focal_center,
                focal_radius: radial.focal_radius,
                space: radial.space,
                relative: radial.relative,
                anti_alias: false,
            })),
            Self::Conic(conic) => Self::Conic(Arc::new(ConicGradient {
                stops,
                angle: conic.angle,
                center: conic.center,
                space: conic.space,
                relative: conic.relative,
                anti_alias: false,
            })),
        })
    }

    /// このグラデーションを指定された回数繰り返します。オプションで、2回目ごとに鏡映できます。
    ///
    /// ```example
    /// #circle(
    ///   radius: 40pt,
    ///   fill: gradient
    ///     .radial(aqua, white)
    ///     .repeat(4),
    /// )
    /// ```
    #[func]
    pub fn repeat(
        &self,
        /// グラデーションを繰り返す回数。
        repetitions: Spanned<usize>,
        /// 2回目ごとにグラデーションを鏡映するかどうか。すなわち、最初のインスタンス（および全ての奇数番目）は変更されません。
        ///
        /// ```example
        /// #circle(
        ///   radius: 40pt,
        ///   fill: gradient
        ///     .conic(green, black)
        ///     .repeat(2, mirror: true)
        /// )
        /// ```
        #[named]
        #[default(false)]
        mirror: bool,
    ) -> SourceResult<Gradient> {
        if repetitions.v == 0 {
            bail!(repetitions.span, "must repeat at least once");
        }

        let n = repetitions.v;
        let mut stops = std::iter::repeat_n(self.stops_ref(), n)
            .enumerate()
            .flat_map(|(i, stops)| {
                let mut stops = stops
                    .iter()
                    .map(move |&(color, offset)| {
                        let r = offset.get();
                        if i % 2 == 1 && mirror {
                            (color, Ratio::new((i as f64 + 1.0 - r) / n as f64))
                        } else {
                            (color, Ratio::new((i as f64 + r) / n as f64))
                        }
                    })
                    .collect::<Vec<_>>();

                if i % 2 == 1 && mirror {
                    stops.reverse();
                }

                stops
            })
            .collect::<Vec<_>>();

        stops.dedup();

        Ok(match self {
            Self::Linear(linear) => Self::Linear(Arc::new(LinearGradient {
                stops,
                angle: linear.angle,
                space: linear.space,
                relative: linear.relative,
                anti_alias: linear.anti_alias,
            })),
            Self::Radial(radial) => Self::Radial(Arc::new(RadialGradient {
                stops,
                center: radial.center,
                radius: radial.radius,
                focal_center: radial.focal_center,
                focal_radius: radial.focal_radius,
                space: radial.space,
                relative: radial.relative,
                anti_alias: radial.anti_alias,
            })),
            Self::Conic(conic) => Self::Conic(Arc::new(ConicGradient {
                stops,
                angle: conic.angle,
                center: conic.center,
                space: conic.space,
                relative: conic.relative,
                anti_alias: conic.anti_alias,
            })),
        })
    }

    /// このグラデーションの種類を返します。
    #[func]
    pub fn kind(&self) -> Func {
        match self {
            Self::Linear(_) => Self::linear_data().into(),
            Self::Radial(_) => Self::radial_data().into(),
            Self::Conic(_) => Self::conic_data().into(),
        }
    }

    /// このグラデーションのストップを返します。
    #[func]
    pub fn stops(&self) -> Vec<GradientStop> {
        match self {
            Self::Linear(linear) => linear
                .stops
                .iter()
                .map(|(color, offset)| GradientStop {
                    color: *color,
                    offset: Some(*offset),
                })
                .collect(),
            Self::Radial(radial) => radial
                .stops
                .iter()
                .map(|(color, offset)| GradientStop {
                    color: *color,
                    offset: Some(*offset),
                })
                .collect(),
            Self::Conic(conic) => conic
                .stops
                .iter()
                .map(|(color, offset)| GradientStop {
                    color: *color,
                    offset: Some(*offset),
                })
                .collect(),
        }
    }

    /// このグラデーションの混合色空間を返します。
    #[func]
    pub fn space(&self) -> ColorSpace {
        match self {
            Self::Linear(linear) => linear.space,
            Self::Radial(radial) => radial.space,
            Self::Conic(conic) => conic.space,
        }
    }

    /// このグラデーションの相対配置を返します。
    #[func]
    pub fn relative(&self) -> Smart<RelativeTo> {
        match self {
            Self::Linear(linear) => linear.relative,
            Self::Radial(radial) => radial.relative,
            Self::Conic(conic) => conic.relative,
        }
    }

    /// このグラデーションの角度を返します。
    ///
    /// グラデーションが線形でも円錐状でもない場合、`{none}` を返します。
    #[func]
    pub fn angle(&self) -> Option<Angle> {
        match self {
            Self::Linear(linear) => Some(linear.angle),
            Self::Radial(_) => None,
            Self::Conic(conic) => Some(conic.angle),
        }
    }

    /// このグラデーションの中心を返します。
    ///
    /// グラデーションが放射状でも円錐状でもない場合、`{none}` を返します。
    #[func]
    pub fn center(&self) -> Option<Axes<Ratio>> {
        match self {
            Self::Linear(_) => None,
            Self::Radial(radial) => Some(radial.center),
            Self::Conic(conic) => Some(conic.center),
        }
    }

    /// このグラデーションの半径を返します。
    ///
    /// グラデーションが放射状でない場合、`{none}` を返します。
    #[func]
    pub fn radius(&self) -> Option<Ratio> {
        match self {
            Self::Linear(_) => None,
            Self::Radial(radial) => Some(radial.radius),
            Self::Conic(_) => None,
        }
    }

    /// このグラデーションの焦点中心を返します。
    ///
    /// グラデーションが放射状でない場合、`{none}` を返します。
    #[func]
    pub fn focal_center(&self) -> Option<Axes<Ratio>> {
        match self {
            Self::Linear(_) => None,
            Self::Radial(radial) => Some(radial.focal_center),
            Self::Conic(_) => None,
        }
    }

    /// このグラデーションの焦点半径を返します。
    ///
    /// グラデーションが放射状でない場合、`{none}` を返します。
    #[func]
    pub fn focal_radius(&self) -> Option<Ratio> {
        match self {
            Self::Linear(_) => None,
            Self::Radial(radial) => Some(radial.focal_radius),
            Self::Conic(_) => None,
        }
    }

    /// 指定された位置でグラデーションをサンプリングします。
    ///
    /// 位置は、グラデーションに沿った位置（`{0%}` から `{100%}` までの[比率]($ratio)）または[角度]($angle)のいずれかです。この範囲外の値は範囲内に丸められます。
    #[func]
    pub fn sample(
        &self,
        /// グラデーションをサンプリングする位置。
        t: RatioOrAngle,
    ) -> Color {
        let value: f64 = t.to_ratio().get();

        match self {
            Self::Linear(linear) => sample_stops(&linear.stops, linear.space, value),
            Self::Radial(radial) => sample_stops(&radial.stops, radial.space, value),
            Self::Conic(conic) => sample_stops(&conic.stops, conic.space, value),
        }
    }

    /// グラデーションを複数の位置で一度にサンプリングし、結果を配列として返します。
    #[func]
    pub fn samples(
        &self,
        /// グラデーションをサンプリングする位置。
        #[variadic]
        ts: Vec<RatioOrAngle>,
    ) -> Array {
        ts.into_iter().map(|t| self.sample(t).into_value()).collect()
    }
}

impl Gradient {
    /// このグラデーションを複製しますが、異なる相対配置を持ちます。
    pub fn with_relative(mut self, relative: RelativeTo) -> Self {
        match &mut self {
            Self::Linear(linear) => {
                Arc::make_mut(linear).relative = Smart::Custom(relative);
            }
            Self::Radial(radial) => {
                Arc::make_mut(radial).relative = Smart::Custom(relative);
            }
            Self::Conic(conic) => {
                Arc::make_mut(conic).relative = Smart::Custom(relative);
            }
        }

        self
    }
    /// このグラデーションのストップへの参照を返します。
    pub fn stops_ref(&self) -> &[(Color, Ratio)] {
        match self {
            Gradient::Linear(linear) => &linear.stops,
            Gradient::Radial(radial) => &radial.stops,
            Gradient::Conic(conic) => &conic.stops,
        }
    }

    /// 指定されたコンテナ内の指定された位置でグラデーションをサンプリングします。
    /// アスペクト比と角度を直接処理します。
    pub fn sample_at(&self, (x, y): (f32, f32), (width, height): (f32, f32)) -> Color {
        // Normalize the coordinates.
        let (mut x, mut y) = (x / width, y / height);
        let t = match self {
            Self::Linear(linear) => {
                // Aspect ratio correction.
                let angle = Gradient::correct_aspect_ratio(
                    linear.angle,
                    Ratio::new((width / height) as f64),
                )
                .to_rad();
                let (sin, cos) = angle.sin_cos();

                let length = sin.abs() + cos.abs();
                if angle > FRAC_PI_2 && angle < 3.0 * FRAC_PI_2 {
                    x = 1.0 - x;
                }

                if angle > PI {
                    y = 1.0 - y;
                }

                (x as f64 * cos.abs() + y as f64 * sin.abs()) / length
            }
            Self::Radial(radial) => {
                // Source: @Enivex - https://typst.app/project/pYLeS0QyCCe8mf0pdnwoAI
                let cr = radial.radius.get();
                let fr = radial.focal_radius.get();
                let z = Vec2::new(x as f64, y as f64);
                let p = Vec2::new(radial.center.x.get(), radial.center.y.get());
                let q =
                    Vec2::new(radial.focal_center.x.get(), radial.focal_center.y.get());

                if (z - q).hypot() < fr {
                    0.0
                } else if (z - p).hypot() > cr {
                    1.0
                } else {
                    let uz = (z - q).normalize();
                    let az = (q - p).dot(uz);
                    let rho = cr.powi(2) - (q - p).hypot().powi(2);
                    let bz = (az.powi(2) + rho).sqrt() - az;

                    ((z - q).hypot() - fr) / (bz - fr)
                }
            }
            Self::Conic(conic) => {
                let (x, y) =
                    (x as f64 - conic.center.x.get(), y as f64 - conic.center.y.get());
                let angle = Gradient::correct_aspect_ratio(
                    conic.angle,
                    Ratio::new((width / height) as f64),
                );
                ((-y.atan2(x) + PI + angle.to_rad()) % TAU) / TAU
            }
        };

        self.sample(RatioOrAngle::Ratio(Ratio::new(t.clamp(0.0, 1.0))))
    }

    /// このグラデーションをアンチエイリアスする必要があるか。
    pub fn anti_alias(&self) -> bool {
        match self {
            Self::Linear(linear) => linear.anti_alias,
            Self::Radial(radial) => radial.anti_alias,
            Self::Conic(conic) => conic.anti_alias,
        }
    }

    /// このグラデーションの相対配置を返します。`auto` の特別なケースを処理します。
    pub fn unwrap_relative(&self, on_text: bool) -> RelativeTo {
        self.relative().unwrap_or_else(|| {
            if on_text { RelativeTo::Parent } else { RelativeTo::Self_ }
        })
    }

    /// グラデーションのアスペクト比に応じてこの角度を補正します。
    ///
    /// これは特にグラデーション用に使用されます。
    pub fn correct_aspect_ratio(angle: Angle, aspect_ratio: Ratio) -> Angle {
        let rad = (angle.to_rad().rem_euclid(TAU).tan() / aspect_ratio.get()).atan();
        let rad = match angle.quadrant() {
            Quadrant::First => rad,
            Quadrant::Second => rad + PI,
            Quadrant::Third => rad + PI,
            Quadrant::Fourth => rad + TAU,
        };
        Angle::rad(rad.rem_euclid(TAU))
    }
}

impl Debug for Gradient {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Linear(v) => v.fmt(f),
            Self::Radial(v) => v.fmt(f),
            Self::Conic(v) => v.fmt(f),
        }
    }
}

impl Repr for Gradient {
    fn repr(&self) -> EcoString {
        match self {
            Self::Radial(radial) => radial.repr(),
            Self::Linear(linear) => linear.repr(),
            Self::Conic(conic) => conic.repr(),
        }
    }
}

/// 軸に沿って2色間を補間するグラデーション。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LinearGradient {
    /// このグラデーションの色のストップ。
    pub stops: Vec<(Color, Ratio)>,
    /// このグラデーションの方向。
    pub angle: Angle,
    /// グラデーションを補間する色空間。
    pub space: ColorSpace,
    /// グラデーションの相対配置。
    pub relative: Smart<RelativeTo>,
    /// グラデーションをアンチエイリアスするかどうか（シャープグラデーションに使用）。
    pub anti_alias: bool,
}

impl Repr for LinearGradient {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::from("gradient.linear(");

        let angle = self.angle.to_rad().rem_euclid(TAU);
        if angle.abs() < f64::EPSILON {
            // Default value, do nothing
        } else if (angle - FRAC_PI_2).abs() < f64::EPSILON {
            r.push_str("dir: rtl, ");
        } else if (angle - PI).abs() < f64::EPSILON {
            r.push_str("dir: ttb, ");
        } else if (angle - 3.0 * FRAC_PI_2).abs() < f64::EPSILON {
            r.push_str("dir: btt, ");
        } else {
            r.push_str("angle: ");
            r.push_str(&self.angle.repr());
            r.push_str(", ");
        }

        if self.space != ColorSpace::Oklab {
            r.push_str("space: ");
            r.push_str(&self.space.into_value().repr());
            r.push_str(", ");
        }

        if self.relative.is_custom() {
            r.push_str("relative: ");
            r.push_str(&self.relative.into_value().repr());
            r.push_str(", ");
        }

        for (i, (color, offset)) in self.stops.iter().enumerate() {
            r.push('(');
            r.push_str(&color.repr());
            r.push_str(", ");
            r.push_str(&offset.repr());
            r.push(')');
            if i != self.stops.len() - 1 {
                r.push_str(", ");
            }
        }

        r.push(')');
        r
    }
}

/// 円に沿って2色間を補間するグラデーション。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RadialGradient {
    /// このグラデーションの色のストップ。
    pub stops: Vec<(Color, Ratio)>,
    /// このグラデーションの終端円の中心。
    pub center: Axes<Ratio>,
    /// このグラデーションの終端円の半径。
    pub radius: Ratio,
    /// このグラデーションの始点円の中心。
    pub focal_center: Axes<Ratio>,
    /// このグラデーションの始点円の半径。
    pub focal_radius: Ratio,
    /// グラデーションを補間する色空間。
    pub space: ColorSpace,
    /// グラデーションの相対配置。
    pub relative: Smart<RelativeTo>,
    /// グラデーションをアンチエイリアスするかどうか（シャープグラデーションに使用）。
    pub anti_alias: bool,
}

impl Repr for RadialGradient {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::from("gradient.radial(");

        if self.center.x != Ratio::new(0.5) || self.center.y != Ratio::new(0.5) {
            r.push_str("center: (");
            r.push_str(&self.center.x.repr());
            r.push_str(", ");
            r.push_str(&self.center.y.repr());
            r.push_str("), ");
        }

        if self.radius != Ratio::new(0.5) {
            r.push_str("radius: ");
            r.push_str(&self.radius.repr());
            r.push_str(", ");
        }

        if self.focal_center != self.center {
            r.push_str("focal-center: (");
            r.push_str(&self.focal_center.x.repr());
            r.push_str(", ");
            r.push_str(&self.focal_center.y.repr());
            r.push_str("), ");
        }

        if self.focal_radius != Ratio::zero() {
            r.push_str("focal-radius: ");
            r.push_str(&self.focal_radius.repr());
            r.push_str(", ");
        }

        if self.space != ColorSpace::Oklab {
            r.push_str("space: ");
            r.push_str(&self.space.into_value().repr());
            r.push_str(", ");
        }

        if self.relative.is_custom() {
            r.push_str("relative: ");
            r.push_str(&self.relative.into_value().repr());
            r.push_str(", ");
        }

        for (i, (color, offset)) in self.stops.iter().enumerate() {
            r.push('(');
            r.push_str(&color.repr());
            r.push_str(", ");
            r.push_str(&offset.repr());
            r.push(')');
            if i != self.stops.len() - 1 {
                r.push_str(", ");
            }
        }

        r.push(')');
        r
    }
}

/// 中心点を中心に放射状に2色間を補間するグラデーション。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConicGradient {
    /// このグラデーションの色のストップ。
    pub stops: Vec<(Color, Ratio)>,
    /// このグラデーションの方向。
    pub angle: Angle,
    /// このグラデーションの円の中心。
    pub center: Axes<Ratio>,
    /// グラデーションを補間する色空間。
    pub space: ColorSpace,
    /// グラデーションの相対配置。
    pub relative: Smart<RelativeTo>,
    /// グラデーションをアンチエイリアスするかどうか（シャープグラデーションに使用）。
    pub anti_alias: bool,
}

impl Repr for ConicGradient {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::from("gradient.conic(");

        let angle = self.angle.to_rad().rem_euclid(TAU);
        if angle.abs() > f64::EPSILON {
            r.push_str("angle: ");
            r.push_str(&self.angle.repr());
            r.push_str(", ");
        }

        if self.center.x != Ratio::new(0.5) || self.center.y != Ratio::new(0.5) {
            r.push_str("center: (");
            r.push_str(&self.center.x.repr());
            r.push_str(", ");
            r.push_str(&self.center.y.repr());
            r.push_str("), ");
        }

        if self.space != ColorSpace::Oklab {
            r.push_str("space: ");
            r.push_str(&self.space.into_value().repr());
            r.push_str(", ");
        }

        if self.relative.is_custom() {
            r.push_str("relative: ");
            r.push_str(&self.relative.into_value().repr());
            r.push_str(", ");
        }

        for (i, (color, offset)) in self.stops.iter().enumerate() {
            r.push('(');
            r.push_str(&color.repr());
            r.push_str(", ");
            r.push_str(&Angle::deg(offset.get() * 360.0).repr());
            r.push(')');
            if i != self.stops.len() - 1 {
                r.push_str(", ");
            }
        }

        r.push(')');
        r
    }
}

/// グラデーションが何に対して相対的か。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum RelativeTo {
    /// 自身に対して相対的（自身のバウンディングボックス）。
    Self_,
    /// 親に対して相対的（親のバウンディングボックス）。
    Parent,
}

/// 色のストップ。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GradientStop {
    /// このストップの色。
    pub color: Color,
    /// グラデーション上のストップのオフセット。
    pub offset: Option<Ratio>,
}

impl GradientStop {
    /// `color` と `offset` から新しいストップを作成します。
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }
}

cast! {
    GradientStop,
    self => if let Some(offset) = self.offset {
        array![self.color.into_value(), offset].into_value()
    } else {
        self.color.into_value()
    },
    color: Color => Self { color, offset: None },
    array: Array => {
        let mut iter = array.into_iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), None) => Self {
                color: a.cast()?,
                offset: Some(b.cast()?)
            },
            _ => Err("a color stop must contain exactly two entries")?,
        }
    }
}

/// 比率または角度。
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum RatioOrAngle {
    Ratio(Ratio),
    Angle(Angle),
}

impl RatioOrAngle {
    pub fn to_ratio(self) -> Ratio {
        match self {
            Self::Ratio(ratio) => ratio,
            Self::Angle(angle) => Ratio::new(angle.to_rad().rem_euclid(TAU) / TAU),
        }
        .clamp(Ratio::zero(), Ratio::one())
    }
}

cast! {
    RatioOrAngle,
    self => match self {
        Self::Ratio(ratio) => ratio.into_value(),
        Self::Angle(angle) => angle.into_value(),
    },
    ratio: Ratio => Self::Ratio(ratio),
    angle: Angle => Self::Angle(angle),
}

/// ストップを前処理し、それらが有効であることを確認し、必要に応じてオフセットを計算します。
///
/// ストップが無効な場合はエラーを返します。
///
/// これは別の関数に分離されています。なぜなら、全ての異なるグラデーションタイプで使用されるためです。
#[comemo::memoize]
fn process_stops(stops: &[Spanned<GradientStop>]) -> SourceResult<Vec<(Color, Ratio)>> {
    let has_offset = stops.iter().any(|stop| stop.v.offset.is_some());
    if has_offset {
        let mut last_stop = f64::NEG_INFINITY;
        for Spanned { v: stop, span } in stops.iter() {
            let Some(stop) = stop.offset else {
                bail!(
                    *span, "either all stops must have an offset or none of them can";
                    hint: "try adding an offset to all stops"
                );
            };

            if stop.get() < last_stop {
                bail!(*span, "offsets must be in monotonic order");
            }

            last_stop = stop.get();
        }

        let out = stops
            .iter()
            .map(|Spanned { v: GradientStop { color, offset }, span }| {
                if offset.unwrap().get() > 1.0 || offset.unwrap().get() < 0.0 {
                    bail!(*span, "offset must be between 0 and 1");
                }
                Ok((*color, offset.unwrap()))
            })
            .collect::<SourceResult<Vec<_>>>()?;

        if out[0].1 != Ratio::zero() {
            bail!(
                stops[0].span,
                "first stop must have an offset of 0";
                hint: "try setting this stop to `0%`"
            );
        }

        if out[out.len() - 1].1 != Ratio::one() {
            bail!(
                stops[out.len() - 1].span,
                "last stop must have an offset of 100%";
                hint: "try setting this stop to `100%`"
            );
        }

        return Ok(out);
    }

    Ok(stops
        .iter()
        .enumerate()
        .map(|(i, stop)| {
            let offset = i as f64 / (stops.len() - 1) as f64;
            (stop.v.color, Ratio::new(offset))
        })
        .collect())
}

/// 指定された位置でストップをサンプリングします。
fn sample_stops(stops: &[(Color, Ratio)], mixing_space: ColorSpace, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    let mut j = stops.partition_point(|(_, ratio)| ratio.get() < t);

    if j == 0 {
        while stops.get(j + 1).is_some_and(|(_, r)| r.is_zero()) {
            j += 1;
        }
        return stops[j].0;
    }

    let (col_0, pos_0) = stops[j - 1];
    let (col_1, pos_1) = stops[j];
    let t = (t - pos_0.get()) / (pos_1.get() - pos_0.get());

    Color::mix_iter(
        [WeightedColor::new(col_0, 1.0 - t), WeightedColor::new(col_1, t)],
        mixing_space,
    )
    .unwrap()
}
