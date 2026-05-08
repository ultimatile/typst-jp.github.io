//! 文章を扱う処理。

mod case;
mod deco;
mod font;
mod item;
mod lang;
mod linebreak;
#[path = "lorem.rs"]
mod lorem_;
mod raw;
mod shift;
#[path = "smallcaps.rs"]
mod smallcaps_;
mod smartquote;
mod space;

pub use self::case::*;
pub use self::deco::*;
pub use self::font::*;
pub use self::item::*;
pub use self::lang::*;
pub use self::linebreak::*;
pub use self::lorem_::*;
pub use self::raw::*;
pub use self::shift::*;
pub use self::smallcaps_::*;
pub use self::smartquote::*;
pub use self::space::*;

use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::LazyLock;

use ecow::{EcoString, eco_format};
use icu_properties::sets::CodePointSetData;
use icu_provider::AsDeserializingBufferProvider;
use icu_provider_blob::BlobDataProvider;
use rustybuzz::Feature;
use smallvec::SmallVec;
use ttf_parser::Tag;
use typst_syntax::Spanned;
use typst_utils::singleton;

use crate::World;
use crate::diag::{HintedStrResult, SourceResult, StrResult, bail, warning};
use crate::engine::Engine;
use crate::foundations::{
    Args, Array, Cast, Construct, Content, Dict, Fold, IntoValue, NativeElement, Never,
    NoneValue, Packed, PlainText, Regex, Repr, Resolve, Scope, Set, Smart, StyleChain,
    cast, dict, elem,
};
use crate::layout::{Abs, Axis, Dir, Em, Length, Ratio, Rel};
use crate::math::{EquationElem, MathSize};
use crate::visualize::{Color, Paint, RelativeTo, Stroke};

/// `text` の全ての定義を結びつけます。
pub(super) fn define(global: &mut Scope) {
    global.start_category(crate::Category::Text);
    global.define_elem::<TextElem>();
    global.define_elem::<LinebreakElem>();
    global.define_elem::<SmartQuoteElem>();
    global.define_elem::<SubElem>();
    global.define_elem::<SuperElem>();
    global.define_elem::<UnderlineElem>();
    global.define_elem::<OverlineElem>();
    global.define_elem::<StrikeElem>();
    global.define_elem::<HighlightElem>();
    global.define_elem::<SmallcapsElem>();
    global.define_elem::<RawElem>();
    global.define_func::<lower>();
    global.define_func::<upper>();
    global.define_func::<lorem>();
    global.reset_category();
}

