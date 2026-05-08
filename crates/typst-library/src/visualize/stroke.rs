use ecow::EcoString;
use typst_utils::{Numeric, Scalar};

use crate::diag::{HintedStrResult, SourceResult};
use crate::foundations::{
    Args, Cast, Dict, Fold, FromValue, NoneValue, Repr, Resolve, Smart, StyleChain,
    Value, cast, dict, func, scope, ty,
};
use crate::layout::{Abs, Length};
use crate::visualize::{Color, Gradient, Paint, Tiling};

/// 線を描画する方法を定義します。
///
/// ストロークには、_paint_（単色またはグラデーション）、_thickness（線の太さ）_、線の_cap（端の形状）_、線の_join（接合の形状）_、_miter limit（マイター制限）_、および_dash_パターンがあります。これらの値は全てオプションであり、適切なデフォルト値が設定されています。
///
/// # 例
/// ```example
/// #set line(length: 100%)
/// #stack(
///   spacing: 1em,
///   line(stroke: 2pt + red),
///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
///   line(stroke: 2pt + gradient.linear(..color.map.rainbow)),
/// )
/// ```
///
/// # シンプルなストローク
/// 色、太さ、またはその両方の組み合わせから、シンプルな単色のストロークを作成できます。具体的には、ストロークが期待される場所であればどこでも、以下のいずれかの値を渡せます。
///
/// - ストロークの太さを指定する長さ。色は継承され、デフォルトは黒です。
/// - ストロークに使用する色。太さは継承され、デフォルトは `{1pt}` です。
/// - `{2pt + red}` のように `+` 演算子を使って色と太さを組み合わせたストローク。
///
/// 完全に制御するには、ストロークが期待される任意の関数に[辞書]($dictionary)または `{stroke}` オブジェクトを渡すこともできます。辞書のキーには、以下に示すコンストラクター関数のパラメーターのいずれかを含められます。
///
/// # フィールド
/// ストロークオブジェクトでは、コンストラクター関数に列挙されている任意のフィールドにアクセスできます。例えば、`{(2pt + blue).thickness}` は `{2pt}` となります。一方、`{stroke(red).cap}` は指定されていないため `{auto}` となります。`{auto}` に設定されたフィールドは継承されます。
#[ty(scope, cast)]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Stroke<T: Numeric = Length> {
    /// ストロークのpaint。
    pub paint: Smart<Paint>,
    /// ストロークの太さ。
    pub thickness: Smart<T>,
    /// ストロークの線端の形状。
    pub cap: Smart<LineCap>,
    /// ストロークの線の接合形状。
    pub join: Smart<LineJoin>,
    /// ストロークの破線パターン。
    pub dash: Smart<Option<DashPattern<T>>>,
    /// マイター制限。
    pub miter_limit: Smart<Scalar>,
}

impl Stroke {
    /// paintと太さからストロークを作成します。
    pub fn from_pair(paint: impl Into<Paint>, thickness: Length) -> Self {
        Self {
            paint: Smart::Custom(paint.into()),
            thickness: Smart::Custom(thickness),
            ..Default::default()
        }
    }
}

