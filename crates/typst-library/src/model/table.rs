use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::Arc;

use ecow::EcoString;
use typst_utils::NonZeroExt;

use crate::diag::{HintedStrResult, HintedString, SourceResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Content, Packed, Smart, StyleChain, Synthesize, cast, elem, scope,
};
use crate::introspection::{Locatable, Tagged};
use crate::layout::resolve::{CellGrid, table_to_cellgrid};
use crate::layout::{
    Abs, Alignment, Celled, GridCell, GridFooter, GridHLine, GridHeader, GridVLine,
    Length, OuterHAlignment, OuterVAlignment, Rel, Sides, TrackSizings,
};
use crate::model::Figurable;
use crate::pdf::TableCellKind;
use crate::text::LocalName;
use crate::visualize::{Paint, Stroke};

/// 複数の項目からなる表。
///
/// 表はコンテンツをセルへ配置するために用います。
/// セルには複数の段落を含む任意のコンテンツを入れられ、その配置は行優先順序で指定します。
/// Typstにおける表の利用とカスタマイズについて、全ての手法の実践的な説明は[表ガイド]($guides/tables)をご覧ください。
///
/// 表は、いくつかのセルのプロパティ（特に`stroke`と`inset`）のデフォルト値が異なるだけのグリッドです。
/// そのため、表のトラックの大きさの指定やセルの外見に関するプロパティの指定については、[グリッドのドキュメント]($grid/#track-size)を参照してください。
///
/// 表とグリッドのどちらを使うべきか迷う場合は、配置しようとしているコンテンツが意味的に関連するデータ項目の集まりなのか、
/// それとも無関係なコンテンツをグリッド状に並べて見た目を整えたいだけなのかを検討してください。
/// 前者の場合は表が適切な選択であり、後者の場合はグリッドの方が適しています。
/// さらに、スクリーンリーダーなどの支援技術（AT）は、`table`に含まれるコンテンツを表形式として読み上げますが、
/// グリッドのコンテンツは文書の流れの中に並ぶ複数のコンテンツブロックと同じようにしか読み上げられません。
/// ATのユーザーは、表をセル単位で2次元的にたどれます。
///
/// 特定のセルのプロパティを上書きしたり、表のセルにshowルールを適用したりするには、[`table.cell`]要素を使用できます。
/// 詳細はそのドキュメントを参照してください。
///
/// `table`と`grid`はほとんどのプロパティを共有しますが、一方に対するsetルールとshowルールは他方に影響しません。
/// スタイル設定の大部分をsetルールとshowルールにまとめることを推奨します。
/// 表の実際の使用箇所が簡潔で読みやすくなり、全ての表の見た目を1か所で変更できるようになるためです。
///
/// 表にキャプションを付けて[参照可能]($ref)にするには、[図表]($figure)の中に入れてください。
///
/// # 例
///
/// 以下の例は、表の最も一般的なオプションのいくつかを示しています。
/// ```example
/// #table(
///   columns: (1fr, auto, auto),
///   inset: 10pt,
///   align: horizon,
///   table.header(
///     [], [*Volume*], [*Parameters*],
///   ),
///   image("cylinder.svg"),
///   $ pi h (D^2 - d^2) / 4 $,
///   [
///     $h$: height \
///     $D$: outer radius \
///     $d$: inner radius
///   ],
///   image("tetrahedron.svg"),
///   $ sqrt(2) / 12 a^3 $,
///   [$a$: edge length]
/// )
/// ```
///
/// グリッドと同様に、[`table.cell`]を使うことで、各セルの外見と位置をカスタマイズできます。
///
/// ```example
/// >>> #set page(width: auto)
/// >>> #set text(font: "IBM Plex Sans")
/// >>> #let gray = rgb("#565565")
/// >>>
/// #set table(
///   stroke: none,
///   gutter: 0.2em,
///   fill: (x, y) =>
///     if x == 0 or y == 0 { gray },
///   inset: (right: 1.5em),
/// )
///
/// #show table.cell: it => {
///   if it.x == 0 or it.y == 0 {
///     set text(white)
///     strong(it)
///   } else if it.body == [] {
///     // Replace empty cells with 'N/A'
///     pad(..it.inset)[_N/A_]
///   } else {
///     it
///   }
/// }
///
/// #let a = table.cell(
///   fill: green.lighten(60%),
/// )[A]
/// #let b = table.cell(
///   fill: aqua.lighten(60%),
/// )[B]
///
/// #table(
///   columns: 4,
///   [], [Exam 1], [Exam 2], [Exam 3],
///
///   [John], [], a, [],
///   [Mary], [], a, a,
///   [Robert], b, a, b,
/// )
/// ```
///
/// # アクセシビリティ { #accessibility }
/// 表は支援技術（AT）のユーザーにとって読み取りが困難です。
/// ATのユーザーの負担を減らすため、[`table.header`]と[`table.footer`]で表のヘッダー部分とフッター部分を明示することを強く推奨します。
/// これにより、ATは各セルに対応する列のラベルを読み上げられるようになります。
///
/// 表をセル単位でたどる作業は、視覚的に読む場合よりも煩雑です。
/// そのため、表の中核となる情報を文章としても提供することを検討してください。
/// これは、表を[図表]($figure)で包み、そのキャプションで表の内容を要約することで実現できます。
#[elem(scope, Locatable, Tagged, Synthesize, LocalName, Figurable)]
pub struct TableElem {
    /// 列の大きさ。
    /// トラックの大きさの設定についての詳細は、[グリッドのドキュメント]($grid/#track-size)を参照してください。
    pub columns: TrackSizings,