/// 文章の見た目とレイアウトをさまざまな方法でカスタマイズします。
///
/// この関数は、setルールを使う方法と直接呼び出す方法の双方で頻繁に使用されます。
/// 多くの場合setルールを使うほうが簡潔ですが、文章を別の関数の引数として渡すような
/// 場合には、`text`関数を直接呼び出すことが有用です。
///
/// # 例
/// ```example
/// #set text(18pt)
/// With a set rule.
///
/// #emph(text(blue)[
///   With a function call.
/// ])
/// ```
#[elem(Debug, Construct, PlainText, Repr)]
pub struct TextElem {
    /// フォントファミリー記述子、またはその優先順位付きリスト。
    ///
    /// フォントファミリー記述子は、ファミリー名を表す単純な文字列、または以下のキーを
    /// 持つ辞書として指定できます：
    ///
    /// - `name` (必須)：フォントファミリー名。
    /// - `covers` (任意)：そのファミリーを使用するUnicodeコードポイントを定義します。
    ///   以下のいずれかを指定できます：
    ///   - 定義済みのカバレッジ集合：
    ///     - `{"latin-in-cjk"}` はラテンフォントに存在するが、CJKフォントから取得する
    ///       ほうが望ましいコードポイントを除く全てのコードポイントをカバーします。
    ///   - カバーすべきコードポイントを正確に定義する[正規表現]($regex)。1個のドット、
    ///     文字、または文字クラスのみで構成される正規表現の部分集合のみが受け付けられます。
    ///
    /// 文章を処理するとき、Typstは指定された全てのフォントファミリーを順に試し、必要な
    /// グリフを持つフォントを見つけるまで探索します。以下の例では、フォント `Inria Serif`
    /// が優先されますが、アラビア語のグリフを含まないため、アラビア語のテキストには代わりに
    /// `Noto Sans Arabic` が使用されます。
    ///
    /// 利用可能なフォントの集合はプラットフォームによって異なります：
    ///
    /// - Webアプリでは、「Ag」ボタンをクリックすると利用可能なフォントの一覧を確認できます。
    ///   `.ttf` または `.otf` ファイルをプロジェクトにアップロードすることで、追加のフォントを
    ///   提供できます。それらは自動的に検出されます。優先順位は、プロジェクトのフォント >
    ///   サーバーのフォントです。
    ///
    /// - ローカルでは、Typstはインストールされたシステムフォント、またはCLIに埋め込まれた
    ///   フォント（`Libertinus Serif`、`New Computer Modern`、`New Computer Modern Math`、
    ///   `DejaVu Sans Mono`）を使用します。これに加えて、`--font-path` 引数または
    ///   `TYPST_FONT_PATHS` 環境変数を使用することで、フォントを走査するディレクトリを
    ///   追加できます。優先順位は、`--font-paths` > システムフォント > 埋め込みフォントです。
    ///   `typst fonts` を実行すると、Typstがシステム上で検出したフォントを確認できます。
    ///   なお、CLIに `--ignore-system-fonts` パラメーターを渡すことで、Typstがシステム
    ///   フォントを検索しないようにすることもできます。
    ///
    /// ```example
    /// #set text(font: "PT Sans")
    /// This is sans-serif.
    ///
    /// #set text(font: (
    ///   "Inria Serif",
    ///   "Noto Sans Arabic",
    /// ))
    ///
    /// This is Latin. \
    /// هذا عربي.
    ///
    /// // Change font only for numbers.
    /// #set text(font: (
    ///   (name: "PT Sans", covers: regex("[0-9]")),
    ///   "Libertinus Serif"
    /// ))
    ///
    /// The number 123.
    ///
    /// // Mix Latin and CJK fonts.
    /// #set text(font: (
    ///   (name: "Inria Serif", covers: "latin-in-cjk"),
    ///   "Noto Serif CJK SC"
    /// ))
    /// 分别设置“中文”和English字体
    /// ```
    #[parse({
        let font_list: Option<Spanned<FontList>> = args.named("font")?;
        if let Some(list) = &font_list {
            check_font_list(engine, list);
        }
        font_list.map(|font_list| font_list.v)
    })]
    #[default(FontList(vec![FontFamily::new("Libertinus Serif")]))]
    #[ghost]
    pub font: FontList,

    /// 主要なフォントリストに一致するものがない場合に、最終手段としてのフォント
    /// フォールバックを許可するかどうか。これにより、Typstは利用可能な全ての
    /// フォントから、必要なグリフを持つ最も近いフォントを検索します。
    ///
    /// _注：_ 現在、フォールバックが無効でグリフが見つからない場合に警告は表示
    /// されません。代わりに、文章は「豆腐」の形（適切なグリフがないことを示す小さな
    /// 四角）で表示されます。将来的には、何か問題があることを知らせるための警告を
    /// Typstに発行させられるようになる予定です。
    ///
    /// ```example
    /// #set text(font: "Inria Serif")
    /// هذا عربي
    ///
    /// #set text(fallback: false)
    /// هذا عربي
    /// ```
    #[default(true)]
    #[ghost]
    pub fallback: bool,

    /// 望ましいフォントスタイル。
    ///
    /// イタリックスタイルが要求されてオブリークスタイルしか利用できない場合、
    /// オブリークスタイルが使用されます。逆もまた同様で、イタリックスタイルは
    /// オブリークスタイルの代わりとして利用できます。イタリックもオブリークも
    /// 利用できない場合、Typstはノーマルスタイルを選択します。ほとんどのフォントは
    /// イタリックかオブリークのどちらか一方しか利用できないため、イタリックスタイルと
    /// オブリークスタイルの違いはほとんど見分けがつきません。
    ///
    /// 文章を強調したい場合は、代わりに [emph]($emph) 関数を使用すべきです。
    /// これにより、後から強調の表現方法を変更したくなった場合に、スタイルを
    /// 簡単に適応させられます。
    ///
    /// ```example
    /// #text(font: "Libertinus Serif", style: "italic")[Italic]
    /// #text(font: "DejaVu Sans", style: "oblique")[Oblique]
    /// ```
    #[ghost]
    pub style: FontStyle,

    /// 望ましいフォントグリフの太さ。`{100}` から `{900}` の整数、または定義済みの
    /// ウェイト名のいずれかを受け付けます。望ましいウェイトが利用できない場合、
    /// Typstはファミリー内でウェイトが最も近いフォントを選択します。
    ///
    /// 文章を強く強調したい場合は、代わりに [strong]($strong) 関数を使用すべきです。
    /// これにより、後から強い強調の表現方法を変更したくなった場合に、スタイルを
    /// 簡単に適応させられます。
    ///
    /// ```example
    /// #set text(font: "IBM Plex Sans")
    ///
    /// #text(weight: "light")[Light] \
    /// #text(weight: "regular")[Regular] \
    /// #text(weight: "medium")[Medium] \
    /// #text(weight: 500)[Medium] \
    /// #text(weight: "bold")[Bold]
    /// ```
    #[ghost]
    pub weight: FontWeight,

    /// 望ましいグリフの幅。`{50%}` から `{200%}` の比率を受け付けます。望ましい
    /// 幅が利用できない場合、Typstはファミリー内でストレッチが最も近いフォントを
    /// 選択します。これは、フォントの縮小版または拡張版が利用可能な場合にのみ
    /// 文章を伸縮させます。
    ///
    /// グリフ自体を伸縮させるのではなく、文字間のスペース量を調整したい場合は、
    /// 代わりに [`tracking`]($text.tracking) プロパティを使用してください。
    ///
    /// ```example
    /// #text(stretch: 75%)[Condensed] \
    /// #text(stretch: 100%)[Normal]
    /// ```
    #[ghost]
    pub stretch: FontStretch,

    /// グリフのサイズ。この値は `em` 単位の基準となります：`{1em}` はフォントサイズに
    /// 等しくなります。
    ///
    /// フォントサイズ自体を `em` 単位で指定することもできます。その場合は、直前の
    /// フォントサイズに対する相対値になります。
    ///
    /// ```example
    /// #set text(size: 20pt)
    /// very #text(1.5em)[big] text
    /// ```
    #[parse(args.named_or_find("size")?)]
    #[fold]
    #[default(TextSize(Abs::pt(11.0).into()))]
    #[ghost]
    pub size: TextSize,

    /// グリフの塗りつぶしのペイント。
    ///
    /// ```example
    /// #set text(fill: red)
    /// This text is red.
    /// ```
    #[parse({
        let paint: Option<Spanned<Paint>> = args.named_or_find("fill")?;
        if let Some(paint) = &paint
            && paint.v.relative() == Smart::Custom(RelativeTo::Self_) {
                bail!(
                    paint.span,
                    "gradients and tilings on text must be relative to the parent";
                    hint: "make sure to set `relative: auto` on your text fill"
                );
            }
        paint.map(|paint| paint.v)
    })]
    #[default(Color::BLACK.into())]
    #[ghost]
    pub fill: Paint,

    /// 文章をどのようにストロークするか。
    ///
    /// ```example
    /// #text(stroke: 0.5pt + red)[Stroked]
    /// ```
    #[ghost]
    pub stroke: Option<Stroke>,

    /// 文字間に追加されるスペースの量。
    ///
    /// ```example
    /// #set text(tracking: 1.5pt)
    /// Distant text.
    /// ```
    #[ghost]
    pub tracking: Length,

    /// 単語間のスペースの量。
    ///
    /// 絶対長で指定できますが、フォント内のスペース文字の幅に対する相対値としても
    /// 指定できます。
    ///
    /// 単語間ではなく文字間のスペース量を調整したい場合は、代わりに
    /// [`tracking`]($text.tracking) プロパティを使用してください。
    ///
    /// ```example
    /// #set text(spacing: 200%)
    /// Text with distant words.
    /// ```
    #[default(Rel::one())]
    #[ghost]
    pub spacing: Rel<Length>,

    /// CJK文字とラテン文字の間に自動的にスペースを挿入するかどうか。
    ///
    /// ```example
    /// #set text(cjk-latin-spacing: auto)
    /// 第4章介绍了基本的API。
    ///
    /// #set text(cjk-latin-spacing: none)
    /// 第4章介绍了基本的API。
    /// ```
    #[ghost]
    pub cjk_latin_spacing: Smart<Option<Never>>,

    /// 文章のベースラインをずらす量。
    ///
    /// ```example
    /// A #text(baseline: 3pt)[lowered]
    /// word.
    /// ```
    #[ghost]
    pub baseline: Length,

    /// 均等割り付けされた文章において、特定のグリフを余白にはみ出させるかどうか。
    /// これにより、均等割り付けが視覚的により美しくなります。
    ///
    /// ```example
    /// #set page(width: 220pt)
    ///
    /// #set par(justify: true)
    /// This justified text has a hyphen in
    /// the paragraph's second line. Hanging
    /// the hyphen slightly into the margin
    /// results in a clearer paragraph edge.
    ///
    /// #set text(overhang: false)
    /// This justified text has a hyphen in
    /// the paragraph's second line. Hanging
    /// the hyphen slightly into the margin
    /// results in a clearer paragraph edge.
    /// ```
    #[default(true)]
    #[ghost]
    pub overhang: bool,

    /// レイアウトと配置のために用いられる、文章の周りの概念的な枠の上端。
    /// これは、文章を保持するコンテナのサイズに影響します。
    ///
    /// ```example
    /// #set rect(inset: 0pt)
    /// #set text(size: 20pt)
    ///
    /// #set text(top-edge: "ascender")
    /// #rect(fill: aqua)[Typst]
    ///
    /// #set text(top-edge: "cap-height")
    /// #rect(fill: aqua)[Typst]
    /// ```
    #[default(TopEdge::Metric(TopEdgeMetric::CapHeight))]
    #[ghost]
    pub top_edge: TopEdge,

    /// レイアウトと配置のために用いられる、文章の周りの概念的な枠の下端。
    /// これは、文章を保持するコンテナのサイズに影響します。
    ///
    /// ```example
    /// #set rect(inset: 0pt)
    /// #set text(size: 20pt)
    ///
    /// #set text(bottom-edge: "baseline")
    /// #rect(fill: aqua)[Typst]
    ///
    /// #set text(bottom-edge: "descender")
    /// #rect(fill: aqua)[Typst]
    /// ```
    #[default(BottomEdge::Metric(BottomEdgeMetric::Baseline))]
    #[ghost]
    pub bottom_edge: BottomEdge,

    /// [ISO 639-1/2/3 言語コード。](https://en.wikipedia.org/wiki/ISO_639)
    ///
    /// 正しい言語を設定すると、Typstのさまざまな部分に影響します：
    ///
    /// - 文章処理のパイプラインでより適切な選択ができるようになります。
    /// - ハイフネーションがその言語に適したパターンを使用します。
    /// - [スマートクォート]($smartquote)がその言語に適したクォートに変換されます。
    /// - その他、言語を認識する全ての処理。
    ///
    /// 正しい言語を選択することはアクセシビリティの観点で重要です。例えば、スクリーン
    /// リーダーは文章の言語に合った音声を選ぶためにこの設定を使用します。文書が英語
    /// （デフォルト）以外の言語の場合は、他のいかなるコンテンツよりも前に、文書の冒頭で
    /// 文章の言語を設定すべきです。例えば、[文書のタイトルを設定する]($document.title)
    /// `[#set document(/* ... */)]` ルールの直後に配置できます。
    ///
    /// 文書が主要言語と異なる言語の文章を含む場合、その部分だけ局所的に文章の言語を
    /// 変更すべきです。これは、[ブロックにスコープされた]($scripting/#blocks) setルール、
    /// または`[#text(lang: "de")[...]]`のような直接の`text`関数呼び出しを使って行えます。
    ///
    /// その言語に複数のコードが利用可能な場合は、3文字のコード（ISO 639-2/3）よりも
    /// 2文字のコード（ISO 639-1）を優先すべきです。3文字のコードを使用しなければならず、
    /// かつその言語のコードがISO 639-2とISO 639-3で異なる場合は、PDF 1.7（TypstのPDF
    /// エクスポートのデフォルト）以下ではISO 639-2を、PDF 2.0およびHTMLエクスポートでは
    /// ISO 639-3を使用してください。
    ///
    /// 言語コードは大文字小文字を区別せず、[コンテキスト]($context)を介して取得する際には
    /// 小文字に変換されます。
    ///
    /// ```example:"Setting the text language to German"
    /// #set text(lang: "de")
    /// #outline()
    ///
    /// = Einleitung
    /// In diesem Dokument, ...
    /// ```
    #[default(Lang::ENGLISH)]
    #[ghost]
    pub lang: Lang,

    /// [ISO 3166-1 alpha-2 地域コード。](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    ///
    /// この設定により、文章処理のパイプラインでより適切な選択ができるようになります。
    ///
    /// 地域コードは大文字小文字を区別せず、[コンテキスト]($context)を介して取得する
    /// 際には大文字に変換されます。
    #[ghost]
    pub region: Option<Region>,

    /// OpenType writing script（OpenType書記体系）。
    ///
    /// `{lang}` と `{script}` の組み合わせにより、グリフの置換などのフォント
    /// 機能の実装方法が決定されます。多くの場合、その値はISO 15924の書記体系
    /// 識別子を変更したもの（全て小文字）で、`math` writing scriptは数式記号に
    /// 適した機能のために使用されます。
    ///
    /// デフォルトかつ推奨される設定の `{auto}` を指定すると、共通のUnicode書記体系
    /// プロパティを共有する文字のブロックごとに適切な書記体系が選択されます。
    ///
    /// ```example
    /// #set text(
    ///   font: "Libertinus Serif",
    ///   size: 20pt,
    /// )
    ///
    /// #let scedilla = [Ş]
    /// #scedilla // S with a cedilla
    ///
    /// #set text(lang: "ro", script: "latn")
    /// #scedilla // S with a subscript comma
    ///
    /// #set text(lang: "ro", script: "grek")
    /// #scedilla // S with a cedilla
    /// ```
    #[ghost]
    pub script: Smart<WritingScript>,

    /// 文章とインライン要素の主要な方向。指定可能な値は以下の通りです：
    ///
    /// - `{auto}`：`lang` プロパティから方向を自動的に推測します。
    /// - `{ltr}`：文章を左から右へレイアウトします。
    /// - `{rtl}`：文章を右から左へレイアウトします。
    ///
    /// アラビア語やヘブライ語などの右から左へ書く書記体系で記述する場合は、
    /// [文章の言語]($text.lang)または方向を設定すべきです。文章の個々の連なりは
    /// 自動的に正しい方向にレイアウトされますが、主要な方向を設定することで、
    /// 双方向並べ替えアルゴリズムに句読点とインライン要素を正しく配置するために
    /// 必要な情報が与えられます。さらに、方向の設定は `start` と `end` の整列値に
    /// 影響します。これらは `ltr` の文章では `left` と `right` に等しく、`rtl` の
    /// 文章ではその逆になります。
    ///
    /// これを `rtl` に設定したときにバグや何らかの見た目の問題が生じた場合は、
    /// [フォーラム](https://forum.typst.app/)、
    /// [Discordサーバー](https://discord.gg/2uDybryKPe)、または
    /// [お問い合わせフォーム](https://typst.app/contact)を通してご連絡ください。
    ///
    /// ```example
    /// #set text(dir: rtl)
    /// هذا عربي.
    /// ```
    #[ghost]
    pub dir: TextDir,

    /// 改行を改善するために文章をハイフネーションするかどうか。`{auto}` のときは、
    /// 均等割り付けが有効な場合に限り、文章がハイフネーションされます。
    ///
    /// [文章の言語]($text.lang)を設定することで、適切なハイフネーションパターンが
    /// 使用されることが保証されます。
    ///
    /// ```example
    /// #set page(width: 200pt)
    ///
    /// #set par(justify: true)
    /// This text illustrates how
    /// enabling hyphenation can
    /// improve justification.
    ///
    /// #set text(hyphenate: false)
    /// This text illustrates how
    /// enabling hyphenation can
    /// improve justification.
    /// ```
    #[ghost]
    pub hyphenate: Smart<bool>,

    /// 文章をレイアウトする際のさまざまな選択肢に対する「コスト」。コストが高いほど、
    /// レイアウトエンジンはその選択をしにくくなります。コストはデフォルトのコストに
    /// 対する比率として指定されるため、`{50%}` を指定すると、文章のレイアウトは特定の
    /// 選択を2倍積極的に行うようになり、`{200%}` を指定すると半分の積極性になります。
    ///
    /// 現在、以下のコストをカスタマイズできます。
    /// - `hyphenation`：単語を複数行にまたいで分割すること
    /// - `runt`：段落を1単語のみの行で終わらせること
    /// - `widow`：段落の最初の1行を次のページに残すこと
    /// - `orphan`：段落の最後の1行を前のページに残すこと
    ///
    /// ハイフネーションは、一般に単語全体を次の行に置くことで回避されるため、
    /// ハイフネーションコストを高くすると、均等割り付けのスペースが不自然になる
    /// 場合があります。注：ハイフネーションコストは、[`linebreaks`]($par.linebreaks)
    /// が "optimized" に設定されている場合にのみ適用されます（例えばデフォルトでは
    /// [`justify`]($par.justify)によって暗黙的に設定されます）。
    ///
    /// runtは前の行に置く単語を増減させることで回避されるため、runtコストを高くすると、
    /// 均等割り付けのスペースがより不自然になる場合があります。
    ///
    /// 文章レイアウトでは、widowとorphanはスタイルガイドで一般に推奨されないため、
    /// デフォルトで防止されます。しかし、防止する方法（行を次のページに移動する）は
    /// ページ間の行数が不均等になる原因にもなるため、文脈によってはそれらが許可される
    /// 場合もあります。`widow` と `orphan` のコストにより、これらの調整を無効化できます。
    /// （現在、`{0%}` を指定するとwidow/orphanが許可され、デフォルトの `{100%}` を
    /// 含むそれ以外の値を指定すると防止されます。これらの調整に対するより細やかな
    /// コスト指定は将来的に予定されています）。
    ///
    /// ```example
    /// #set text(hyphenate: true, size: 11.4pt)
    /// #set par(justify: true)
    ///
    /// #lorem(10)
    ///
    /// // Set hyphenation to ten times the normal cost.
    /// #set text(costs: (hyphenation: 1000%))
    ///
    /// #lorem(10)
    /// ```
    #[fold]
    #[ghost]
    pub costs: Costs,

    /// カーニングを適用するかどうか。
    ///
    /// 有効にすると、視覚的により美しい結果を得るために、特定の文字の組み合わせが
    /// 互いに近づいたり遠ざかったりするようになります。以下の例では、「T」と「o」の
    /// 間隔を狭めることでより自然な見た目になることを示しています。これを `{false}` に
    /// 設定すると、OpenTypeの `kern` フォント機能がオフになり、カーニングが無効化
    /// されます。
    ///
    /// ```example
    /// #set text(size: 25pt)
    /// Totally
    ///
    /// #set text(kerning: false)
    /// Totally
    /// ```
    #[default(true)]
    #[ghost]
    pub kerning: bool,

    /// スタイリスティックな異体字を適用するかどうか。
    ///
    /// フォントによっては、同じコードポイントに対する代替のグリフを持つことがあります。
    /// これを `{true}` に設定すると、OpenTypeの `salt` フォント機能を有効にすることで
    /// それらの異体字に切り替わります。
    ///
    /// ```example
    /// #set text(
    ///   font: "IBM Plex Sans",
    ///   size: 20pt,
    /// )
    ///
    /// 0, a, g, ß
    ///
    /// #set text(alternates: true)
    /// 0, a, g, ß
    /// ```
    #[default(false)]
    #[ghost]
    pub alternates: bool,

    /// 適用するスタイリスティックセット。フォントデザイナーは代替のグリフ形状を
    /// スタイリスティックセットに分類できます。この値はフォント固有であるため、どの
    /// セットが利用可能かを知るためには使用するフォントを確認する必要があります。
    ///
    /// 整数または整数の配列を設定でき、それぞれ `{1}` から `{20}` の範囲でなければ
    /// なりません。これにより、`ss01` から `ss20` に対応するOpenType機能が有効化
    /// されます。これを `{none}` に設定すると、全てのスタイリスティックセットが
    /// 無効化されます。
    ///
    /// ```example
    /// #set text(font: "IBM Plex Serif")
    /// ß vs #text(stylistic-set: 5)[ß] \
    /// 10 years ago vs #text(stylistic-set: (1, 2, 3))[10 years ago]
    /// ```
    #[ghost]
    pub stylistic_set: StylisticSets,

    /// 標準的な合字（リガチャ）を有効にするかどうか。
    ///
    /// 「fi」のような特定の文字の組み合わせは、しばしば _合字_ と呼ばれる単一の融合
    /// したグリフとして表示されます。これを `{false}` に設定すると、OpenTypeの `liga`
    /// および `clig` フォント機能がオフになり、これらの合字が無効化されます。
    ///
    /// ```example
    /// #set text(size: 20pt)
    /// A fine ligature.
    ///
    /// #set text(ligatures: false)
    /// A fine ligature.
    /// ```
    #[default(true)]
    #[ghost]
    pub ligatures: bool,

    /// 控えめに使用すべき合字を有効にするかどうか。これを `{true}` に設定すると、
    /// OpenTypeの `dlig` フォント機能が有効化されます。
    #[default(false)]
    #[ghost]
    pub discretionary_ligatures: bool,

    /// 歴史的合字を有効にするかどうか。これを `{true}` に設定すると、OpenTypeの
    /// `hlig` フォント機能が有効化されます。
    #[default(false)]
    #[ghost]
    pub historical_ligatures: bool,

    /// どの種類の数字を選択するか。`{auto}` に設定すると、フォントのデフォルトの
    /// 数字が使用されます。
    ///
    /// ```example
    /// #set text(font: "Noto Sans", 20pt)
    /// #set text(number-type: "lining")
    /// Number 9.
    ///
    /// #set text(number-type: "old-style")
    /// Number 9.
    /// ```
    #[ghost]
    pub number_type: Smart<NumberType>,

    /// 数字の幅。`{auto}` に設定すると、フォントのデフォルトの数字が使用されます。
    ///
    /// ```example
    /// #set text(font: "Noto Sans", 20pt)
    /// #set text(number-width: "proportional")
    /// A 12 B 34. \
    /// A 56 B 78.
    ///
    /// #set text(number-width: "tabular")
    /// A 12 B 34. \
    /// A 56 B 78.
    /// ```
    #[ghost]
    pub number_width: Smart<NumberWidth>,

    /// 0のグリフにスラッシュを入れるかどうか。これを `{true}` に設定すると、
    /// OpenTypeの `zero` フォント機能が有効化されます。
    ///
    /// ```example
    /// 0, #text(slashed-zero: true)[0]
    /// ```
    #[default(false)]
    #[ghost]
    pub slashed_zero: bool,

    /// 数字を分数に変換するかどうか。これを `{true}` に設定すると、OpenTypeの
    /// `frac` フォント機能が有効化されます。
    ///
    /// このプロパティをグローバルに有効化することは推奨されません。スラッシュの後の
    /// 数字の表示が（例えばURL内などでも）全て影響を受けてしまうためです。代わりに、
    /// 分数表示が必要な箇所で局所的に有効化してください。
    ///
    /// ```example
    /// 1/2 \
    /// #text(fractions: true)[1/2]
    /// ```
    #[default(false)]
    #[ghost]
    pub fractions: bool,

    /// 適用する生のOpenType機能。
    ///
    /// - 文字列の配列が与えられた場合、文字列によって識別される機能を `{1}` に設定
    ///   します。
    /// - 数値へのマッピングを表す辞書が与えられた場合、キーによって識別される機能を
    ///   その値に設定します。
    ///
    /// ```example
    /// // Enable the `frac` feature manually.
    /// #set text(features: ("frac",))
    /// 1/2
    /// ```
    #[fold]
    #[ghost]
    pub features: FontFeatures,

    /// 全ての文章が他の引数に従ってスタイル設定されるコンテンツ。
    #[external]
    #[required]
    pub body: Content,

    /// 文章。
    #[required]
    pub text: EcoString,

    /// この要素のspanが参照するテキスト構文ノードにおける文章のオフセット。
    #[internal]
    #[ghost]
    pub span_offset: usize,

    /// フォントウェイトに適用するデルタ。
    #[internal]
    #[fold]
    #[ghost]
    pub delta: WeightDelta,

    /// フォントスタイルを反転すべきかどうか。
    #[internal]
    #[fold]
    #[default(ItalicToggle(false))]
    #[ghost]
    pub emph: ItalicToggle,

    /// 装飾用の線。
    #[internal]
    #[fold]
    #[ghost]
    pub deco: SmallVec<[Decoration; 1]>,

    /// 文章に適用すべき大文字小文字の変換。
    #[internal]
    #[ghost]
    pub case: Option<Case>,

    /// スモールキャピタルのグリフを使用すべきかどうか。（"smcp"、"c2sc"）
    #[internal]
    #[ghost]
    pub smallcaps: Option<Smallcaps>,

    /// 上付き文字または下付き文字のいずれかが有効な場合の、それらの設定。
    #[internal]
    #[ghost]
    pub shift_settings: Option<ShiftSettings>,
}