#[scope]
impl Stroke {
    /// 値をストロークに変換するか、指定されたパラメーターでストロークを構築します。
    ///
    /// ほとんどの場合、値を使用するためにストロークに変換する必要はありません。これは自動的に変換されるためです。ただし、このコンストラクターは値がストロークの全てのフィールドを持つことを保証するのに有用です。
    ///
    /// ```example
    /// #let my-func(x) = {
    ///     x = stroke(x) // Convert to a stroke
    ///     [Stroke has thickness #x.thickness.]
    /// }
    /// #my-func(3pt) \
    /// #my-func(red) \
    /// #my-func(stroke(cap: "round", thickness: 1pt))
    /// ```
    #[func(constructor)]
    pub fn construct(
        args: &mut Args,

        /// ストロークに使用する色またはグラデーション。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{black}` です。
        #[external]
        paint: Smart<Paint>,

        /// ストロークの太さ。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{1pt}` です。
        #[external]
        thickness: Smart<Length>,

        /// ストロークの両端をどのように描画するか。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{"butt"}` です。
        #[external]
        cap: Smart<LineCap>,

        /// 鋭く曲がった部分をどのように描画するか。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{"miter"}` です。
        #[external]
        join: Smart<LineJoin>,

        /// 使用する破線パターン。次のいずれかにできます。
        ///
        /// - 定義済みのパターンの一つ：
        ///   - `{"solid"}` または `{none}`
        ///   - `{"dotted"}`
        ///   - `{"densely-dotted"}`
        ///   - `{"loosely-dotted"}`
        ///   - `{"dashed"}`
        ///   - `{"densely-dashed"}`
        ///   - `{"loosely-dashed"}`
        ///   - `{"dash-dotted"}`
        ///   - `{"densely-dash-dotted"}`
        ///   - `{"loosely-dash-dotted"}`
        /// - 線分と間隔の長さを交互に並べた[配列]($array)。線の太さに等しい長さを表すために、文字列 `{"dot"}` も使用できます。
        /// - キー `array`（上記の配列と同じ）と `phase`（[長さ]($length)型、パターンの描画開始位置を定義）を持つ[辞書]($dictionary)。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{none}` です。
        ///
        /// ```example
        /// #set line(length: 100%, stroke: 2pt)
        /// #stack(
        ///   spacing: 1em,
        ///   line(stroke: (dash: "dashed")),
        ///   line(stroke: (dash: (10pt, 5pt, "dot", 5pt))),
        ///   line(stroke: (dash: (array: (10pt, 5pt, "dot", 5pt), phase: 10pt))),
        /// )
        /// ```
        #[external]
        dash: Smart<Option<DashPattern>>,

        /// 突き出した鋭角の曲がりがマイター結合ではなくベベルで描画される閾値。値が大きいほど、ベベル化される前により鋭い角度を許容します。`join` が `{"miter"}` の場合のみ適用されます。
        ///
        /// 具体的には、マイター制限は角の突出長とストロークの太さとの最大比率です。
        ///
        /// `{auto}` に設定された場合、値は継承され、デフォルトは `{4.0}` です。
        ///
        /// ```example
        /// #let items = (
        ///   curve.move((15pt, 0pt)),
        ///   curve.line((0pt, 30pt)),
        ///   curve.line((30pt, 30pt)),
        ///   curve.line((10pt, 20pt)),
        /// )
        ///
        /// #set curve(stroke: 6pt + blue)
        /// #stack(
        ///   dir: ltr,
        ///   spacing: 1cm,
        ///   curve(stroke: (miter-limit: 1), ..items),
        ///   curve(stroke: (miter-limit: 4), ..items),
        ///   curve(stroke: (miter-limit: 5), ..items),
        /// )
        /// ```
        #[external]
        miter_limit: Smart<f64>,
    ) -> SourceResult<Stroke> {
        if let Some(stroke) = args.eat::<Stroke>()? {
            return Ok(stroke);
        }

        fn take<T: FromValue>(args: &mut Args, arg: &str) -> SourceResult<Smart<T>> {
            Ok(args.named::<Smart<T>>(arg)?.unwrap_or(Smart::Auto))
        }

        let paint = take::<Paint>(args, "paint")?;
        let thickness = take::<Length>(args, "thickness")?;
        let cap = take::<LineCap>(args, "cap")?;
        let join = take::<LineJoin>(args, "join")?;
        let dash = take::<Option<DashPattern>>(args, "dash")?;
        let miter_limit = take::<f64>(args, "miter-limit")?.map(Scalar::new);

        Ok(Self { paint, thickness, cap, join, dash, miter_limit })
    }
}