    /// 行の大きさ。
    /// トラックの大きさの設定についての詳細は、[グリッドのドキュメント]($grid/#track-size)を参照してください。
    pub rows: TrackSizings,

    /// 行間と列間の間隔。
    /// これは`column-gutter`と`row-gutter`に同じ値を設定するための省略記法です。
    /// 罫間についての詳細は、[グリッドのドキュメント]($grid.gutter)を参照してください。
    #[external]
    pub gutter: TrackSizings,

    /// 列間の間隔。`gutter`より優先されます。
    /// 罫間についての詳細は、[グリッドのドキュメント]($grid.gutter)を参照してください。
    #[parse(
        let gutter = args.named("gutter")?;
        args.named("column-gutter")?.or_else(|| gutter.clone())
    )]
    pub column_gutter: TrackSizings,

    /// 行間の間隔。`gutter`より優先されます。
    /// 罫間についての詳細は、[グリッドのドキュメント]($grid.gutter)を参照してください。
    #[parse(args.named("row-gutter")?.or_else(|| gutter.clone()))]
    pub row_gutter: TrackSizings,

    /// セルのコンテンツのパディング量。
    ///
    /// 全てのセルに同じインセットを指定するには、全ての辺に対して単一の長さを使用するか、各辺の長さを指定する辞書を使用します。
    /// 詳細は[ボックスのドキュメント]($box.inset)を参照してください。
    ///
    /// セルごとに異なるインセットを指定するには、以下の方法があります。
    /// - 全てのセルに対して単一の一律なインセットを使用する
    /// - 各列に対応するインセットの配列を使用する
    /// - セルのX/Y位置（いずれも0始まり）をインセットに変換する関数を使用する
    ///
    /// 詳細は[グリッドのドキュメント]($grid/#styling)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   columns: 2,
    ///   inset: 10pt,
    ///   [Hello],
    ///   [World],
    /// )
    ///
    /// #table(
    ///   columns: 2,
    ///   inset: (x: 20pt, y: 10pt),
    ///   [Hello],
    ///   [World],
    /// )
    /// ```
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Abs::pt(5.0).into()))))]
    pub inset: Celled<Sides<Option<Rel<Length>>>>,

    /// セルのコンテンツの揃え方。
    ///
    /// `{auto}`に設定すると、外側の配置が使用されます。
    ///
    /// 配置は以下のいずれかの方法で指定できます。
    /// - 全てのセルに対して単一の配置を使用する
    /// - 各列に対応する配置の配列を使用する
    /// - セルのX/Y位置（いずれも0始まり）を配置に変換する関数を使用する
    ///
    /// 詳細は[表ガイド]($guides/tables/#alignment)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   columns: 3,
    ///   align: (left, center, right),
    ///   [Hello], [Hello], [Hello],
    ///   [A], [B], [C],
    /// )
    /// ```
    pub align: Celled<Smart<Alignment>>,

    /// セルの塗りつぶし方。
    ///
    /// 以下のいずれかを指定できます。
    /// - 全てのセルに対する単一の塗りつぶし
    /// - 各列に対応する塗りつぶしの配列
    /// - セルの位置を塗りつぶしに変換する関数
    ///
    /// 特に配列と関数は、縞模様の表を作成する際に便利です。
    /// 詳細は[表ガイド]($guides/tables/#fills)を参照してください。
    ///
    /// ```example
    /// #table(
    ///   fill: (x, _) =>
    ///     if calc.odd(x) { luma(240) }
    ///     else { white },
    ///   align: (x, y) =>
    ///     if y == 0 { center }
    ///     else if x == 0 { left }
    ///     else { right },
    ///   columns: 4,
    ///   [], [*Q1*], [*Q2*], [*Q3*],
    ///   [Revenue:], [1000 €], [2000 €], [3000 €],
    ///   [Expenses:], [500 €], [1000 €], [1500 €],
    ///   [Profit:], [500 €], [1000 €], [1500 €],
    /// )
    /// ```
    pub fill: Celled<Option<Paint>>,

    /// セルの[ストローク]($stroke)をどうするか。
    ///
    /// `{none}`に設定すると、ストロークを無効にできます。
    ///
    /// [`gutter`]($table.gutter)オプションによって作成されたセル間の空白を横切る線を配置する必要がある場合や、
    /// 複数の特定のセル間のストロークを上書きする必要がある場合は、
    /// 表のセルにあわせて[`table.hline`]および[`table.vline`]のいずれか、または両方を指定することを検討してください。
    ///
    /// 全てのセルに同じストロークを指定するには、全ての辺に対して単一の[ストローク]($stroke)を使用するか、各辺の[ストローク]($stroke)を指定する辞書を使用します。
    /// 詳細は[長方形のドキュメント]($rect.stroke)を参照してください。
    ///
    /// セルごとに異なるストロークを指定するには、以下の方法があります。
    /// - 全てのセルに対して単一のストロークを使用する
    /// - 各列に対応するストロークの配列を使用する
    /// - セルの位置をストロークに変換する関数を使用する
    ///
    /// 詳細は[表ガイド]($guides/tables/#strokes)を参照してください。
    #[fold]
    #[default(Celled::Value(Sides::splat(Some(Some(Arc::new(Stroke::default()))))))]
    pub stroke: Celled<Sides<Option<Option<Arc<Stroke>>>>>,

    /// A summary of the purpose and structure of complex tables.
    ///
    /// See the [`crate::pdf::accessibility::table_summary`] function for more
    /// information.
    #[internal]
    #[parse(None)]
    pub summary: Option<EcoString>,

    #[internal]
    #[synthesized]
    pub grid: Arc<CellGrid>,

    /// 表のセルの内容と、[`table.hline`]および[`table.vline`]要素で指定された追加の線。
    #[variadic]
    pub children: Vec<TableChild>,
}