impl TextElem {
    /// 新しいパックされた text 要素を作成します。
    pub fn packed(text: impl Into<EcoString>) -> Content {
        Self::new(text.into()).pack()
    }
}

impl Debug for TextElem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Text({})", self.text)
    }
}

impl Repr for TextElem {
    fn repr(&self) -> EcoString {
        eco_format!("[{}]", self.text)
    }
}

impl Construct for TextElem {
    fn construct(engine: &mut Engine, args: &mut Args) -> SourceResult<Content> {
        // text コンストラクターは特別である：text 要素を作成しない。
        // 代わりに、渡された引数を構造的には変更せず、その中の全ての文章にスタイルを適用する。
        let styles = Self::set(engine, args)?;
        let body = args.expect::<Content>("body")?;
        Ok(body.styled_with_map(styles))
    }
}

impl PlainText for Packed<TextElem> {
    fn plain_text(&self, text: &mut EcoString) {
        text.push_str(&self.text);
    }
}

/// "arial" のような小文字化されたフォントファミリー。
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FontFamily {
    // フォントファミリーの名前
    name: EcoString,
    // フォントがサポートするUnicodeコードポイントを定義する正規表現。
    covers: Option<Covers>,
}

impl FontFamily {
    /// 名前付きのフォントファミリー異体を作成します。
    pub fn new(string: &str) -> Self {
        Self::with_coverage(string, None)
    }