impl<T: Numeric> Stroke<T> {
    /// 含まれる長さを `f` でマッピングします。
    pub fn map<F, U: Numeric>(self, f: F) -> Stroke<U>
    where
        F: Fn(T) -> U,
    {
        Stroke {
            paint: self.paint,
            thickness: self.thickness.map(&f),
            cap: self.cap,
            join: self.join,
            dash: self.dash.map(|dash| {
                dash.map(|dash| DashPattern {
                    array: dash
                        .array
                        .into_iter()
                        .map(|l| match l {
                            DashLength::Length(v) => DashLength::Length(f(v)),
                            DashLength::LineWidth => DashLength::LineWidth,
                        })
                        .collect(),
                    phase: f(dash.phase),
                })
            }),
            miter_limit: self.miter_limit,
        }
    }
}

impl Stroke<Abs> {
    /// ストロークを展開し、不足しているフィールドを `default` から補完します。
    pub fn unwrap_or(self, default: FixedStroke) -> FixedStroke {
        let thickness = self.thickness.unwrap_or(default.thickness);
        let dash = self
            .dash
            .map(|dash| {
                dash.map(|dash| DashPattern {
                    array: dash.array.into_iter().map(|l| l.finish(thickness)).collect(),
                    phase: dash.phase,
                })
            })
            .unwrap_or(default.dash);

        FixedStroke {
            paint: self.paint.unwrap_or(default.paint),
            thickness,
            cap: self.cap.unwrap_or(default.cap),
            join: self.join.unwrap_or(default.join),
            dash,
            miter_limit: self.miter_limit.unwrap_or(default.miter_limit),
        }
    }

    /// ストロークを展開し、不足しているフィールドをデフォルト値で補完します。
    pub fn unwrap_or_default(self) -> FixedStroke {
        // we want to do this; the Clippy lint is not type-aware
        #[allow(clippy::unwrap_or_default)]
        self.unwrap_or(FixedStroke::default())
    }
}

impl<T: Numeric + Repr> Repr for Stroke<T> {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::new();
        let Self { paint, thickness, cap, join, dash, miter_limit } = &self;
        if cap.is_auto() && join.is_auto() && dash.is_auto() && miter_limit.is_auto() {
            match (&self.paint, &self.thickness) {
                (Smart::Custom(paint), Smart::Custom(thickness)) => {
                    r.push_str(&thickness.repr());
                    r.push_str(" + ");
                    r.push_str(&paint.repr());
                }
                (Smart::Custom(paint), Smart::Auto) => r.push_str(&paint.repr()),
                (Smart::Auto, Smart::Custom(thickness)) => r.push_str(&thickness.repr()),
                (Smart::Auto, Smart::Auto) => r.push_str("1pt + black"),
            }
        } else {
            r.push('(');
            let mut sep = "";
            if let Smart::Custom(paint) = &paint {
                r.push_str(sep);
                r.push_str("paint: ");
                r.push_str(&paint.repr());
                sep = ", ";
            }
            if let Smart::Custom(thickness) = &thickness {
                r.push_str(sep);
                r.push_str("thickness: ");
                r.push_str(&thickness.repr());
                sep = ", ";
            }
            if let Smart::Custom(cap) = &cap {
                r.push_str(sep);
                r.push_str("cap: ");
                r.push_str(&cap.repr());
                sep = ", ";
            }
            if let Smart::Custom(join) = &join {
                r.push_str(sep);
                r.push_str("join: ");
                r.push_str(&join.repr());
                sep = ", ";
            }
            if let Smart::Custom(dash) = &dash {
                r.push_str(sep);
                r.push_str("dash: ");
                if let Some(dash) = dash {
                    r.push_str(&dash.repr());
                } else {
                    r.push_str(&NoneValue.repr());
                }
                sep = ", ";
            }
            if let Smart::Custom(miter_limit) = &miter_limit {
                r.push_str(sep);
                r.push_str("miter-limit: ");
                r.push_str(&miter_limit.get().repr());
            }
            r.push(')');
        }
        r
    }
}

impl<T: Numeric + Fold> Fold for Stroke<T> {
    fn fold(self, outer: Self) -> Self {
        Self {
            paint: self.paint.or(outer.paint),
            thickness: self.thickness.or(outer.thickness),
            cap: self.cap.or(outer.cap),
            join: self.join.or(outer.join),
            dash: self.dash.or(outer.dash),
            miter_limit: self.miter_limit.or(outer.miter_limit),
        }
    }
}