#[scope]
impl TableElem {
    #[elem]
    type TableCell;

    #[elem]
    type TableHLine;

    #[elem]
    type TableVLine;

    #[elem]
    type TableHeader;

    #[elem]
    type TableFooter;
}

impl Synthesize for Packed<TableElem> {
    fn synthesize(
        &mut self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<()> {
        let grid = table_to_cellgrid(self, engine, styles)?;
        self.grid = Some(Arc::new(grid));
        Ok(())
    }
}

impl LocalName for Packed<TableElem> {
    const KEY: &'static str = "table";
}

impl Figurable for Packed<TableElem> {}

cast! {
    TableElem,
    v: Content => v.unpack::<Self>().map_err(|_| "expected table")?,
}

/// Any child of a table element.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TableChild {
    Header(Packed<TableHeader>),
    Footer(Packed<TableFooter>),
    Item(TableItem),
}

cast! {
    TableChild,
    self => match self {
        Self::Header(header) => header.into_value(),
        Self::Footer(footer) => footer.into_value(),
        Self::Item(item) => item.into_value(),
    },
    v: Content => {
        v.try_into()?
    },
}

impl TryFrom<Content> for TableChild {
    type Error = HintedString;

    fn try_from(value: Content) -> HintedStrResult<Self> {
        if value.is::<GridHeader>() {
            bail!(
                "cannot use `grid.header` as a table header";
                hint: "use `table.header` instead"
            )
        }
        if value.is::<GridFooter>() {
            bail!(
                "cannot use `grid.footer` as a table footer";
                hint: "use `table.footer` instead"
            )
        }

        value
            .into_packed::<TableHeader>()
            .map(Self::Header)
            .or_else(|value| value.into_packed::<TableFooter>().map(Self::Footer))
            .or_else(|value| TableItem::try_from(value).map(Self::Item))
    }
}