    /// 名前と任意のUnicodeカバレッジから、フォントファミリーを作成します。
    pub fn with_coverage(string: &str, covers: Option<Covers>) -> Self {
        Self { name: string.to_lowercase().into(), covers }
    }

    /// 小文字化されたファミリー名。
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// ユーザーが設定したフォントファミリーのカバレッジ。
    pub fn covers(&self) -> Option<&Regex> {
        self.covers.as_ref().map(|covers| covers.as_regex())
    }
}

cast! {
    FontFamily,
    self => self.name.into_value(),
    string: EcoString => Self::new(&string),
    mut v: Dict => {
        let ret = Self::with_coverage(
            &v.take("name")?.cast::<EcoString>()?,
            v.take("covers").ok().map(|v| v.cast()).transpose()?
        );
        v.finish(&["name", "covers"])?;
        ret
    },
}

/// フォントファミリーがどのコードポイントに使用されるかを定義します。
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Covers {
    /// ラテンフォントとCJKフォントの双方で使用されるものを除く、全てのコードポイントをカバーします。
    LatinInCjk,
    /// 正規表現に一致するコードポイントの集合をカバーします。
    Regex(Regex),
}

impl Covers {
    /// カバレッジを表す正規表現を取得します。
    pub fn as_regex(&self) -> &Regex {
        match self {
            Self::LatinInCjk => singleton!(
                Regex,
                Regex::new(
                    "[^\u{00B7}\u{2013}\u{2014}\u{2018}\u{2019}\
                       \u{201C}\u{201D}\u{2025}-\u{2027}\u{2E3A}]"
                )
                .unwrap()
            ),
            Self::Regex(regex) => regex,
        }
    }
}