impl Resolve for Stroke {
    type Output = Stroke<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        Stroke {
            paint: self.paint,
            thickness: self.thickness.resolve(styles),
            cap: self.cap,
            join: self.join,
            dash: self.dash.resolve(styles),
            miter_limit: self.miter_limit,
        }
    }
}

cast! {
    type Stroke,
    thickness: Length => Self {
        thickness: Smart::Custom(thickness),
        ..Default::default()
    },
    color: Color => Self {
        paint: Smart::Custom(color.into()),
        ..Default::default()
    },
    gradient: Gradient => Self {
        paint: Smart::Custom(gradient.into()),
        ..Default::default()
    },
    tiling: Tiling => Self {
        paint: Smart::Custom(tiling.into()),
        ..Default::default()
    },
    mut dict: Dict => {
        // Get a value by key, accepting either Auto or something convertible to type T.
        fn take<T: FromValue>(dict: &mut Dict, key: &str) -> HintedStrResult<Smart<T>> {
            Ok(dict.take(key).ok().map(Smart::<T>::from_value)
                .transpose()?.unwrap_or(Smart::Auto))
        }

        let paint = take::<Paint>(&mut dict, "paint")?;
        let thickness = take::<Length>(&mut dict, "thickness")?;
        let cap = take::<LineCap>(&mut dict, "cap")?;
        let join = take::<LineJoin>(&mut dict, "join")?;
        let dash = take::<Option<DashPattern>>(&mut dict, "dash")?;
        let miter_limit = take::<f64>(&mut dict, "miter-limit")?;
        dict.finish(&["paint", "thickness", "cap", "join", "dash", "miter-limit"])?;

        Self {
            paint,
            thickness,
            cap,
            join,
            dash,
            miter_limit: miter_limit.map(Scalar::new),
        }
    },
}

cast! {
    Stroke<Abs>,
    self => self.map(Length::from).into_value(),
}

/// ストロークの線端の形状
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineCap {
    /// ストロークの終点に端を持つ正方形の線端。
    Butt,
    /// ストロークの終点を中心とする円形の線端。
    Round,
    /// ストロークの終点を中心とする正方形の線端。
    Square,
}

impl Repr for LineCap {
    fn repr(&self) -> EcoString {
        match self {
            Self::Butt => "butt".repr(),
            Self::Round => "round".repr(),
            Self::Square => "square".repr(),
        }
    }
}

/// ストロークの線の接合形状
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum LineJoin {
    /// セグメントは鋭い角で接合されます。マイター制限を超える鋭い曲がりは、代わりにベベルで接合されます。
    Miter,
    /// セグメントは円形の角で接合されます。
    Round,
    /// セグメントはベベル（接合されるセグメントの端を結ぶ直線の縁）で接合されます。
    Bevel,
}

impl Repr for LineJoin {
    fn repr(&self) -> EcoString {
        match self {
            Self::Miter => "miter".repr(),
            Self::Round => "round".repr(),
            Self::Bevel => "bevel".repr(),
        }
    }
}

/// 破線パターン。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DashPattern<T: Numeric = Length, DT = DashLength<T>> {
    /// 破線の配列。
    pub array: Vec<DT>,
    /// 破線の位相。
    pub phase: T,
}

impl<T: Numeric + Repr, DT: Repr> Repr for DashPattern<T, DT> {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::from("(array: (");
        for (i, elem) in self.array.iter().enumerate() {
            if i != 0 {
                r.push_str(", ")
            }
            r.push_str(&elem.repr())
        }
        r.push_str("), phase: ");
        r.push_str(&self.phase.repr());
        r.push(')');
        r
    }
}

impl<T: Numeric + Default> From<Vec<DashLength<T>>> for DashPattern<T> {
    fn from(array: Vec<DashLength<T>>) -> Self {
        Self { array, phase: T::default() }
    }
}

impl Resolve for DashPattern {
    type Output = DashPattern<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        DashPattern {
            array: self.array.into_iter().map(|l| l.resolve(styles)).collect(),
            phase: self.phase.resolve(styles),
        }
    }
}