/// A table item, which is the basic unit of table specification.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TableItem {
    HLine(Packed<TableHLine>),
    VLine(Packed<TableVLine>),
    Cell(Packed<TableCell>),
}

cast! {
    TableItem,
    self => match self {
        Self::HLine(hline) => hline.into_value(),
        Self::VLine(vline) => vline.into_value(),
        Self::Cell(cell) => cell.into_value(),
    },
    v: Content => {
        v.try_into()?
    },
}

impl TryFrom<Content> for TableItem {
    type Error = HintedString;

    fn try_from(value: Content) -> HintedStrResult<Self> {
        if value.is::<GridHeader>() {
            bail!("cannot place a grid header within another header or footer");
        }
        if value.is::<TableHeader>() {
            bail!("cannot place a table header within another header or footer");
        }
        if value.is::<GridFooter>() {
            bail!("cannot place a grid footer within another footer or header");
        }
        if value.is::<TableFooter>() {
            bail!("cannot place a table footer within another footer or header");
        }
        if value.is::<GridCell>() {
            bail!(
                "cannot use `grid.cell` as a table cell";
                hint: "use `table.cell` instead"
            );
        }
        if value.is::<GridHLine>() {
            bail!(
                "cannot use `grid.hline` as a table line";
                hint: "use `table.hline` instead"
            );
        }
        if value.is::<GridVLine>() {
            bail!(
                "cannot use `grid.vline` as a table line";
                hint: "use `table.vline` instead"
            );
        }

        Ok(value
            .into_packed::<TableHLine>()
            .map(Self::HLine)
            .or_else(|value| value.into_packed::<TableVLine>().map(Self::VLine))
            .or_else(|value| value.into_packed::<TableCell>().map(Self::Cell))
            .unwrap_or_else(|value| {
                let span = value.span();
                Self::Cell(Packed::new(TableCell::new(value)).spanned(span))
            }))
    }
}