cast! {
    Covers,
    self => match self {
        Self::LatinInCjk => "latin-in-cjk".into_value(),
        Self::Regex(regex) => regex.into_value(),
    },

    /// ラテンフォントとCJKフォントの双方で使用されるものを除く、全てのコードポイントをカバーします。
    "latin-in-cjk" => Covers::LatinInCjk,

    regex: Regex => {
        let ast = regex_syntax::ast::parse::Parser::new().parse(regex.as_str());
        match ast {
            Ok(
                regex_syntax::ast::Ast::ClassBracketed(..)
                | regex_syntax::ast::Ast::ClassUnicode(..)
                | regex_syntax::ast::Ast::ClassPerl(..)
                | regex_syntax::ast::Ast::Dot(..)
                | regex_syntax::ast::Ast::Literal(..),
            ) => {}
            _ => bail!(
                "coverage regex may only use dot, letters, and character classes";
                hint: "the regex is applied to each letter individually"
            ),
        }
        Covers::Regex(regex)
    },
}

/// フォントファミリーのフォールバックリスト。
///
/// 少なくとも1つのフォントを含まなければなりません。
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct FontList(pub Vec<FontFamily>);

impl FontList {
    pub fn new(fonts: Vec<FontFamily>) -> StrResult<Self> {
        if fonts.is_empty() {
            bail!("font fallback list must not be empty")
        } else {
            Ok(Self(fonts))
        }
    }
}

