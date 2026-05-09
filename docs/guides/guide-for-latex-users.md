---
description: |
  あなたはLaTeXユーザーですか？ このガイドではTypstとLaTeXの違いや類似点を説明し、すぐにTypstを使い始められるよう手助けをします。
---

# LaTeXユーザー向けガイド { # }
このページは、これまでLaTeXを使ってきた方がTypstを試してみたいという場合に最適な出発点です。ここでは、これら2つのシステムの主な違いをユーザーの視点から見ていきます。TypstはLaTeXの上に構築されたものではなく、構文も異なりますが、あなたのLaTeXのスキルを活かして素早く使い始めるための方法を学べます。

LaTeXと同様に、Typstはマークアップベースの組版システムです。すなわち、テキストファイルに文書を書き、コマンドやその他の構文で印付けを行い、コンパイラーを使ってソースファイルをPDFに組版します。しかし、Typstはいくつかの点でLaTeXとは異なります。1つは、Typstは（Markdownでおなじみのように）共通的なタスクに対してより専用の構文を採用していることです。Typstのコマンドはより一貫性があり、全てが同じ仕組みで動作します。そのため、LaTeXのようにパッケージごとに異なる流儀を覚える必要はなく、いくつかの一般的な概念を理解するだけで済みます。さらに、TypstはLaTeXより高速にコンパイルできます。コンパイルにかかる時間は通常、秒単位ではなくミリ秒単位です。そのため、Webアプリでもコンパイラーでも、即座にプレビューを提供できます。

以下では、LaTeXからTypstに移行するユーザーがTypstで文書を書く際によく抱く質問のいくつかを取り上げます。Typstの段階的な入門をお望みの場合は、[チュートリアル]($tutorial)をご覧ください。