/// 繰り返し可能な表のヘッダー。
///
/// 表を複数ページにまたがらせる予定がない場合でも、表の見出し行はこの関数で包むべきです。
/// Typstはこの関数を使って表にアクセシビリティのメタデータを付与し、文書への[ユニバーサルアクセス]($guides/accessibility/#basics)を確保するためです。
///
/// `repeat`パラメーターを使うことで、表のヘッダーをページごとに繰り返すかどうかを制御できます。
///
/// 現在のところ、この関数はヘッダーの列や単独のヘッダーセルの作成には適していません。
/// 通常のセルを使用するか、PDFへエクスポートする場合は[`pdf.header-cell`]関数でセルをヘッダーセルとして明示できます。
/// 同様に、[`pdf.data-cell`]を使うことで、この関数内のセルをデータセルとして明示できます。
/// なお、これらの関数の仕様は未確定であるため、`a11y-extras`フィーチャーを有効にした場合にのみ利用できます。
/// 詳細は[PDFモジュールのドキュメント]($pdf)を参照してください。
///
/// ```example
/// #set page(height: 11.5em)
/// #set table(
///   fill: (x, y) =>
///     if x == 0 or y == 0 {
///       gray.lighten(40%)
///     },
///   align: right,
/// )
///
/// #show table.cell.where(x: 0): strong
/// #show table.cell.where(y: 0): strong
///
/// #table(
///   columns: 4,
///   table.header(
///     [], [Blue chip],
///     [Fresh IPO], [Penny st'k],
///   ),
///   table.cell(
///     rowspan: 6,
///     align: horizon,
///     rotate(-90deg, reflow: true)[
///       *USD / day*
///     ],
///   ),
///   [0.20], [104], [5],
///   [3.17], [108], [4],
///   [1.59], [84],  [1],
///   [0.26], [98],  [15],
///   [0.01], [195], [4],
///   [7.34], [57],  [2],
/// )
/// ```
#[elem(name = "header", title = "Table Header")]
pub struct TableHeader {
    /// ページごとにヘッダーを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// ヘッダーのレベル。0にはできません。
    ///
    /// これにより、複数のヘッダーを同時に繰り返せます。
    /// 異なるレベルを持つヘッダーは、レベルが昇順であれば一緒に繰り返せます。
    ///
    /// 特に、より低いレベルのヘッダーが繰り返しを開始すると、それ以上のレベルのヘッダーの繰り返しは停止します（新しいヘッダーに「置き換え」られます）。
    #[default(NonZeroU32::ONE)]
    pub level: NonZeroU32,

    /// ヘッダー内のセルと線。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 繰り返し可能な表のフッター。
///
/// [`table.header`]要素と同様に、フッターは表の各ページで繰り返し表示できます。
/// これは、大きな表のヘッダーとフッターの両方に列のラベルを配置したり、合計値やページごとに見えるべきその他の情報を配置したりして、読みやすさを向上させる際に便利です。
///
/// フッターの後に他の表のセルを配置できません。
#[elem(name = "footer", title = "Table Footer")]
pub struct TableFooter {
    /// ページごとにフッターを繰り返すかどうか。
    #[default(true)]
    pub repeat: bool,

    /// フッター内のセルと線。
    #[variadic]
    pub children: Vec<TableItem>,
}

/// 表の水平方向の線。
///
/// 表の`stroke`フィールドを用いて指定されたものを含めて、セルごとに設定されたストロークを上書きします。
/// 表の[`column-gutter`]($table.column-gutter)オプションで作成されたセル間の間隔をまたげます。
///
/// 単一の表の特定の位置に水平方向の線を手動で配置したい場合は、表の`stroke`フィールドの代わりにこの関数を使用してください。
/// 配置したい線が全ての表のデザインの一部である場合は、代わりに[表の`stroke`]($table.stroke)フィールドや[`table.cell`の`stroke`]($table.cell.stroke)フィールドの使用を検討してください。
///
/// ```example
/// #set table.hline(stroke: .6pt)
///
/// #table(
///   stroke: none,
///   columns: (auto, 1fr),
///   [09:00], [Badge pick up],
///   [09:45], [Opening Keynote],
///   [10:30], [Talk: Typst's Future],
///   [11:15], [Session: Good PRs],
///   table.hline(start: 1),
///   [Noon], [_Lunch break_],
///   table.hline(start: 1),
///   [14:00], [Talk: Tracked Layout],
///   [15:00], [Talk: Automations],
///   [16:00], [Workshop: Tables],
///   table.hline(),
///   [19:00], [Day 1 Attendee Mixer],
/// )
/// ```
#[elem(name = "hline", title = "Table Horizontal Line")]
pub struct TableHLine {
    /// 配置される水平方向の線の下にある行（0始まり）。
    /// [`grid.hline`]($grid.hline.y)の`y`フィールドと同じように動作します。
    pub y: Smart<usize>,

    /// 水平方向の線を開始する列（0始まりで、指定した列を含む）。
    pub start: usize,