impl<'a> IntoIterator for &'a FontList {
    type IntoIter = std::slice::Iter<'a, FontFamily>;
    type Item = &'a FontFamily;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

cast! {
    FontList,
    self => if self.0.len() == 1 {
        self.0.into_iter().next().unwrap().name.into_value()
    } else {
        self.0.into_value()
    },
    family: FontFamily => Self(vec![family]),
    values: Array => Self::new(values.into_iter().map(|v| v.cast()).collect::<HintedStrResult<_>>()?)?,
}

/// フォントファミリーに対する優先順位付けされたイテレーターを解決します。
pub fn families(styles: StyleChain<'_>) -> impl Iterator<Item = &'_ FontFamily> + Clone {
    let fallbacks = singleton!(Vec<FontFamily>, {
        [
            "libertinus serif",
            "twitter color emoji",
            "noto color emoji",
            "apple color emoji",
            "segoe ui emoji",
        ]
        .into_iter()
        .map(FontFamily::new)
        .collect()
    });

    let tail = if styles.get(TextElem::fallback) { fallbacks.as_slice() } else { &[] };
    styles.get_ref(TextElem::font).into_iter().chain(tail.iter())
}

/// フォント異体を解決します。
pub fn variant(styles: StyleChain) -> FontVariant {
    let mut variant = FontVariant::new(
        styles.get(TextElem::style),
        styles.get(TextElem::weight),
        styles.get(TextElem::stretch),
    );

    let WeightDelta(delta) = styles.get(TextElem::delta);
    variant.weight = variant
        .weight
        .thicken(delta.clamp(i16::MIN as i64, i16::MAX as i64) as i16);

    if styles.get(TextElem::emph).0 {
        variant.style = match variant.style {
            FontStyle::Normal => FontStyle::Italic,
            FontStyle::Italic => FontStyle::Normal,
            FontStyle::Oblique => FontStyle::Normal,
        }
    }

    variant
}

/// 文章のサイズ。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextSize(pub Length);

impl Fold for TextSize {
    fn fold(self, outer: Self) -> Self {
        // 2つの線形関数を掛け合わせる。
        Self(Length {
            em: Em::new(self.0.em.get() * outer.0.em.get()),
            abs: self.0.em.get() * outer.0.abs + self.0.abs,
        })
    }
}

impl Resolve for TextSize {
    type Output = Abs;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        let factor = match styles.get(EquationElem::size) {
            MathSize::Display | MathSize::Text => 1.0,
            MathSize::Script => styles.get(EquationElem::script_scale).0 as f64 / 100.0,
            MathSize::ScriptScript => {
                styles.get(EquationElem::script_scale).1 as f64 / 100.0
            }
        };
        factor * self.0.resolve(styles)
    }
}

cast! {
    TextSize,
    self => self.0.into_value(),
    v: Length => Self(v),
}

/// 文章の上端を指定します。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TopEdge {
    /// フォントのメトリクスまたはバウンディングボックスを介して指定される端。
    Metric(TopEdgeMetric),
    /// 長さとして指定される端。
    Length(Length),
}