## インストール
Typstを使うには2つの方法があります。1つは[Webアプリ](https://typst.app/signup/)を利用する方法、もう1つは[コンパイラーをインストール](https://github.com/typst/typst/releases)してご自身のコンピューターで使う方法です。Webアプリを使う場合は、必要なものが全て揃った共同編集エディタが提供され、Typstがブラウザー上で動作するため、インストールは不要です。

代わりにご自身のコンピューターでTypstを使うことを選んだ場合は、コンパイラーを単一の小さなバイナリとしてダウンロードできます。これはどんなユーザーでも実行可能で、root権限は必要ありません。TeX LiveのようなLaTeXディストリビューションとは異なり、パッケージは初めて使用するときにダウンロードされ、ローカルにキャッシュされるため、Typstのインストールを軽量に保てます。ローカルのコンパイラーを使うことで、お好みのエディターを使用したり、ファイルの保存場所を自分で決めたりできます。

## 新しい空の文書を作成するにはどうすればよいですか？ { #getting-started }
それは簡単です。新しい空のテキストファイル（拡張子は`.typ`）を作成するだけです。始めるためのボイラープレートは必要ありません。文章を書き始めるだけで、空のA4サイズのページに配置されます。Webアプリを利用している場合は、「+ Empty document」をクリックすることで、ファイル付きの新規プロジェクトを作成し、エディターに入れます。[段落区切り]($parbreak)はLaTeXと同じように動作します。空行を1つ入れるだけです。

```example
Hey there!

Here are two paragraphs. The
output is shown to the right.
```

代わりに既存のLaTeX文書から始めたい場合は、[Pandoc](https://pandoc.org)を使うことで、ソースコードをTypstマークアップに変換できます。この変換機能はWebアプリにも組み込まれているため、`.tex`ファイルをアップロードしてTypstでプロジェクトを始められます。

## セクション見出しや強調などを作成するにはどうすればよいですか？ { #elements }
LaTeXでは、`\section`コマンドを使ってセクション見出しを作成します。入れ子の見出しは`\subsection`、`\subsubsection`などで示します。文書クラスによっては、`\part`や`\chapter`もあります。

Typstでは、[見出し]($heading)はより簡潔に書けます。見出しのある行の先頭に等号とスペースを付けると、第1階層の見出しになります（`[= Introduction]`）。第2階層の見出しが必要なら、等号を2つ使います（`[== In this paper]`）。等号を増やすことで、好きなだけ深く見出しを入れ子にできます。

強調（通常はイタリック体で表示される）はテキストを`[_underscores_]`で囲んで表します。強い強調（通常はボールド体で表示される）は代わりに`[*stars*]`を使います。

以下は、LaTeXで使われる代表的なマークアップコマンドと、それに対応するTypstの記法の一覧です。[構文チートシート]($syntax)もあわせてご覧ください。

| 要素                   | LaTeX                     | Typst                  | 参照       |
|:-----------------------|:--------------------------|:-----------------------|:-----------|
| 強い強調               | `\textbf{strong}`         | `[*strong*]`           | [`strong`] |
| 強調                   | `\emph{emphasis}`         | `[_emphasis_]`         | [`emph`]   |
| リンク                 | `\url{https://typst.app}` | `[https://typst.app/]` | [`link`]   |
| ラベル                 | `\label{intro}`           | `[<intro>]`            | [`label`]  |
| 参照                   | `\ref{intro}`             | `[@intro]`             | [`ref`]    |
| 引用                   | `\cite{humphrey97}`       | `[@humphrey97]`        | [`cite`]   |
| 等幅（タイプライター） | `\texttt{mono}` | `text`または`mono`関数 | [`text`], [`mono`]($math.mono) |
| コード                 | `lstlisting`環境          | ``[`print(f"{x}")`]``  | [`raw`]  |
| 逐語的表示             | `verbatim`環境            | ``[`#typst-code()`]``  | [`raw`]  |
| 箇条書きリスト         | `itemize`環境             | `[- List]`             | [`list`]   |
| 番号付きリスト         | `enumerate`環境           | `[+ List]`             | [`enum`]   |
| 用語リスト             | `description`環境         | `[/ Term: List]`       | [`terms`]  |
| 図表                   | `figure`環境              | `figure`関数           | [`figure`] |
| 表                     | `table`環境               | `table`関数            | [`table`]  |
| 数式                   | `$x$`、`align`/`equation`環境 | `[$x$]`、`[$ x = y $]` | [`equation`]($math.equation) |

[リスト]($list)はTypstでは環境に依存しません。代わりに、見出しのような軽量な構文を持っています。順序なしリスト（`itemize`）を作成するには、各項目の行の先頭にハイフンを付けます。

````example
To write this list in Typst...

```latex
\begin{itemize}
  \item Fast
  \item Flexible
  \item Intuitive
\end{itemize}
```

...just type this:

- Fast
- Flexible
- Intuitive

````

リストの入れ子は、適切なインデントを行うだけで実現できます。項目の間に空行を入れると、より[広い行間]($list.tight)のリストになります。

[番号付きリスト]($enum)（`enumerate`）にしたい場合は、ハイフンの代わりに`+`を使います。[用語リスト]($terms)（`description`）にしたい場合は、代わりに`[/ Term: Description]`と書きます。

[`raw`関数]($raw)および記法（例えば``[`raw`]``）は、逐語的（書式なし）テキストに対してのみ動作することに注意してください。書式が必要な場合は、以下の例のように[`text`関数]($text)に等幅フォントを指定して使用できます。

```example
#text(
  font: "DejaVu Sans Mono",
  size: 0.8em,
)[monospace *bold*]
```

## コマンドの使い方は？ { #commands }
LaTeXは（バックスラッシュで始まる）コマンドに大きく依存しています。これらの_マクロ_を使うことで、組版処理に影響を与えたり、コンテンツを挿入・操作したりします。一部のコマンドは引数を受け取り、その引数は通常、波括弧で囲まれます（例：`\cite{rasmus}`）。

Typstは[マークアップモードとコードモード]($scripting/#blocks)を区別します。デフォルトはマークアップモードで、テキストを書いたり、`[*stars for bold text*]`のような構文構造を適用したりします。一方、コードモードはPythonのようなプログラミング言語に類似していて、コードを入力・実行するためのものです。

Typstのマークアップ内では、ハッシュ（`#`）を使うことで、単一のコマンド（より厳密には_式_）に対してコードモードに切り替えられます。これは、プロジェクトを複数の[ファイル]($scripting/#modules)に分割したり、ある[条件]($scripting/#conditionals)に基づいてテキストを描画したりするために、関数を呼び出す方法です。コードモード内では、角括弧を使うことで通常のマークアップによる[_コンテンツ_]($content)を含められます。コードモード内では、このコンテンツは変数の他の通常の値と全く同じように扱われます。

```example
First, a rectangle:
#rect()

Let me show how to do
#underline([_underlined_ text])

We can also do some maths:
#calc.max(3, 2 * 4)

And finally a little loop:
#for x in range(3) [
  Hi #x.
]
```

関数呼び出しは常に、関数名（[`rect`]、[`underline`]、[`calc.max`]($calc.max)、[`range`]($array.range)）の後に括弧が続きます（マクロが引数を必要としない場合に角括弧や波括弧が省略可能なLaTeXとは対照的です）。これらの括弧の中で渡される引数のリストは、具体的な関数によって異なり、[リファレンス]($reference)で規定されています。

### 引数
1つの関数は複数の引数を持てます。引数の中には位置引数があり、これは値を渡すだけです。例えば、`[#lower("SCREAM")]`関数は引数を全て小文字にして返します。多くの関数では、可読性を高めるために位置引数の代わりに名前付き引数を使用します。例えば、矩形の寸法とストロークは名前付き引数で定義されます。

```example
#rect(
  width: 2cm,
  height: 1cm,
  stroke: red,
)
```

名前付き引数を指定するには、まずその名前（上記では`width`、`height`、`stroke`）を入力し、続いてコロン、その後に値（`2cm`、`1cm`、`red`）を続けます。利用可能な名前付き引数は、各関数の[リファレンスページ]($reference)、または入力時の自動補完パネルで確認できます。名前付き引数は、一部のLaTeX環境を設定する方法と似ています。例えば、ラベルが`a)`、`b)`などのリストを開始するには、`\begin{enumerate}[label={\alph*)}]`と書くでしょう。

しばしば、関数に何らかの[コンテンツ]($content)を渡したい場合があります。例えば、LaTeXのコマンド`\underline{Alternative A}`はTypstでは`[#underline([Alternative A])]`に変換されます。角括弧は値が[コンテンツ]($content)であることを示します。これらの括弧の中では、通常のマークアップを使用できます。しかし、これは非常に単純な構造の割には括弧が多すぎます。そのため、末尾のコンテンツ引数を括弧の後ろに移動することもでき（その結果、括弧が空になる場合は省略可能です）、これにより構造を簡潔にできます。

```example
Typst is an #underline[alternative]
to LaTeX.

#rect(fill: aqua)[Get started here!]
```

### データ型
おそらくお気づきのように、引数にはそれぞれ特有のデータ型があります。Typstは多くの[データ型]($type)をサポートしています。以下に、最も重要なものといくつかとその書き方を表にまとめます。これらの型の値を指定するには、コードモードでなければなりません！

| データ型                        | 例                                |
|:--------------------------------|:----------------------------------|
| [コンテンツ]($content)          | `{[*fast* typesetting]}`          |
| [文字列]($str)                  | `{"Pietro S. Author"}`            |
| [整数]($int)                    | `{23}`                            |
| [浮動小数点数]($float)          | `{1.459}`                         |
| [絶対長]($length)               | `{12pt}`、`{5in}`、`{0.3cm}`など  |
| [相対長]($ratio)                | `{65%}`                           |

コンテンツと文字列の違いは、コンテンツは関数呼び出しを含むマークアップを含めることができるのに対し、文字列は単なる文字の連なりに過ぎないという点です。

Typstは[制御構造]($scripting/#conditionals)や、加算のための`+`や、2つの変数の等価性を確認するための`==`といった[演算子]($scripting/#operators)を提供しています。

関数を含む値を、独自の[変数]($scripting/#bindings)に格納することもできます。これは、それらに対して計算を行ったり、再利用可能な自動化を作成したり、値を複数回参照したりするのに便利です。変数の束縛はletキーワードを使って行い、これは`\newcommand`と同様の動作をします。

```example
// Store the integer `5`.
#let five = 5

// Define a function that
// increments a value.
#let inc(i) = i + 1

// Reference the variables.
I have #five fingers.

If I had one more, I'd have
#inc(five) fingers. Whoa!
```

### 文書の以降に影響を与えるコマンド { #rules }
LaTeXでは、`\textbf{bold text}`のように波括弧の中の引数を受け取り、その引数のみに影響を与えるコマンドがあります。一方、`\bfseries bold text`のようなコマンドはスイッチ（LaTeXではこれを宣言と呼びます）として機能し、文書または現在のスコープ内の以降のコンテンツ全ての見た目を変更します。

Typstでは、同じ関数を、文書の残り、ブロック（またはスコープ）、もしくはその引数のみに対して見た目を変えるために使用できます。例えば、`[#text(weight: "bold")[bold text]]`はその引数のみを太字にしますが、`[#set text(weight: "bold")]`は現在のブロックの終わり（あるいはなければ文書の終わり）まで全てのテキストを太字にします。関数の効果は、それが呼び出しとして使われているか[setルール]($styling/#set-rules)として使われているかから即座に分かります。

```example
I am starting out with small text.

#set text(14pt)

This is a bit #text(18pt)[larger,]
don't you think?
```

setルールは文書のどこにでも書けます。これは、対応する関数のデフォルトの引数値と捉えることができます。

```example
#set enum(numbering: "I.")

Good results can only be obtained by
+ following best practices
+ being aware of current results
  of other researchers
+ checking the data for biases
```

`+`は、上でsetルールを適用した[`{enum}`]($enum)関数の呼び出しに対する糖衣構文（略記法と捉えてください）です。[ほとんどの構文はこのように関数に紐付いています。]($syntax) 引数で実現できる以上に要素のスタイルを設定する必要がある場合は、[showルール]($styling/#show-rules)（`\renewcommand`にやや近い）を使用してその見た目を完全に再定義できます。

`\textbf`、`\textsf`、`\rmfamily`、`\mdseries`、`\itshape`のようなLaTeXコマンドの効果は、`text`関数の[`font`]($text.font)、[`style`]($text.style)、[`weight`]($text.weight)引数によって実現できます。`text`関数はsetルール（宣言スタイル）でもコンテンツ引数とともにでも使用できます。`\textsc`を置き換えるには、コンテンツ引数をスモールキャップとして描画する[`smallcaps`]($smallcaps)関数を使用できます。これを宣言スタイル（`\scshape`のように）で使いたい場合は、関数をスコープの残りに適用する[全体に適用するshowルール]($styling/#show-rules)を使用できます。

```example
#show: smallcaps

Boisterous Accusations
```

## 文書クラスを読み込むにはどうすればよいですか？ { #templates }
LaTeXでは、メインの`.tex`ファイルの先頭で`\documentclass{article}`コマンドを使い、文書の見た目を定義します。そのコマンドで、`article`を`report`や`amsart`などの別の値に置き換えることで、別の見た目を選択していたかもしれません。

Typstを使うときは、[関数]($function)で文書のスタイルを設定します。通常は、文書全体のスタイルを設定する関数を提供するテンプレートを使用します。まず、テンプレートファイルからその関数をインポートし、それを文書全体に適用します。これは、後続の文書を指定された関数で包む[showルール]($styling/#show-rules)で実現します。以下の例は、その動作を示しています。

```example:single
>>> #let conf(
>>>   title: none,
>>>   authors: (),
>>>   abstract: [],
>>>   doc,
>>> ) = {
>>>   set text(font: "Libertinus Serif", 11pt)
>>>   set par(justify: true)
>>>   set page(
>>>     "us-letter",
>>>     margin: auto,
>>>     header: align(
>>>       right + horizon,
>>>       title
>>>     ),
>>>     numbering: "1",
>>>     columns: 2
>>>   )
>>>
>>>   show heading.where(
>>>     level: 1
>>>   ): it => block(
>>>     align(center,
>>>       text(
>>>         13pt,
>>>         weight: "regular",
>>>         smallcaps(it.body),
>>>       )
>>>     ),
>>>   )
>>>   show heading.where(
>>>     level: 2
>>>   ): it => box(
>>>     text(
>>>       11pt,
>>>       weight: "regular",
>>>       style: "italic",
>>>       it.body + [.],
>>>     )
>>>   )
>>>
>>>   place(top, float: true, scope: "parent", {
>>>     set align(center)
>>>     text(17pt, title)
>>>
>>>     let count = calc.min(authors.len(), 3)
>>>     grid(
>>>       columns: (1fr,) * count,
>>>       row-gutter: 24pt,
>>>       ..authors.map(author => [
>>>         #author.name \
>>>         #author.affiliation \
>>>         #link("mailto:" + author.email)
>>>       ]),
>>>     )
>>>
>>>     par(justify: false)[
>>>       *Abstract* \
>>>       #abstract
>>>     ]
>>>   })
>>>
>>>   set align(left)
>>>   doc
>>> }
<<< #import "conf.typ": conf
#show: conf.with(
  title: [
    Towards Improved Modelling
  ],
  authors: (
    (
      name: "Theresa Tungsten",
      affiliation: "Artos Institute",
      email: "tung@artos.edu",
    ),
    (
      name: "Eugene Deklan",
      affiliation: "Honduras State",
      email: "e.deklan@hstate.hn",
    ),
  ),
  abstract: lorem(80),
)

Let's get started writing this
article by putting insightful
paragraphs right here!
>>> #lorem(500)
```

[`{import}`]($scripting/#modules)文によって、別のファイルの[関数]($function)（およびその他の定義）を利用できるようにします。この例では、`conf.typ`ファイルから`conf`関数をインポートしています。この関数は、文書を会議論文として整形します。showルールを使ってこの関数を文書に適用すると同時に、論文のメタデータも設定しています。showルールを適用した後は、すぐに論文を書き始められます！

Typst Universe（TypstにおけるCTANに相当するもの）のテンプレートを、`[#import "@preview/elsearticle:0.2.1": elsearticle]`のようなインポート文を使って利用することもできます。各テンプレートのドキュメントを確認して、そのテンプレート関数の名前を確認してください。Typst Universeのテンプレートやパッケージは、初めて使うときに自動的にダウンロードされます。

Webアプリでは、Typst Universeのテンプレートからプロジェクトを作成したり、テンプレートウィザードで自分のテンプレートを作成したりできます。ローカルでは、`typst init`CLIを使ってテンプレートから新規プロジェクトを作成できます。Typst Universeで公開されている[テンプレート一覧]($universe/search/?kind=templates)をご覧ください。Universeから入手できないコミュニティ製テンプレートを探すには、[`awesome-typst`リポジトリ](https://github.com/qjcg/awesome-typst)も参照できます。

[独自のカスタムテンプレートを作成する]($tutorial/making-a-template)こともできます。それらは対応するLaTeXの`.sty`ファイルよりも桁違いに短く、読みやすいので、ぜひ試してみてください！

<div class="info-box">

関数はTypstの「コマンド」であり、引数を変換して、文書_コンテンツ_を含む出力値を生成できます。関数は「純粋」であり、出力値や出力コンテンツを生成する以外の効果は持ちえないことを意味します。これは、文書に対して任意の効果を持ちうるLaTeXのマクロとは対照的です。

関数に文書全体のスタイルを設定させるために、showルールはそれより後にある全てを処理し、コロンの後に指定された関数をその結果を引数として呼び出します。`.with`部分は、`conf`関数を受け取り、その引数の一部を事前に設定してshowルールに渡す_メソッド_です。
</div>

## パッケージを読み込むにはどうすればよいですか？ { #packages }
Typstは「電池付き」（必要なものが最初から揃っている）であり、多くの代表的なLaTeXパッケージに相当する機能が組み込まれています。以下に、よく読み込まれるパッケージとそれに対応するTypstの関数を表にまとめました。

| LaTeXパッケージ                 | Typstでの代替                              |
|:--------------------------------|:-------------------------------------------|
| graphicx, svg                   | [`image`]関数                              |
| tabularx, tabularray            | [`table`]、[`grid`]関数                    |
| fontenc, inputenc, unicode-math | 書き始めるだけ！                           |
| babel, polyglossia              | [`text`]($text.lang)関数：`[#set text(lang: "zh")]` |
| amsmath                         | [数式モード]($category/math)               |
| amsfonts, amssymb               | [`sym`]($category/symbols)モジュールおよび[構文]($syntax/#math) |
| geometry, fancyhdr              | [`page`]関数                               |
| xcolor                          | [`text`]($text.fill)関数：`[#set text(fill: rgb("#0178A4"))]` |
| hyperref                        | [`link`]関数                               |
| bibtex, biblatex, natbib        | [`cite`]、[`bibliography`]関数             |
| lstlisting, minted              | [`raw`]関数および構文                      |
| parskip                         | [`block`]($block.spacing)および[`par`]($par.first-line-indent)関数 |
| csquotes                        | [`text`]($text.lang)言語を設定し、`["]`または`[']`を入力 |
| caption                         | [`figure`]関数                             |
| enumitem                        | [`list`]、[`enum`]、[`terms`]関数          |
| nicefrac                        | [`frac.style`]($math.frac.style)プロパティ |

_多くのもの_が組み込まれているとはいえ、全てが組み込まれているわけではありません。だからこそ、Typstには独自の[パッケージエコシステム]($universe)があり、コミュニティが自分たちの作品や自動化を共有しています。例えば、_CeTZ_パッケージを取り上げてみましょう。このパッケージを使うことで、複雑な図やプロットを作成できます。文書でCeTZを使うには、次のように書くだけです。

```typ
#import "@preview/cetz:0.4.1"
```

（`@preview`は、パッケージマネージャーがまだ初期かつ実験的な段階にある間に使用される_名前空間_です。これは将来的に置き換えられる予定です）

公式のパッケージハブとは別に、Typst向けに作成されたリソースを厳選して集めた[awesome-typstリポジトリ](https://github.com/qjcg/awesome-typst)も参照すると良いかもしれません。

例えばテンプレートを使うために、プロジェクト内の別のファイルから関数や変数を読み込む必要がある場合は、パッケージ指定の代わりにファイル名を指定して同じ[`import`]($scripting/#modules)文を使用できます。代わりに別のファイルのテキスト内容を含めるには、[`include`]($scripting/#modules)文を使用できます。これは指定したファイルの内容を取得して、文書に挿入します。

## 数式の入力方法は？ { #maths }
Typstで数式モードに入るには、数式をドル記号で囲むだけです。数式の内容と前後のドル記号の間にスペースや改行を入れることで、ディスプレイモードに入れます。

```example
The sum of the numbers from
$1$ to $n$ is:

$ sum_(k=1)^n k = (n(n+1))/2 $
```

[数式モード]($category/math)は通常のマークアップやコードモードとは異なる動作をします。数字や1文字の文字はそのまま表示されますが、複数の連続した（数字以外の）文字はTypstの変数として解釈されます。

Typstは数式モードで使える有用な変数を多数事前定義しています。全てのギリシャ文字（`alpha`、`beta`など）と一部のヘブライ文字（`alef`、`bet`など）は名前を通じて利用できます。一部の記号は、`<=`、`>=`、`->`のような略記でも追加で利用できます。

記号の全リストについては、[記号ページ]($reference/symbols)を参照してください。記号が見つからない場合は、[Unicodeエスケープシーケンス]($syntax/#escapes)を通じてアクセスすることもできます。

記号の代替形や関連形は、[ピリオドの後に修飾子を追加する]($symbol)ことで選択できる場合がよくあります。例えば、`arrow.l.squiggly`は左向きの波線矢印を挿入します。代わりに数式中に複数文字のテキストを挿入したい場合は、ダブルクオートで囲みます。

```example
$ delta "if" x <= 5 $
```

Typstでは、LaTeXで`\left`と`\right`コマンドが暗黙的に挿入されたかのように、区切り記号は数式に応じて自動的に拡大・縮小されます。区切り記号の挙動は[`lr`関数]($math.lr)を使ってカスタマイズできます。区切り記号のペアの拡大・縮小を防ぐには、バックスラッシュでエスケープできます。

Typstは、演算子の優先順位を保ちつつ、スラッシュ`/`で区切られた項を自動的に分数として組版します。分数によって冗長にならない丸括弧は全て出力に現れます。

```example
$ f(x) = (x + 1) / x $
```

[添え字（下付き・上付き）]($math.attach)はTypstとLaTeXで同じように動作します。`{$x^2$}`は上付き文字を生成し、`{$x_2$}`は下付き文字を生成します。下付き・上付き文字に複数の値を含めたい場合は、その内容を括弧で囲みます（例：`{$x_(a -> epsilon)$}`）。

数式モードの変数は（LaTeXのように`\`や）`#`を前置する必要がないため、これらの特殊文字なしで関数も呼び出せます。

```example
$ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $
```

上記の例では、[`cases`関数]($math.cases)を使ってfを記述しています。cases関数の中では、引数はカンマで区切られ、引数も数式として解釈されます。代わりに引数をTypstの値として解釈する必要がある場合は、`#`を前置します。

```example
$ (a + b)^2
  = a^2
  + text(fill: #maroon, 2 a b)
  + b^2 $
```

数式モード内では、Typstの全ての関数を使用でき、任意のコンテンツを挿入できます。引数リストでコードモードを使い、通常のように関数を動作させたい場合は、関数呼び出しの前に`#`を付けることで実現できます。これで、矩形や絵文字を変数として使うことを誰も止められません。

```example
$ sum^10_(🤓=1)
  #rect(width: 4mm, height: 2mm)/🤓
  = 🧠 maltese $
```

数式記号をUnicodeで直接入力したい場合は、それも可能です！

数式の呼び出しでは、`;`を区切り文字として使うことで2次元の引数リストを持てます。これの最も一般的な用途は、行列を作成する[`mat`関数]($math.mat)です。

```example
$ mat(
  1, 2, ..., 10;
  2, 2, ..., 10;
  dots.v, dots.v, dots.down, dots.v;
  10, 10, ..., 10;
) $
```

## 「LaTeXらしい見た目」にするにはどうすればよいですか？ { #latex-look }
LaTeXで組版された論文には特有の見た目があります。これは主に、フォントのComputer Modern、両端揃え、狭い行間、広い余白によるものです。

以下の例は、
- [余白]($page.margin)を広く設定し
- [両端揃え]($par.justify)、[詰めた行送り]($par.leading)、[1行目のインデント]($par.first-line-indent)を有効化し
- 本文と[コードブロック]($raw)の両方の[フォント]($text.font)を、Computer ModernのOpenType派生である「New Computer Modern」に設定し
- 段落の[間隔]($block.spacing)を無効化し
- [見出し]($heading)の周りの[間隔]($block.spacing)を広げます

```typ
#set page(margin: 1.75in)
#set par(leading: 0.55em, spacing: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show heading: set block(above: 1.4em, below: 1em)
```

これは良い出発点になるはずです！　もっと先に進みたい場合は、再利用可能なテンプレートを作成してみてはいかがでしょうか？

## 参考文献
Typstは、BibTeXファイルと互換性のある、フル機能の参考文献システムを備えています。[`bibliography`]($bibliography)関数で読み込むことで、お手元の`.bib`の文献ライブラリを引き続き使用できます。もう1つの選択肢として、[TypstのYAMLベースのネイティブ形式](https://github.com/typst/hayagriva/blob/main/docs/file-format.md)を使うこともできます。

Typstは引用および参考文献のスタイルを定義・処理するためにCitation Style Languageを使用します。CSLファイルはBibLaTeXの`.bbx`ファイルに相当します。コンパイラーには[80以上の引用スタイル]($bibliography.style)がすでに含まれていますが、[CSLリポジトリ](https://github.com/citation-style-language/styles)にあるCSL準拠の任意のスタイルを使ったり、独自のスタイルを書いたりすることもできます。

参考文献内のエントリーを引用したり、文書内のラベルを参照したりするには同じ構文を使います。`[@key]`（これは`key`という名前のエントリーを参照します）。代わりに、[`cite`]($cite)関数を使うこともできます。

引用の代替形式、例えば年のみの形式や（`\citet`や`\textcite`に相当する）散文の中で自然に使うための形式は、[`[#cite(<key>, form: "prose")]`]($cite.form)で利用できます。

詳しい情報は[`bibliography`]($bibliography)関数のドキュメントページにあります。

## TypstはLaTeXに対して現在どのような制約がありますか？ { #limitations }
Typstは今日、多くの方にとってLaTeXの代替となりますが、Typstがまだ（現時点で）サポートしていない機能があります。以下にそれらの一覧を示し、可能な場合には回避策もあわせて記します。

- **確立されたプロット環境**：LaTeXのユーザーは文書とともにPGF/TikZで凝ったチャートを作成することがよくあります。Typstのエコシステムはまだ同じ幅広い選択肢を提供できていませんが、[`cetz`パッケージ](https://typst.app/universe/package/cetz)を中心としたエコシステムが急速に追いつきつつあります。

- **改ページなしでのページ余白の変更**：LaTeXでは、改ページなしであっても余白を常に調整できます。Typstで余白を変更するには[`page`関数]($page)を使用しますが、これは強制的に改ページを発生させます。少数の段落だけを余白に張り出させ、その後に元の余白に戻したい場合は、負のパディングを指定した[`pad`関数]($pad)を使えます。