    /// 水平方向の線が終了する直前の列（0始まりで、指定した列は含まない）。
    pub end: Option<NonZeroUsize>,

    /// 線のストローク。
    ///
    /// `{none}`を指定すると、水平方向の線の範囲にこれまで配置された全ての線が削除されます。
    /// これには水平方向の線の下にあるhlineやセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 行（`y`）が与えられた場合の線が配置される位置。
    /// `{top}`か`{bottom}`のいずれかを指定し、それぞれその行の上または下に描画します。
    ///
    /// この設定は`row-gutter`が設定されている場合にのみ有効です（それ以外の場合は使用せず、単に`y`フィールドの値を1ずつ増やしてください）。
    /// これは、行の下部の位置と次の行の上部の位置の間に空白が発生し、両者が一致しなくなるためです。
    #[default(OuterVAlignment::Top)]
    pub position: OuterVAlignment,
}

/// 表の垂直方向の線。
/// この要素のフィールドの使い方についての詳細は、[`grid.vline`]のドキュメントを参照してください。
///
/// 表の`stroke`フィールドを用いて指定されたものを含めて、セルごとに設定されたストロークを上書きします。
/// 表の[`row-gutter`]($table.row-gutter)オプションで作成されたセル間の間隔をまたげます。
///
/// [`table.hline`]と同様に、単一の表の特定の位置に垂直方向の線を手動で配置したい場合はこの関数を使用してください。
/// 配置したい線が全ての表のデザインの一部である場合は、代わりに[表の`stroke`]($table.stroke)フィールドや[`table.cell`の`stroke`]($table.cell.stroke)フィールドを使用してください。
#[elem(name = "vline", title = "Table Vertical Line")]
pub struct TableVLine {
    /// 配置される垂直方向の線の後にある列（0始まり）。
    /// [`grid.vline`]の`x`フィールドと同じように動作します。
    pub x: Smart<usize>,

    /// 垂直方向の線を開始する行（0始まりで、指定した行を含む）。
    pub start: usize,

    /// 垂直方向の線が終了する直前の行（0始まりで、指定した行は含まない）。
    pub end: Option<NonZeroUsize>,

    /// 線のストローク。
    ///
    /// `{none}`を指定すると、垂直方向の線の範囲にこれまで配置された全ての線が削除されます。
    /// これには垂直方向の線の下にあるvlineやセルごとのストロークが含まれます。
    #[fold]
    #[default(Some(Arc::new(Stroke::default())))]
    pub stroke: Option<Arc<Stroke>>,