cast! {
    TopEdge,
    self => match self {
        Self::Metric(metric) => metric.into_value(),
        Self::Length(length) => length.into_value(),
    },
    v: TopEdgeMetric => Self::Metric(v),
    v: Length => Self::Length(v),
}

/// 文章の上端を表すメトリクス。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum TopEdgeMetric {
    /// フォントアセンダー。通常、これは全ての字形の高さを超えます。
    Ascender,
    /// 大文字の高さの近似値。
    CapHeight,
    /// アセンダーを持たない小文字の高さの近似値。
    XHeight,
    /// 文字が置かれるベースライン。
    Baseline,
    /// 字形が持つバウンディングボックスの上端。
    Bounds,
}

impl TryInto<VerticalFontMetric> for TopEdgeMetric {
    type Error = ();

    fn try_into(self) -> Result<VerticalFontMetric, Self::Error> {
        match self {
            Self::Ascender => Ok(VerticalFontMetric::Ascender),
            Self::CapHeight => Ok(VerticalFontMetric::CapHeight),
            Self::XHeight => Ok(VerticalFontMetric::XHeight),
            Self::Baseline => Ok(VerticalFontMetric::Baseline),
            _ => Err(()),
        }
    }
}

/// 文章の下端を指定します。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BottomEdge {
    /// フォントのメトリクスまたはバウンディングボックスを介して指定される端。
    Metric(BottomEdgeMetric),
    /// 長さとして指定される端。
    Length(Length),
}

cast! {
    BottomEdge,
    self => match self {
        Self::Metric(metric) => metric.into_value(),
        Self::Length(length) => length.into_value(),
    },
    v: BottomEdgeMetric => Self::Metric(v),
    v: Length => Self::Length(v),
}

/// 文章の下端を表すメトリクス。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum BottomEdgeMetric {
    /// 文字が置かれるベースライン。
    Baseline,
    /// フォントディセンダー。通常、これは全ての字形の深さを超えます。
    Descender,
    /// 字形が持つバウンディングボックスの下端。
    Bounds,
}

impl TryInto<VerticalFontMetric> for BottomEdgeMetric {
    type Error = ();

    fn try_into(self) -> Result<VerticalFontMetric, Self::Error> {
        match self {
            Self::Baseline => Ok(VerticalFontMetric::Baseline),
            Self::Descender => Ok(VerticalFontMetric::Descender),
            _ => Err(()),
        }
    }
}

/// 行内における文章とインライン要素の方向。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextDir(pub Smart<Dir>);

cast! {
    TextDir,
    self => self.0.into_value(),
    v: Smart<Dir> => {
        if v.is_custom_and(|dir| dir.axis() == Axis::Y) {
            bail!("text direction must be horizontal");
        }
        Self(v)
    },
}

impl Resolve for TextDir {
    type Output = Dir;

    fn resolve(self, styles: StyleChain) -> Self::Output {
        match self.0 {
            Smart::Auto => styles.get(TextElem::lang).dir(),
            Smart::Custom(dir) => dir,
        }
    }
}

/// 有効化するスタイリスティックセットの集合。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct StylisticSets(u32);

impl StylisticSets {
    /// この集合をTypstの値の配列に変換します。
    pub fn into_array(self) -> Array {
        self.sets().map(IntoValue::into_value).collect()
    }

    /// この集合が特定のスタイリスティックセットを含むかどうかを返します。
    pub fn has(self, ss: u8) -> bool {
        self.0 & (1 << (ss as u32)) != 0
    }

    /// 有効化する全てのスタイリスティックセットを反復するイテレーターを返します。
    pub fn sets(self) -> impl Iterator<Item = u8> {
        (1..=20).filter(move |i| self.has(*i))
    }
}

cast! {
    StylisticSets,
    self => self.into_array().into_value(),
    _: NoneValue => Self(0),
    v: i64 => match v {
        1 ..= 20 => Self(1 << (v as u32)),
        _ => bail!("stylistic set must be between 1 and 20"),
    },
    v: Vec<i64> => {
        let mut flags = 0;
        for i in v {
            match i {
                1 ..= 20 => flags |= 1 << (i as u32),
                _ => bail!("stylistic set must be between 1 and 20"),
            }
        }
        Self(flags)
    },
}

/// どの種類の数字を選択するか。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum NumberType {
    /// 大文字の文章になじむ数字（OpenTypeの `lnum` フォント機能）。
    Lining,
    /// 大文字と小文字が混在した文章の流れになじむ数字（OpenTypeの `onum` フォント機能）。
    OldStyle,
}

/// 数字の幅。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum NumberWidth {
    /// グリフごとに異なる幅を持つ数字（OpenTypeの `pnum` フォント機能）。
    Proportional,
    /// 等幅の数字（OpenTypeの `tnum` フォント機能）。
    Tabular,
}

/// OpenTypeフォント機能の設定。
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct FontFeatures(pub Vec<(Tag, u32)>);

cast! {
    FontFeatures,
    self => self.0
        .into_iter()
        .map(|(tag, num)| {
            let bytes = tag.to_bytes();
            let key = std::str::from_utf8(&bytes).unwrap_or_default();
            (key.into(), num.into_value())
        })
        .collect::<Dict>()
        .into_value(),
    values: Array => Self(values
        .into_iter()
        .map(|v| {
            let tag = v.cast::<EcoString>()?;
            Ok((Tag::from_bytes_lossy(tag.as_bytes()), 1))
        })
        .collect::<HintedStrResult<_>>()?),
    values: Dict => Self(values
        .into_iter()
        .map(|(k, v)| {
            let num = v.cast::<u32>()?;
            let tag = Tag::from_bytes_lossy(k.as_bytes());
            Ok((tag, num))
        })
        .collect::<HintedStrResult<_>>()?),
}

impl Fold for FontFeatures {
    fn fold(self, outer: Self) -> Self {
        Self(self.0.fold(outer.0))
    }
}