// Same names as tikz:
// https://tex.stackexchange.com/questions/45275/tikz-get-values-for-predefined-dash-patterns
cast! {
    DashPattern,
    self => dict! { "array" => self.array, "phase" => self.phase }.into_value(),

    "solid" => Vec::new().into(),
    "dotted" => vec![DashLength::LineWidth, Abs::pt(2.0).into()].into(),
    "densely-dotted" => vec![DashLength::LineWidth, Abs::pt(1.0).into()].into(),
    "loosely-dotted" => vec![DashLength::LineWidth, Abs::pt(4.0).into()].into(),
    "dashed" => vec![Abs::pt(3.0).into(), Abs::pt(3.0).into()].into(),
    "densely-dashed" => vec![Abs::pt(3.0).into(), Abs::pt(2.0).into()].into(),
    "loosely-dashed" => vec![Abs::pt(3.0).into(), Abs::pt(6.0).into()].into(),
    "dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(2.0).into(), DashLength::LineWidth, Abs::pt(2.0).into()].into(),
    "densely-dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(1.0).into(), DashLength::LineWidth, Abs::pt(1.0).into()].into(),
    "loosely-dash-dotted" => vec![Abs::pt(3.0).into(), Abs::pt(4.0).into(), DashLength::LineWidth, Abs::pt(4.0).into()].into(),

    array: Vec<DashLength> => Self { array, phase: Length::zero() },
    mut dict: Dict => {
        let array: Vec<DashLength> = dict.take("array")?.cast()?;
        let phase = dict.take("phase").ok().map(Value::cast)
            .transpose()?.unwrap_or(Length::zero());
        dict.finish(&["array", "phase"])?;
        Self {
            array,
            phase,
        }
    },
}

/// 破線パターンにおける線分の長さ。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DashLength<T: Numeric = Length> {
    LineWidth,
    Length(T),
}

impl<T: Numeric> DashLength<T> {
    fn finish(self, line_width: T) -> T {
        match self {
            Self::LineWidth => line_width,
            Self::Length(l) => l,
        }
    }
}

impl<T: Numeric + Repr> Repr for DashLength<T> {
    fn repr(&self) -> EcoString {
        match self {
            Self::LineWidth => "dot".repr(),
            Self::Length(v) => v.repr(),
        }
    }
}

impl Resolve for DashLength {
    type Output = DashLength<Abs>;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        match self {
            Self::LineWidth => DashLength::LineWidth,
            Self::Length(v) => DashLength::Length(v.resolve(styles)),
        }
    }
}

impl From<Abs> for DashLength {
    fn from(l: Abs) -> Self {
        DashLength::Length(l.into())
    }
}

cast! {
    DashLength,
    self => match self {
        Self::LineWidth => "dot".into_value(),
        Self::Length(v) => v.into_value(),
    },
    "dot" => Self::LineWidth,
    v: Length => Self::Length(v),
}

/// 幾何学的形状の完全に指定されたストローク。
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FixedStroke {
    /// ストロークのpaint。
    pub paint: Paint,
    /// ストロークの太さ。
    pub thickness: Abs,
    /// ストロークの線端の形状。
    pub cap: LineCap,
    /// ストロークの線の接合形状。
    pub join: LineJoin,
    /// ストロークの破線パターン。
    pub dash: Option<DashPattern<Abs, Abs>>,
    /// マイター制限。デフォルトは `tiny-skia` と同じ 4.0 です。
    pub miter_limit: Scalar,
}

impl FixedStroke {
    /// paintと太さからストロークを作成します。
    pub fn from_pair(paint: impl Into<Paint>, thickness: Abs) -> Self {
        Self {
            paint: paint.into(),
            thickness,
            ..Default::default()
        }
    }
}

impl Default for FixedStroke {
    fn default() -> Self {
        Self {
            paint: Paint::Solid(Color::BLACK),
            thickness: Abs::pt(1.0),
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            dash: None,
            miter_limit: Scalar::new(4.0),
        }
    }
}