    /// 列（`x`）が与えられた場合の線が配置される位置。
    /// `{start}`か`{end}`のいずれかを指定し、それぞれその列の前または後に描画します。
    ///
    /// `{left}`と`{right}`も指定できますが、左横書きの文書と右横書きの文書とで表の見え方が一致しなくなるため推奨しません。
    ///
    /// この設定は`column-gutter`が設定されている場合にのみ有効です（それ以外の場合は使用せず、単に`x`フィールドの値を1ずつ増やしてください）。
    /// これは、列の後の位置と次の列の前の位置の間に空白が発生し、両者が一致しなくなるためです。
    #[default(OuterHAlignment::Start)]
    pub position: OuterHAlignment,
}

/// 表のセル。
/// セルを手動で配置したり、スタイルを適用したりするために使用します。
/// 後者の場合、この関数で特定のセルのプロパティを上書きするか、showルールで用いて複数のセルに特定のスタイルを一度に適用できます。
///
/// `{table.cell}`のおそらく最も重要な用途は、`colspan`および`rowspan`フィールドによって、セルを複数の列や行にまたがらせることです。
///
/// ```example
/// >>> #set page(width: auto)
/// #show table.cell.where(y: 0): strong
/// #set table(
///   stroke: (x, y) => if y == 0 {
///     (bottom: 0.7pt + black)
///   },
///   align: (x, y) => (
///     if x > 0 { center }
///     else { left }
///   )
/// )
///
/// #table(
///   columns: 3,
///   table.header(
///     [Substance],
///     [Subcritical °C],
///     [Supercritical °C],
///   ),
///   [Hydrochloric Acid],
///   [12.0], [92.1],
///   [Sodium Myreth Sulfate],
///   [16.6], [104],
///   [Potassium Hydroxide],
///   table.cell(colspan: 2)[24.7],
/// )
/// ```
///
/// 例えば、以下のようにある単一のセルの塗りつぶし、配置、インセットを上書きできます。
///
/// ```example
/// >>> #set page(width: auto)
/// // You can also import those.
/// #import table: cell, header
///
/// #table(
///   columns: 2,
///   align: center,
///   header(
///     [*Trip progress*],
///     [*Itinerary*],
///   ),
///   cell(
///     align: right,
///     fill: fuchsia.lighten(80%),
///     [🚗],
///   ),
///   [Get in, folks!],
///   [🚗], [Eat curbside hotdog],
///   cell(align: left)[🌴🚗],
///   cell(
///     inset: 0.06em,
///     text(1.62em)[🏝️🌅🌊],
///   ),
/// )
/// ```
///
/// `table.cell`へshowルールを適用すれば、全てのセルのスタイルを一度に設定できます。
/// セレクターと組み合わせれば、以下のようにセルの位置に基づいてスタイルを適用できます。
///
/// ```example
/// #show table.cell.where(x: 0): strong
///
/// #table(
///   columns: 3,
///   gutter: 3pt,
///   [Name], [Age], [Strength],
///   [Hannes], [36], [Grace],
///   [Irma], [50], [Resourcefulness],
///   [Vikram], [49], [Perseverance],
/// )
/// ```
#[elem(name = "cell", title = "Table Cell")]
pub struct TableCell {
    /// セルの本文。
    #[required]
    pub body: Content,

    /// セルの列（0始まり）。
    /// [`grid.cell`]の`x`フィールドと同じように動作します。
    pub x: Smart<usize>,

    /// セルの行（0始まり）。
    /// [`grid.cell`]の`y`フィールドと同じように動作します。
    pub y: Smart<usize>,

    /// このcellがまたぐ列の数。
    #[default(NonZeroUsize::ONE)]
    pub colspan: NonZeroUsize,

    /// このcellがまたぐ行の数。
    #[default(NonZeroUsize::ONE)]
    pub rowspan: NonZeroUsize,

    /// セルの[インセット]($table.inset)の上書き。
    pub inset: Smart<Sides<Option<Rel<Length>>>>,

    /// セルの[配置]($table.align)の上書き。
    pub align: Smart<Alignment>,

    /// セルの[塗りつぶし]($table.fill)の上書き。
    pub fill: Smart<Option<Paint>>,

    /// セルの[ストローク]($table.stroke)の上書き。
    #[fold]
    pub stroke: Sides<Option<Option<Arc<Stroke>>>>,

    /// このcellがまたぐ行を異なるページに配置できるかどうか。
    /// `{auto}`の場合、固定サイズの行のみをまたぐセルは分割不可となり、
    /// `{auto}`サイズの行を1つ以上またぐセルは分割可能となります。
    pub breakable: Smart<bool>,

    #[internal]
    #[parse(Some(Smart::Auto))]
    pub kind: Smart<TableCellKind>,

    #[internal]
    #[parse(Some(false))]
    pub is_repeated: bool,
}

cast! {
    TableCell,
    v: Content => v.into(),
}

impl Default for Packed<TableCell> {
    fn default() -> Self {
        Packed::new(
            // Explicitly set colspan and rowspan to ensure they won't be
            // overridden by set rules (default cells are created after
            // colspans and rowspans are processed in the resolver)
            TableCell::new(Content::default())
                .with_colspan(NonZeroUsize::ONE)
                .with_rowspan(NonZeroUsize::ONE),
        )
    }
}

impl From<Content> for TableCell {
    fn from(value: Content) -> Self {
        #[allow(clippy::unwrap_or_default)]
        value.unpack::<Self>().unwrap_or_else(Self::new)
    }
}