/// 適用するOpenType機能を収集します。
pub fn features(styles: StyleChain) -> Vec<Feature> {
    let mut tags = vec![];
    let mut feat = |tag: &[u8; 4], value: u32| {
        tags.push(Feature::new(Tag::from_bytes(tag), value, ..));
    };

    // Harfbuzzでデフォルトで有効な機能は、無効化された場合にのみ追加する。
    if !styles.get(TextElem::kerning) {
        feat(b"kern", 0);
    }

    // Harfbuzzでデフォルトで無効な機能は、有効化された場合にのみ追加する。
    if let Some(sc) = styles.get(TextElem::smallcaps) {
        feat(b"smcp", 1);
        if sc == Smallcaps::All {
            feat(b"c2sc", 1);
        }
    }

    if styles.get(TextElem::alternates) {
        feat(b"salt", 1);
    }

    for set in styles.get(TextElem::stylistic_set).sets() {
        let storage = [b's', b's', b'0' + set / 10, b'0' + set % 10];
        feat(&storage, 1);
    }

    if !styles.get(TextElem::ligatures) {
        feat(b"liga", 0);
        feat(b"clig", 0);
    }

    if styles.get(TextElem::discretionary_ligatures) {
        feat(b"dlig", 1);
    }

    if styles.get(TextElem::historical_ligatures) {
        feat(b"hlig", 1);
    }

    match styles.get(TextElem::number_type) {
        Smart::Auto => {}
        Smart::Custom(NumberType::Lining) => feat(b"lnum", 1),
        Smart::Custom(NumberType::OldStyle) => feat(b"onum", 1),
    }

    match styles.get(TextElem::number_width) {
        Smart::Auto => {}
        Smart::Custom(NumberWidth::Proportional) => feat(b"pnum", 1),
        Smart::Custom(NumberWidth::Tabular) => feat(b"tnum", 1),
    }

    if styles.get(TextElem::slashed_zero) {
        feat(b"zero", 1);
    }

    if styles.get(TextElem::fractions) {
        feat(b"frac", 1);
    }

    match styles.get(EquationElem::size) {
        MathSize::Script => feat(b"ssty", 1),
        MathSize::ScriptScript => feat(b"ssty", 2),
        _ => {}
    }

    for (tag, value) in styles.get_cloned(TextElem::features).0 {
        tags.push(Feature::new(tag, value, ..))
    }

    tags
}

/// スタイルチェインの言語と地域を、rustybuzz互換のBCP 47言語に処理します。
pub fn language(styles: StyleChain) -> rustybuzz::Language {
    let mut bcp: EcoString = styles.get(TextElem::lang).as_str().into();
    if let Some(region) = styles.get(TextElem::region) {
        bcp.push('-');
        bcp.push_str(region.as_str());
    }
    rustybuzz::Language::from_str(&bcp).unwrap()
}

/// foldされるとオンとオフを交互に切り替えるトグル。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ItalicToggle(pub bool);

impl Fold for ItalicToggle {
    fn fold(self, outer: Self) -> Self {
        Self(self.0 ^ outer.0)
    }
}

/// foldされると合算されるデルタ。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WeightDelta(pub i64);

impl Fold for WeightDelta {
    fn fold(self, outer: Self) -> Self {
        Self(outer.0 + self.0)
    }
}

/// さまざまなレイアウトの選択肢に対するコスト。
///
/// foldされると、コストは（後の値を優先して）更新されます。
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct Costs {
    hyphenation: Option<Ratio>,
    runt: Option<Ratio>,
    widow: Option<Ratio>,
    orphan: Option<Ratio>,
}

impl Costs {
    #[must_use]
    pub fn hyphenation(&self) -> Ratio {
        self.hyphenation.unwrap_or(Ratio::one())
    }

    #[must_use]
    pub fn runt(&self) -> Ratio {
        self.runt.unwrap_or(Ratio::one())
    }

    #[must_use]
    pub fn widow(&self) -> Ratio {
        self.widow.unwrap_or(Ratio::one())
    }

    #[must_use]
    pub fn orphan(&self) -> Ratio {
        self.orphan.unwrap_or(Ratio::one())
    }
}

impl Fold for Costs {
    #[inline]
    fn fold(self, outer: Self) -> Self {
        Self {
            hyphenation: self.hyphenation.or(outer.hyphenation),
            runt: self.runt.or(outer.runt),
            widow: self.widow.or(outer.widow),
            orphan: self.orphan.or(outer.orphan),
        }
    }
}

cast! {
    Costs,
    self => dict![
        "hyphenation" => self.hyphenation(),
        "runt" => self.runt(),
        "widow" => self.widow(),
        "orphan" => self.orphan(),
    ].into_value(),
    mut v: Dict => {
        let ret = Self {
            hyphenation: v.take("hyphenation").ok().map(|v| v.cast()).transpose()?,
            runt: v.take("runt").ok().map(|v| v.cast()).transpose()?,
            widow: v.take("widow").ok().map(|v| v.cast()).transpose()?,
            orphan: v.take("orphan").ok().map(|v| v.cast()).transpose()?,
        };
        v.finish(&["hyphenation", "runt", "widow", "orphan"])?;
        ret
    },
}

/// コードポイントがUnicodeの `Default_Ignorable` であるかどうか。
pub fn is_default_ignorable(c: char) -> bool {
    /// Unicodeのdefault ignorableの集合。
    static DEFAULT_IGNORABLE_DATA: LazyLock<CodePointSetData> = LazyLock::new(|| {
        icu_properties::sets::load_default_ignorable_code_point(
            &BlobDataProvider::try_new_from_static_blob(typst_assets::icu::ICU)
                .unwrap()
                .as_deserializing(),
        )
        .unwrap()
    });
    DEFAULT_IGNORABLE_DATA.as_borrowed().contains(c)
}

/// 利用できないフォントファミリーをチェックします。
fn check_font_list(engine: &mut Engine, list: &Spanned<FontList>) {
    let book = engine.world.book();
    for family in &list.v {
        match book.select_family(family.as_str()).next() {
            Some(index) => {
                if book
                    .info(index)
                    .is_some_and(|x| x.flags.contains(FontFlags::VARIABLE))
                {
                    engine.sink.warn(warning!(
                        list.span,
                        "variable fonts are not currently supported and may render incorrectly";
                        hint: "try installing a static version of \"{}\" instead", family.as_str()
                    ))
                }
            }
            None => engine.sink.warn(warning!(
                list.span,
                "unknown font family: {}",
                family.as_str(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_elem_size() {
        assert_eq!(std::mem::size_of::<TextElem>(), std::mem::size_of::<EcoString>());
    }
}
