use self::PathVertex::{AllControlPoints, MirroredControlPoint, Vertex};
use crate::diag::bail;
use crate::foundations::{Array, Reflect, Smart, array, cast, elem};
use crate::layout::{Axes, Length, Rel};
use crate::visualize::{FillRule, Paint, Stroke};

/// ベジェ曲線で結ばれた点のリストを通るパス。
///
/// # 例
/// ```example
/// #path(
///   fill: blue.lighten(80%),
///   stroke: blue,
///   closed: true,
///   (0pt, 50pt),
///   (100%, 50pt),
///   ((50%, 0pt), (40pt, 0pt)),
/// )
/// ```
#[elem]
pub struct PathElem {
    /// パスの塗りつぶし方法。
    ///
    /// fillを設定すると、デフォルトのストロークは消えます。塗りつぶしと
    /// ストロークの両方を持つ矩形を作るには、両方を設定する必要があります。
    pub fill: Option<Paint>,

    /// パスの塗りつぶしに使用する描画規則。
    ///
    /// ```example
    /// // We use `.with` to get a new
    /// // function that has the common
    /// // arguments pre-applied.
    /// #let star = path.with(
    ///   fill: red,
    ///   closed: true,
    ///   (25pt, 0pt),
    ///   (10pt, 50pt),
    ///   (50pt, 20pt),
    ///   (0pt, 20pt),
    ///   (40pt, 50pt),
    /// )
    ///
    /// #star(fill-rule: "non-zero")
    /// #star(fill-rule: "even-odd")
    /// ```
    #[default]
    pub fill_rule: FillRule,

    /// パスの[ストローク]($stroke)の方法。
    ///
    /// ストロークを無効にするには `{none}` を設定でき、塗りつぶしが指定されて
    /// いない場合に限り `{1pt}` の黒のストロークにするには `{auto}` を設定できます。
    #[fold]
    pub stroke: Smart<Option<Stroke>>,

    /// このパスを最後のベジェ曲線で閉じるかどうか。この曲線は隣接する制御点を
    /// 考慮します。直線で閉じたい場合は、始点と同じ点を最後の点として
    /// 追加するだけです。
    #[default(false)]
    pub closed: bool,

    /// パスの頂点。
    ///
    /// 各頂点は3通りの方法で定義できます。
    ///
    /// - [`line`]関数や[`polygon`]関数に与えるような通常の点。
    /// - 2つの点からなる配列。1つ目は頂点で、2つ目は制御点です。制御点は頂点
    ///   からの相対位置で表され、ミラーリングされて2つ目の制御点になります。
    ///   与えられた制御点は、（最初の点であっても）この頂点に_入ってくる_
    ///   曲線に影響します。ミラーリングされた制御点はこの頂点から出ていく
    ///   曲線に影響します。
    /// - 3つの点からなる配列。1つ目は頂点で、続く2つは制御点（それぞれ入って
    ///   くる曲線と出ていく曲線の制御点）です。
    #[variadic]
    pub vertices: Vec<PathVertex>,
}

/// パスの作成に使用されるコンポーネント。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PathVertex {
    Vertex(Axes<Rel<Length>>),
    MirroredControlPoint(Axes<Rel<Length>>, Axes<Rel<Length>>),
    AllControlPoints(Axes<Rel<Length>>, Axes<Rel<Length>>, Axes<Rel<Length>>),
}

impl PathVertex {
    pub fn vertex(&self) -> Axes<Rel<Length>> {
        match self {
            Vertex(x) => *x,
            MirroredControlPoint(x, _) => *x,
            AllControlPoints(x, _, _) => *x,
        }
    }

    pub fn control_point_from(&self) -> Axes<Rel<Length>> {
        match self {
            Vertex(_) => Axes::new(Rel::zero(), Rel::zero()),
            MirroredControlPoint(_, a) => a.map(|x| -x),
            AllControlPoints(_, _, b) => *b,
        }
    }

    pub fn control_point_to(&self) -> Axes<Rel<Length>> {
        match self {
            Vertex(_) => Axes::new(Rel::zero(), Rel::zero()),
            MirroredControlPoint(_, a) => *a,
            AllControlPoints(_, a, _) => *a,
        }
    }
}

cast! {
    PathVertex,
    self => match self {
        Vertex(x) => x.into_value(),
        MirroredControlPoint(x, c) => array![x, c].into_value(),
        AllControlPoints(x, c1, c2) => array![x, c1, c2].into_value(),
    },
    array: Array => {
        let mut iter = array.into_iter();
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), None, None, None) => {
                Vertex(a.cast()?)
            },
            (Some(a), Some(b), None, None) => {
                if Axes::<Rel<Length>>::castable(&a) {
                    MirroredControlPoint(a.cast()?, b.cast()?)
                } else {
                    Vertex(Axes::new(a.cast()?, b.cast()?))
                }
            },
            (Some(a), Some(b), Some(c), None) => {
                AllControlPoints(a.cast()?, b.cast()?, c.cast()?)
            },
            _ => bail!("path vertex must have 1, 2, or 3 points"),
        }
    },
}
