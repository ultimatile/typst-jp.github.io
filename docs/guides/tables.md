---
description: |
  表の罫線の変更方法がわからない？表を回転したい？このガイドでは、Typstで表を扱うために必要な知識を全て解説します。
---

# 表ガイド
表は、データを読みやすく、コンパクトで、整理された形で読者に提示するためのすぐれた方法です。表は数値だけでなく、アンケートの回答、タスク計画、スケジュールなど、さまざまな用途で使われます。このように応用範囲が広いため、表のレイアウトに唯一最善の方法というものはありません。代わりに、強調したいデータ、文書全体のデザイン、そして最終的に読者にとってその表がどのように役立つかを考えましょう。

Typstは、スタイル設定の自動化や他のアプリケーションからのデータの取り込みなど、さまざまな機能で表の作成を支援します。このガイドでは、Typstで文書に表を追加する際によくある質問のいくつかを取り上げます。順番通りに読まなくても理解できるよう設計しているので、自分に最も関連のあるセクションに飛んでお読みください。

表の動作の詳細を調べたい場合は、[リファレンスページ]($table)もご確認ください。また、通常の表ではなく目次を探している場合は、[`outline`関数]($outline)のリファレンスページが詳しく学ぶのに適した場所です。

## 基本的な表の作り方 { #basic-tables }
Typstで表を作成するには、[`table`関数]($table)を使用します。基本的な表を作るには、table関数に次の2つを伝える必要があります。

- 列の数
- 表の各セルのコンテンツ

例えば、クッキーのレシピの材料を説明する2列の表を作りたい場合を考えてみましょう。

```example
#table(
  columns: 2,
  [*Amount*], [*Ingredient*],
  [360g], [Baking flour],
  [250g], [Butter (room temp.)],
  [150g], [Brown sugar],
  [100g], [Cane sugar],
  [100g], [70% cocoa chocolate],
  [100g], [35-40% cocoa chocolate],
  [2], [Eggs],
  [Pinch], [Salt],
  [Drizzle], [Vanilla extract],
)
```

この例は、表の呼び出し、設定、内容の埋め込み方を示しています。列数とセルの内容はどちらも引数として表に渡されます。[引数リスト]($function)は丸括弧で囲まれます。その中で、まず列数を名前付き引数として渡します。次に、複数の[コンテンツブロック]($content)を位置引数として渡します。各コンテンツブロックには、1つのセルの内容が含まれます。

この例を読みやすくするため、各行に2つのコンテンツブロック引数を配置し、表上での見た目を模倣しました。各セルを独立した行に書くこともできます。Typstは引数をどの行に配置するかは気にしません。代わりに、Typstはコンテンツセルを左から右（言語の書字方向によっては右から左）に、そして上から下へと配置していきます。全てのコンテンツが収まるよう、行を自動的に追加します。

表のヘッダー行は、[`table.header`関数]($table.header)で囲むのが最善です。これにより意図が明確になり、Typstがスクリーンリーダーを使用するユーザーにとってより[アクセシブル]($guides/accessibility)な出力を生成できるようになります。

```example
#table(
  columns: 2,
  table.header[*Amount*][*Ingredient*],
  [360g], [Baking flour],
<<<  // ... the remaining cells
>>>  [250g], [Butter (room temp.)],
>>>  [150g], [Brown sugar],
>>>  [100g], [Cane sugar],
>>>  [100g], [70% cocoa chocolate],
>>>  [100g], [35-40% cocoa chocolate],
>>>  [2], [Eggs],
>>>  [Pinch], [Salt],
>>>  [Drizzle], [Vanilla extract],
)
```

全ての表で先頭セルの内容を自動的に[強調表示]($strong)するshowルールを書くこともできます。文書に複数の表が含まれる場合、これはすぐに役立ちます。

```example
#show table.cell.where(y: 0): strong

#table(
  columns: 2,
  table.header[Amount][Ingredient],
  [360g], [Baking flour],
<<<  // ... the remaining cells
>>>  [250g], [Butter (room temp.)],
>>>  [150g], [Brown sugar],
>>>  [100g], [Cane sugar],
>>>  [100g], [70% cocoa chocolate],
>>>  [100g], [35-40% cocoa chocolate],
>>>  [2], [Eggs],
>>>  [Pinch], [Salt],
>>>  [Drizzle], [Vanilla extract],
)
```

ここでは、`table.header`に直接スタイルを適用するのではなく、セル座標のセレクターを使ったshowルールを使っています。これは、将来のリリースで修正される予定のTypstの現在の制限によるものです。

おめでとうございます、最初の表を作成できました。次は、[列のサイズ変更](#column-sizes)、[罫線の調整](#strokes)、[行の縞模様の追加](#fills)などに進めます。

## 列のサイズの変更方法 { #column-sizes }
表を作成して列数を指定すると、Typstは各列を最大のセルが収まるサイズに調整します。多くの場合、それとは別の挙動、例えば表をページの全幅にわたって広げたいでしょう。`columns`引数を通じて、各列の幅を指定するリストを渡すことができます。列幅を指定する方法はいくつかあります。

- まず、`{auto}`があります。これはデフォルトの動作で、Typstに対し、内容にあわせて列を伸ばすよう指示します。十分なスペースがない場合、Typstは可能な限り`{auto}`サイズの列の間でスペースを分配しようとします。
- `{6cm}`、`{0.7in}`、`{120pt}`のような[長さ]($length)。通常通り、フォント依存の`em`単位も使えます。これは現在のフォントサイズの倍数です。フォントサイズに関係なく、常に同じ程度のテキストが収まるようにサイズ調整したい場合に便利です。
- `{40%}`のような[パーセント比率]($ratio)。これにより、列は表に対して使用可能な水平方向のスペース全体（つまりページ内の幅、または表のコンテナ）の40%を占めます。比率と長さを組み合わせて[相対長]($relative)にすることもできます。合計が100%になる列幅のリストを指定しても、表がコンテナよりも大きくなる場合があることに注意してください。これは列幅に含まれない[罫間]($table.gutter)が列の間に存在し得るためです。表をページに合わせたい場合は、次の選択肢が役立つことが多いです。
- `1fr`のような`fr`単位を使った[空きスペースの分数部分]($fraction)。この単位を使うことで、利用可能なスペースを列に分配できます。動作は次の通りです。まず、Typstは`fr`を使用しない全ての列の長さを合計します。次に、残りの水平方向のスペースを決定します。この水平方向のスペースは、`fr`で表現された全ての列に分配されます。この処理の中で、`2fr`の列は`1fr`の列の2倍の幅になります。この単位の名前はここから来ています。列の幅は、分数指定された列全体に対するその列の分数です。

これを実際に使ってみましょう。日付、番号、いくつかの定期点検の説明を含む表を例にします。最初の2列は`auto`サイズで、最後の列はページ全体に広がるよう`1fr`の幅です。

```example
#table(
  columns: (auto, auto, 1fr),
  table.header[Date][°No][Description],
  [24/01/03], [813], [Filtered participant pool],
  [24/01/03], [477], [Transitioned to sec. regimen],
  [24/01/11], [051], [Cycled treatment substrate],
)
```

ここでは、列の長さのリストを丸括弧で囲み、要素をカンマで区切った[配列]($array)として渡しました。最初の2列は自動サイズなので、内容のサイズにあわせて調整され、3列目は`{1fr}`サイズで指定され、ページの残りのスペースを埋めます。代わりに、2列目をもう少しゆとりのあるサイズにしたい場合は、`columns`配列内のその列のエントリを`{6em}`のような値に置き換えればよいでしょう。

## 表にキャプションと参照を付ける方法 { #captions-and-references }
表は、読者がそこから読み取る情報と同じくらい価値があります。本文と表との間に明確な関連を相互参照によって持たせることで、両者の効果を高めることができます。Typstは自動的な[参照]($ref)と[`figure`関数]($figure)で支援します。

画像と同様に、表を`figure`関数で囲むことで、キャプションとラベルを追加でき、後で図表を参照できます。表をfigureで囲むと、`placement`パラメーターを使って図表をページの上下に浮動配置できるようにもなります。

キャプション付きの表と本文中での参照方法を見てみましょう。

```example
>>> #set page(width: 14cm)
#show table.cell.where(y: 0): set text(weight: "bold")

#figure(
  table(
    columns: 4,
    stroke: none,

    table.header[Test Item][Specification][Test Result][Compliance],
    [Voltage], [220V ± 5%], [218V], [Pass],
    [Current], [5A ± 0.5A], [4.2A], [Fail],
  ),
  caption: [Probe results for design A],
) <probe-a>

The results from @probe-a show that the design is not yet optimal.
We will show how its performance can be improved in this section.
```

この例では、表をfigureで囲み、キャプションとラベルを設定し、そのラベルを参照する方法を示しています。まず`figure`関数を使います。この関数は、図表の内容を位置引数として受け取ります。table関数の呼び出しをその引数リストに入れ、`#`文字は省略します。`#`はマークアップモードで関数を呼び出すときにのみ必要だからです。また、キャプションを名前付き引数として（表の上または下に）追加します。

figureの呼び出しの後、山括弧で囲まれたラベル（`[<probe-a>]`）を置きます。これにより、Typstはこの要素を記憶し、文書全体でこの名前で参照できるようにします。本文中では、アットマークとラベル名（`[@probe-a]`）を使って参照できます。Typstはきれいに整形された参照を表示し、表の番号が変わった場合にも自動的にラベルを更新します。

## 縞模様の表を作る方法 { #fills }
多くの表では、行や列を区別するために罫線ではなく縞模様の行や列を使います。この効果はしばしば _ゼブラストライプ_ と呼ばれます。ゼブラストライプの表はビジネスや商用のデータ分析の場で人気がある一方、学術用途では罫線が好まれる傾向があります。

表にゼブラストライプを追加するには、`table`関数の`fill`引数を使います。この引数は3種類の値を受け取ります。

- 全てのセルを塗りつぶす単一の色（グラデーションやタイリングも使えます）。一部のセルに別の色を付けたいので、ゼブラストライプの表を作るのには役立ちません。
- 各列ごとにTypstが順に使用する色の配列。2要素の配列を使うことで、縞模様の列を作ることができます。
- セルの水平座標`x`と垂直座標`y`を受け取り、その塗りつぶしを返す関数。これを使うことで、横縞や[市松模様]($grid.cell)を作成できます。

横方向の縞模様の表の例から始めましょう。

```example
>>> #set page(width: 16cm)
#set text(font: "IBM Plex Sans")

// Medium bold table header.
#show table.cell.where(y: 0): set text(weight: "medium")

// Bold titles.
#show table.cell.where(x: 1): set text(weight: "bold")

// See the strokes section for details on this!
#let frame(stroke) = (x, y) => (
  left: if x > 0 { 0pt } else { stroke },
  right: stroke,
  top: if y < 2 { stroke } else { 0pt },
  bottom: stroke,
)

#set table(
  fill: (rgb("EAF2F5"), none),
  stroke: frame(1pt + rgb("21222C")),
)

#table(
  columns: (0.4fr, 1fr, 1fr, 1fr),

  table.header[Month][Title][Author][Genre],
  [January], [The Great Gatsby], [F. Scott Fitzgerald], [Classic],
  [February], [To Kill a Mockingbird], [Harper Lee], [Drama],
  [March], [1984], [George Orwell], [Dystopian],
  [April], [The Catcher in the Rye], [J.D. Salinger], [Coming-of-Age],
)
```

この例は読書クラブの読書リストを示しています。`table`のsetルール内の`{fill: (rgb("EAF2F5"), none)}`という行が、縞模様の列を追加するために必要な全てです。これは、明るい青（[`rgb`]($color.rgb)関数の呼び出し内）と何もない（`{none}`）状態を交互に切り替えて列を塗るようTypstに指示します。スタイルを全て`table`関数の呼び出し自体からsetルールとshowルールに抽出したことに注意してください。これにより、複数の表で自動的に再利用できます。

縞模様自体の設定は簡単なので、見栄えをよくするためにいくつか別のスタイルも追加しました。例の他のコードでは、表の周囲と最初の行の下に濃い青の[ストローク](#stroke-functions)を引き、最初の行と本のタイトルの列を太字にしています。このストロークの設定方法の詳細は、[ストローク](#strokes)のセクションを参照してください。

次に、setルールだけを変えて、代わりに横方向の縞模様を作る方法を見てみましょう。

```example
>>> #set page(width: 16cm)
>>> #set text(font: "IBM Plex Sans")
>>> #show table.cell.where(x: 1): set text(weight: "medium")
>>> #show table.cell.where(y: 0): set text(weight: "bold")
>>>
>>> #let frame(stroke) = (x, y) => (
>>>   left: if x > 0 { 0pt } else { stroke },
>>>   right: stroke,
>>>   top: if y < 2 { stroke } else { 0pt },
>>>   bottom: stroke,
>>> )
>>>
#set table(
  fill: (_, y) => if calc.odd(y) { rgb("EAF2F5") },
  stroke: frame(1pt + rgb("21222C")),
)
>>>
>>> #table(
>>>   columns: (0.4fr, 1fr, 1fr, 1fr),
>>>
>>>   table.header[Month][Title][Author][Genre],
>>>   [January], [The Great Gatsby],
>>>     [F. Scott Fitzgerald], [Classic],
>>>   [February], [To Kill a Mockingbird],
>>>     [Harper Lee], [Drama],
>>>   [March], [1984],
>>>     [George Orwell], [Dystopian],
>>>   [April], [The Catcher in the Rye],
>>>     [J.D. Salinger], [Coming-of-Age],
>>> )
```

前の例のsetルールをこれに置き換えるだけで、代わりに横方向の縞模様が得られます。ここでは、`fill`に関数を渡しています。アンダースコアで水平座標を破棄し、セルの垂直座標`y`が奇数かどうかをチェックします。奇数であれば、セルに明るい青の塗りつぶしが適用され、そうでなければ何も塗りつぶされません。

もちろん、この関数は任意の複雑さで作ることができます。例えば、明るい青と濃い青の濃淡で行を縞模様にしたい場合は、次のようにできます。

```example
>>> #set page(width: 16cm)
>>> #set text(font: "IBM Plex Sans")
>>> #show table.cell.where(x: 1): set text(weight: "medium")
>>> #show table.cell.where(y: 0): set text(weight: "bold")
>>>
>>> #let frame(stroke) = (x, y) => (
>>>   left: if x > 0 { 0pt } else { stroke },
>>>   right: stroke,
>>>   top: if y < 2 { stroke } else { 0pt },
>>>   bottom: stroke,
>>> )
>>>
#set table(
  fill: (_, y) => (none, rgb("EAF2F5"), rgb("DDEAEF")).at(calc.rem(y, 3)),
  stroke: frame(1pt + rgb("21222C")),
)
>>>
>>> #table(
>>>   columns: (0.4fr, 1fr, 1fr, 1fr),
>>>
>>>   table.header[Month][Title][Author][Genre],
>>>   [January], [The Great Gatsby],
>>>     [F. Scott Fitzgerald], [Classic],
>>>   [February], [To Kill a Mockingbird],
>>>     [Harper Lee], [Drama],
>>>   [March], [1984],
>>>     [George Orwell], [Dystopian],
>>>   [April], [The Catcher in the Rye],
>>>     [J.D. Salinger], [Coming-of-Age],
>>> )
```

この例は、塗りつぶし関数を書くもう1つのアプローチを示しています。この関数は3色の配列を使い、`y`を3で割った余りで配列にインデックスすることで、各行ごとに値を切り替えます。

最後に、_ストローク_ を使って縞模様の行を実現するボーナス例を示します。

```example
>>> #set page(width: 16cm)
>>> #set text(font: "IBM Plex Sans")
>>> #show table.cell.where(x: 1): set text(weight: "medium")
>>> #show table.cell.where(y: 0): set text(weight: "bold")
>>>
>>> #let frame(stroke) = (x, y) => (
>>>   left: if x > 0 { 0pt } else { stroke },
>>>   right: stroke,
>>>   top: if y < 2 { stroke } else { 0pt },
>>>   bottom: stroke,
>>> )
>>>
#set table(
  stroke: (x, y) => (
    y: 1pt,
    left: if x > 0 { 0pt } else if calc.even(y) { 1pt },
    right: if calc.even(y) { 1pt },
  ),
)
>>>
>>> #table(
>>>   columns: (0.4fr, 1fr, 1fr, 1fr),
>>>
>>>   table.header[Month][Title][Author][Genre],
>>>   [January], [The Great Gatsby],
>>>     [F. Scott Fitzgerald], [Classic],
>>>   [February], [To Kill a Mockingbird],
>>>     [Harper Lee], [Drama],
>>>   [March], [1984],
>>>     [George Orwell], [Dystopian],
>>>   [April], [The Catcher in the Rye],
>>>     [J.D. Salinger], [Coming-of-Age],
>>> )
```

### 個別のセルの塗りつぶしの色を手動で上書きする { #fill-override }
セルの塗りつぶしを表内の位置ではなく内容に応じて変えたい場合があります。`table`のパラメーターリストで[`table.cell`要素]($table.cell)を使うことで、セルの内容を囲み、その塗りつぶしを上書きできます。

例えば、ドイツの歴代大統領のリストを示し、各セルの枠を所属政党の色で塗ったものを示します。

```example
>>> #set page(width: 10cm)
#set text(font: "Roboto")

#let cdu(name) = ([CDU], table.cell(fill: black, text(fill: white, name)))
#let spd(name) = ([SPD], table.cell(fill: red, text(fill: white, name)))
#let fdp(name) = ([FDP], table.cell(fill: yellow, name))

#table(
  columns: (auto, auto, 1fr),
  stroke: (x: none),

  table.header[Tenure][Party][President],
  [1949-1959], ..fdp[Theodor Heuss],
  [1959-1969], ..cdu[Heinrich Lübke],
  [1969-1974], ..spd[Gustav Heinemann],
  [1974-1979], ..fdp[Walter Scheel],
  [1979-1984], ..cdu[Karl Carstens],
  [1984-1994], ..cdu[Richard von Weizsäcker],
  [1994-1999], ..cdu[Roman Herzog],
  [1999-2004], ..spd[Johannes Rau],
  [2004-2010], ..cdu[Horst Köhler],
  [2010-2012], ..cdu[Christian Wulff],
  [2012-2017], [n/a], [Joachim Gauck],
  [2017-],     ..spd[Frank-Walter-Steinmeier],
)
```

この例では、メンバーが大統領になった政党は3つしかなく（さらに無所属の大統領が1人）、その色が複数回繰り返されるため変数を使っています。政党名と、その政党の色および大統領名を持つ表セルを含む配列を生成する関数（`cdu`、`spd`、`fdp`）を保存します。これらの関数を、`table`の引数リストの中で名前を直接追加する代わりに使います。[展開演算子]($arguments/#spreading)`..`を使って、配列の項目を個々のセルに変換します。`table`の引数リストの各セルに対して`{[FDP], table.cell(fill: yellow)[Theodor Heuss]}`のように直接書くこともできますが、これは特に色が暗くて白いテキストが必要な政党では読みにくくなります。また、垂直の罫線を消し、フォントをRobotoに設定しています。

この例では、政党列とセルの色は意図的に冗長な情報を伝えています。重要なデータを色のみで伝えることはアクセシビリティに反する悪い習慣です。視覚障害のあるユーザーにとって不利となり、[WCAG 2.1 達成基準1.4.1](https://www.w3.org/WAI/WCAG21/Understanding/use-of-color.html)などのユニバーサルアクセス基準に違反します。この表を改善するために、政党名を表示する列を追加しました。あるいは、色覚多様性に配慮したパレットを選び、スクリーンリーダーが読み上げられる追加のラベルでセルをマークアップすることもできます。後者の機能は現在Typstではサポートされていませんが、将来のリリースで追加される予定です。色がどのように色覚多様性のある読者に見えるかは、[このChrome拡張機能](https://chromewebstore.google.com/detail/colorblindly/floniaahmccleoclneebhhmnjgdfijgg)、[Photoshop](https://helpx.adobe.com/photoshop/using/proofing-colors.html)、[GIMP](https://docs.gimp.org/2.10/en/gimp-display-filter-dialog.html)で確認できます。

## 表の罫線を調整する方法 { #strokes }
デフォルトでは、Typstは表の各行と各列の間に罫線を引きます。これらの罫線はさまざまな方法で調整できます。どれが最も実用的かは、行いたい変更と意図によって異なります。

- サイズや内容に関係なく、文書内の全ての表のスタイルを変更したいですか？setルールで`table`関数の[stroke]($table.stroke)引数を使います。
- 単一の表内の全ての罫線をカスタマイズしたいですか？table関数を呼び出すときに`table`関数の[stroke]($table.stroke)引数を使います。
- 単一のセル周囲の罫線を変更、追加、または削除したいですか？table呼び出しの引数リストで`table.cell`要素を使います。
- 単一の表の中の単一の水平または垂直の罫線を変更、追加、または削除したいですか？table呼び出しの引数リストで[`table.hline`]($table.hline)および[`table.vline`]($table.vline)要素を使います。

これらの選択肢を、次の例で全て見ていきます。まず、`table`関数の[stroke]($table.stroke)引数について見てみましょう。ここでは、表の罫線の描画方法と、どの罫線を描くかの両方を調整できます。

罫線の色と太さを変更することから始めましょう。

```example
#table(
  columns: 4,
  stroke: 0.5pt + rgb("666675"),
  [*Monday*], [11.5], [13.0], [4.0],
  [*Tuesday*], [8.0], [14.5], [5.0],
  [*Wednesday*], [9.0], [18.5], [13.0],
)
```

これにより、表の罫線が少し細くなり、青みがかった灰色になります。カスタマイズした罫線を実現するため、ポイント単位の幅と色を加算したことがわかるでしょう。この加算により[ストローク型]($stroke)の値が得られます。あるいは、破線などの高度な機能にアクセスできるストロークの辞書表現を使うこともできます。

前の例は、table関数の呼び出しで`stroke`引数を使う方法を示しました。代わりに、`table`のsetルールで`stroke`引数を指定することもできます。これは、引数リストに`stroke`引数を指定した場合と全く同じ効果を、その後の全ての`table`呼び出しに対して持ちます。これは、テンプレートを書く場合や、文書全体のスタイルを設定したい場合に役立ちます。

```typ
// Renders the exact same as the last example
#set table(stroke: 0.5pt + rgb("666675"))

#table(
  columns: 4,
  [*Monday*], [11.5], [13.0], [4.0],
  [*Tuesday*], [8.0], [14.5], [5.0],
  [*Wednesday*], [9.0], [18.5], [13.0],
)
```

小さな表では、視覚的なノイズが多すぎるため、全ての罫線を抑制したいことがあります。これを実現するには、`stroke`引数を`{none}`に設定するだけです。

```example
#table(
  columns: 4,
  stroke: none,
  [*Monday*], [11.5], [13.0], [4.0],
  [*Tuesday*], [8.0], [14.5], [5.0],
  [*Wednesday*], [9.0], [18.5], [13.0],
)
```

表内の罫線の配置をより細かく制御したい場合は、`top`、`left`、`right`、`bottom`（それぞれセルの該当する側を制御）、`x`、`y`（垂直および水平の罫線を制御）、`rest`（他の辞書エントリでスタイル設定されない全ての罫線をカバー）というキーを持つ辞書を渡すこともできます。全てのキーは省略可能です。省略されたキーは、以前に設定された値、または一度も設定されていない場合はデフォルト値を使用します。例えば、水平方向の罫線のみの表を得るには、次のようにします。

```example
#table(
  columns: 2,
  stroke: (x: none),
  align: horizon,
  [☒], [Close cabin door],
  [☐], [Start engines],
  [☐], [Radio tower],
  [☐], [Push back],
)
```

これは全ての垂直の罫線を消し、水平の罫線をそのまま残します。逆の効果（垂直の罫線のみ）を得るには、`stroke`引数を`{(y: none)}`に設定します。

[ガイドの後半](#stroke-functions)で、`stroke`引数に関数を使って全ての罫線を個別にカスタマイズする方法を扱います。これがより複雑な罫線パターンを実現する方法です。

### 表に個別の罫線を追加する { #individual-lines }
例えば行のグループを区切るために、表に1本の水平または垂直の罫線を追加したい場合は、それぞれ水平および垂直の罫線用の[`table.hline`]($table.hline)および[`table.vline`]($table.vline)要素を使えます。個別のセルやヘッダーを追加するときと同様に、`table`関数の引数リストに追加します。

リファレンスから次の例を見てみましょう。

```example
#set table.hline(stroke: 0.6pt)

#table(
  stroke: none,
  columns: (auto, 1fr),
  // Morning schedule abridged.
  [14:00], [Talk: Tracked Layout],
  [15:00], [Talk: Automations],
  [16:00], [Workshop: Tables],
  table.hline(),
  [19:00], [Day 1 Attendee Mixer],
)
```

この例では、セルの間に`table.hline`の呼び出しを置き、その位置に水平の罫線を生成しています。また、要素にsetルールを使って、フォントの太さに合うようにストローク幅を縮小しました。

デフォルトでは、Typstは引数リストでの位置に応じて、現在の行または列の後に水平および垂直の罫線を配置します。`y`引数（`hline`の場合）または`x`引数（`vline`の場合）を追加することで、別の位置に手動で移動させることもできます。例えば、次のコードは同じ結果を生成します。

```typ
#set table.hline(stroke: 0.6pt)

#table(
  stroke: none,
  columns: (auto, 1fr),
  // Morning schedule abridged.
  table.hline(y: 3),
  [14:00], [Talk: Tracked Layout],
  [15:00], [Talk: Automations],
  [16:00], [Workshop: Tables],
  [19:00], [Day 1 Attendee Mixer],
)
```

最初の行と2行目の間以外には罫線を表示しないテンプレートを使っているとします。一方で、最初の列にラベルがある表があり、それに垂直の罫線を追加で入れたいとします。ただし、この垂直の罫線が一番上の行を横切ってほしくありません。これは`start`引数で実現できます。

```example
>>> #set page(width: 12cm)
>>> #show table.cell.where(y: 0): strong
>>> #set table(stroke: (_, y) => if y == 0 { (bottom: 1pt) })
// Base template already configured tables, but we need some
// extra configuration for this table.
#{
  set table(align: (x, _) => if x == 0 { left } else { right })
  show table.cell.where(x: 0): smallcaps
  table(
    columns: (auto, 1fr, 1fr, 1fr),
    table.vline(x: 1, start: 1),
    table.header[Trainset][Top Speed][Length][Weight],
    [TGV Réseau], [320 km/h], [200m], [383t],
    [ICE 403], [330 km/h], [201m], [409t],
    [Shinkansen N700], [300 km/h], [405m], [700t],
  )
}
```

この例では、位置引数リストの先頭に`table.vline`を追加しました。しかし、この罫線は最初の列の左には引かないため、`x`引数を`{1}`に指定しました。また、`start`引数を`{1}`に設定し、罫線が最初の行の後からのみ始まるようにしました。

この例にはさらに2つの要素があります。`align`引数に関数を使って、最初の列以外の全ての列のデータを右揃えにし、showルールで最初の列の表セルをスモールキャピタルで表示しています。これらのスタイルはこの表だけに固有なので、全てを[コードブロック]($scripting/#blocks)に入れて、後続の表にスタイルが影響しないようにしています。

### 単一のセルの罫線を上書きする { #stroke-override }
1つのセル周りの罫線を変更したいとします。例えば、そのセルが非常に重要で強調が必要な場合などです。このようなシナリオには、[`table.cell`関数]($table.cell)があります。コンテンツを直接tableの引数リストに追加するのではなく、`table.cell`の呼び出しで囲みます。これで、`table.cell`の引数リストを使って、そのセルだけに対して罫線などの表のプロパティを上書きできます。

ビッグファイブ性格因子のうち2つの行列を示し、1つの交点を強調表示する例を示します。

```example
>>> #set page(width: 16cm)
#table(
  columns: 3,
  stroke: (x: none),

  table.header[][*High Neuroticism*][*Low Neuroticism*],

  [*High Agreeableness*],
  table.cell(stroke: orange + 2pt)[
    _Sensitive_ \ Prone to emotional distress but very empathetic.
  ],
  [_Compassionate_ \ Caring and stable, often seen as a supportive figure.],

  [*Low Agreeableness*],
  [_Contentious_ \ Competitive and easily agitated.],
  [_Detached_ \ Independent and calm, may appear aloof.],
)
```

上の例では、tableの引数リストで`table.cell`要素を使い、セルの内容を渡しています。その`stroke`引数を使って、より太いオレンジ色の罫線を設定しました。表で垂直の罫線を無効にしたにもかかわらず、変更したセルの全ての側にオレンジの罫線が現れていることから、表のストローク設定が上書きされていることが分かります。

### 文書全体での複雑な罫線のカスタマイズ { #stroke-functions }
このセクションでは、1つまたは複数の表で全ての罫線を一度にカスタマイズする方法を説明します。これにより、表のセル数を知らなくても、最初の水平の罫線だけを引いたり、外側の罫線を省略したりできます。これは、tableの`stroke`パラメーターに関数を渡すことで実現します。関数は、現在のセルの0始まりのxとyの位置を受け取り、ストロークを返します。これらの関数は、テンプレートの作者である場合、テンプレートを使わない場合、または表を大幅にカスタマイズする必要がある場合にのみ必要です。それ以外の場合、テンプレートが適切なデフォルトの表の罫線を設定するはずです。

例えば、これは最初と最後の水平の罫線を除く全ての水平の罫線を引くsetルールです。

```example
#show table.cell.where(x: 0): set text(style: "italic")
#show table.cell.where(y: 0): set text(style: "normal", weight: "bold")
#set table(stroke: (_, y) => if y > 0 { (top: 0.8pt) })

#table(
  columns: 3,
  align: center + horizon,
  table.header[Technique][Advantage][Drawback],
  [Diegetic], [Immersive], [May be contrived],
  [Extradiegetic], [Breaks immersion], [Obtrusive],
  [Omitted], [Fosters engagement], [May fracture audience],
)
```

このsetルールでは、2つの引数を受け取る関数を渡しています。垂直座標を`y`に割り当て、水平座標は破棄します。次に、最初の行を除く全ての行に対して`{0.8pt}`のtopストロークを持つストローク辞書を返します。最初の行のセルには、代わりに暗黙的に`{none}`が返されます。代わりに内側の垂直の罫線だけを引くには、関数を`{(x, _) => if x > 0 { (left: 0.8pt) }}`のように簡単に変更できます。

さらにいくつかのストローク関数を試してみましょう。次の関数は、最初の行の下にのみ罫線を引きます。

```example
>>> #show table.cell: it => if it.x == 0 and it.y > 0 {
>>>   set text(style: "italic")
>>>   it
>>> } else {
>>>   it
>>> }
>>>
>>> #show table.cell.where(y: 0): strong
#set table(stroke: (_, y) => if y == 0 { (bottom: 1pt) })

<<< // Table as seen above
>>> #table(
>>>   columns: 3,
>>>   align: center + horizon,
>>>   table.header[Technique][Advantage][Drawback],
>>>   [Diegetic], [Immersive], [May be contrived],
>>>   [Extradiegetic], [Breaks immersion], [Obtrusive],
>>>   [Omitted], [Fosters engagement], [May fracture audience],
>>> )
```

最初の例を理解していれば、ここで何が起こっているかは明らかでしょう。最初の行にいるかどうかをチェックします。そうであれば、bottomストロークを返します。そうでなければ、暗黙的に`{none}`を返します。

次の例では、外側の罫線を除く全ての罫線を引く方法を示します。

```example
>>> #show table.cell: it => if it.x == 0 and it.y > 0 {
>>>   set text(style: "italic")
>>>   it
>>> } else {
>>>   it
>>> }
>>>
>>> #show table.cell.where(y: 0): strong
#set table(stroke: (x, y) => (
  left: if x > 0 { 0.8pt },
  top: if y > 0 { 0.8pt },
))

<<< // Table as seen above
>>> #table(
>>>   columns: 3,
>>>   align: center + horizon,
>>>   table.header[Technique][Advantage][Drawback],
>>>   [Diegetic], [Immersive], [May be contrived],
>>>   [Extradiegetic], [Breaks immersion], [Obtrusive],
>>>   [Omitted], [Fosters engagement], [May fracture audience],
>>> )
```

この例では、`x`座標と`y`座標の両方を使います。最初の列ではleftストロークを省略し、最初の行ではtopストロークを省略します。右と下の罫線は引かれません。

最後に、最初の行の垂直の罫線と本体の水平の罫線を除く全ての罫線を引く表を示します。これは少しカレンダーのような見た目になります。

```example
>>> #show table.cell: it => if it.x == 0 and it.y > 0 {
>>>   set text(style: "italic")
>>>   it
>>> } else {
>>>   it
>>> }
>>>
>>> #show table.cell.where(y: 0): strong
#set table(stroke: (x, y) => (
  left: if x == 0 or y > 0 { 1pt } else { 0pt },
  right: 1pt,
  top: if y <= 1 { 1pt } else { 0pt },
  bottom: 1pt,
))

<<< // Table as seen above
>>> #table(
>>>   columns: 3,
>>>   align: center + horizon,
>>>   table.header[Technique][Advantage][Drawback],
>>>   [Diegetic], [Immersive], [May be contrived],
>>>   [Extradiegetic], [Breaks immersion], [Obtrusive],
>>>   [Omitted], [Fosters engagement], [May fracture audience],
>>> )
```

この例は少し複雑です。まず、セルの右側に全ての罫線を引きます。しかしこれは最上行にも罫線を引いてしまうことになり、それは不要です。ここで、`left`が`right`を上書きするという事実を利用し、最初の行にいない、または最初の列にいる場合にのみleftの罫線を引きます。それ以外の場合は、明示的にleftの罫線を消します。最後に、`bottom`の罫線を設定し、最初の2行については`top`キーで水平の罫線を引いて、それ以外の全てのtop罫線を抑制することで、水平の罫線を引きます。最後の罫線は、それを抑制できる`top`の罫線がないため表示されます。

### 二重線を実現する方法 { #double-stroke }
Typstはまだ二重線を引くネイティブな方法を持っていませんが、それを模倣する方法は複数あります。例えば[タイリング]($tiling)を使う方法もあります。このセクションでは、別の回避策として、表の罫間を使う方法を示します。

表は`gutter`引数を使ってセル同士を離すことができます。罫間が適用されると、新たに分離された各セルにストロークが描かれます。二重線を引きたい行または列の間に、選択的に罫間を追加できます。`row-gutter`および`column-gutter`引数を使ってこれを行えます。これらは罫間の値の配列を受け取ります。例を見てみましょう。

```example
#table(
  columns: 3,
  stroke: (x: none),
  row-gutter: (2.2pt, auto),
  table.header[Date][Exercise Type][Calories Burned],
  [2023-03-15], [Swimming], [400],
  [2023-03-17], [Weightlifting], [250],
  [2023-03-18], [Yoga], [200],
)
```

`row-gutter`に対して、最初の行と2行目の間に`{2.2pt}`の隙間を指定する配列を使ったことが分かります。次に`auto`が続きます（この場合のデフォルトは`{0pt}`の罫間）。これは配列の最後のエントリなので、他の全ての行の間の罫間になります。

## 表のセル内のコンテンツの揃え方 { #alignment }
表のコンテンツを揃えるには、複数の仕組みを使えます。`table`関数の`align`引数を使って表全体の整列を設定する（あるいはsetルールで使って文書全体の表の整列を設定する）か、[`align`]($align)関数（または`table.cell`の`align`引数）を使って単一のセルの整列を上書きできます。

`table`関数の`align`引数を使う際、[整列]($alignment)を指定する3つの方法から選べます。

- `right`（右上隅に整列）や`center + horizon`（全てのセルのコンテンツを中央に整列）のような単一の整列を指定するだけです。これは全てのセルの整列を変更します。
- 配列を渡します。Typstは各列に対してこの配列を順に使います。
- セルの水平座標`x`と垂直座標`y`を受け取り、整列を返す関数を渡します。

例えば、次の旅行の行程表は、`align`引数に配列を渡すことで、日付の列を右揃え、それ以外の全てを左揃えにしています。

```example
>>> #set page(width: 12cm)
#set text(font: "IBM Plex Sans")
#show table.cell.where(y: 0): set text(weight: "bold")

#table(
  columns: 4,
  align: (right, left, left, left),
  fill: (_, y) => if calc.odd(y) { green.lighten(90%) },
  stroke: none,

  table.header[Day][Location][Hotel or Apartment][Activities],
  [1], [Paris, France], [Hôtel de l'Europe], [Arrival, Evening River Cruise],
  [2], [Paris, France], [Hôtel de l'Europe], [Louvre Museum, Eiffel Tower],
  [3], [Lyon, France], [Lyon City Hotel], [City Tour, Local Cuisine Tasting],
  [4], [Geneva, Switzerland], [Lakeview Inn], [Lake Geneva, Red Cross Museum],
  [5], [Zermatt, Switzerland], [Alpine Lodge], [Visit Matterhorn, Skiing],
)
```

しかし、この例はまだ完璧ではありません。ヘッダーセルは下揃えにすべきです。代わりに関数を使ってこれを実現しましょう。

```example
>>> #set page(width: 12cm)
#set text(font: "IBM Plex Sans")
#show table.cell.where(y: 0): set text(weight: "bold")

#table(
  columns: 4,
  align: (x, y) =>
    if x == 0 { right } else { left } +
    if y == 0 { bottom } else { top },
  fill: (_, y) => if calc.odd(y) { green.lighten(90%) },
  stroke: none,

  table.header[Day][Location][Hotel or Apartment][Activities],
  [1], [Paris, France], [Hôtel de l'Europe], [Arrival, Evening River Cruise],
  [2], [Paris, France], [Hôtel de l'Europe], [Louvre Museum, Eiffel Tower],
<<<  // ... remaining days omitted
>>>  [3], [Lyon, France], [Lyon City Hotel], [City Tour, Local Cuisine Tasting],
>>>  [4], [Geneva, Switzerland], [Lakeview Inn], [Lake Geneva, Red Cross Museum],
>>>  [5], [Zermatt, Switzerland], [Alpine Lodge], [Visit Matterhorn, Skiing],
)
```

この関数では、最初の列にいるか（`{x == 0}`）、最初の行にいるか（`{y == 0}`）に基づいて、水平方向と垂直方向の整列を計算します。次に、水平方向の整列と垂直方向の整列を`+`で加算して、単一の2次元の整列を得るという事実を利用しています。

`table.cell`を使って単一のセルの整列を変更する例は、[そのリファレンスページ]($table.cell)で見つけられます。

## セルを結合する方法 { #merge-cells }
表に論理的なグループ分けや、隣接する複数のセルに同じデータが含まれている場合、複数のセルを1つの大きなセルに結合すると有利です。セルグループのもう1つの使い方は、複数行のヘッダーを持つ表です。これにより、例えば売上データの表を最初の行で四半期ごとに、2行目で月ごとにグループ化できます。

結合されたセルは、複数の行や列にまたがります。これは[`table.cell`]($table.cell)関数の`rowspan`と`colspan`引数で実現できます。セルにまたがらせたい行数または列数を指定するだけです。

下の例は、各チームメンバーの出社日とリモート勤務日を含むオフィスの出勤カレンダーです。表を一目でわかるようにするため、同じ値を持つ隣接するセルを結合しています。

```example
>>> #set page(width: 22cm)
#let ofi = [Office]
#let rem = [_Remote_]
#let lea = [*On leave*]

#show table.cell.where(y: 0): set text(
  fill: white,
  weight: "bold",
)

#table(
  columns: 6 * (1fr,),
  align: (x, y) => if x == 0 or y == 0 { left } else { center },
  stroke: (x, y) => (
    // Separate black cells with white strokes.
    left: if y == 0 and x > 0 { white } else { black },
    rest: black,
  ),
  fill: (_, y) => if y == 0 { black },

  table.header(
    [Team member],
    [Monday],
    [Tuesday],
    [Wednesday],
    [Thursday],
    [Friday]
  ),
  [Evelyn Archer],
    table.cell(colspan: 2, ofi),
    table.cell(colspan: 2, rem),
    ofi,
  [Lila Montgomery],
    table.cell(colspan: 5, lea),
  [Nolan Pearce],
    rem,
    table.cell(colspan: 2, ofi),
    rem,
    ofi,
)
```

この例では、まず「Office」、「Remote」、「On leave」の変数を定義し、繰り返しこれらのラベルを書き出さなくて済むようにしています。これらの変数は表本体で直接使うか、あるいはチームメンバーが連続する複数日を出社、リモート、休暇で過ごす場合は`table.cell`の呼び出し内で使えます。

この例には、さらに黒いヘッダー（`table`の`fill`引数で作成）、白いストローク（`table`の`stroke`引数）、白いテキスト（`table.cell`のsetルールで設定）も含まれています。最後に、本体の全ての表セルのコンテンツを中央に整列させています。`align`、`stroke`、`fill`に渡される関数についてもっと知りたい場合は、[整列](#alignment)、[ストローク](#stroke-functions)、[縞模様の表](#fills)のセクションを参照してください。

この表は、外部のデータソースから完全に自動生成するのに適した候補でしょう。これについて詳しくは、[データの取り込みについてのセクション](#importing-data)をご覧ください。

## 表を回転する方法 { #rotate-table }
表に多くの列がある場合、縦長の用紙では窮屈になりがちです。そのため、表を横向きに切り替えたいことがあります。Typstでこれを実現する方法は2つあります。

- 表だけを回転させ、ページの他のコンテンツやページ自体は回転させたくない場合は、`reflow`引数を`{true}`に設定して[`rotate`関数]($rotate)を使います。
- 表が含まれるページ全体を回転させたい場合は、`flipped`引数を`{true}`に設定して[`page`関数]($page)を使えます。ヘッダー、フッター、ページ番号も用紙の長辺に表示されるようになります。これにはコンピューターで読むときに表が正しい向きで表示される利点がありますが、文書内のあるページが他のページと寸法が異なることになり、読者にとって違和感を与えることがあります。

下では、両方の手法を学生の成績表で示します。

まず、ページ上で表を回転させます。この例では、表の右側にテキストも配置しています。

```example
#set page("a5", columns: 2, numbering: "— 1 —")
>>> #set page(margin: auto)
#show table.cell.where(y: 0): set text(weight: "bold")

#rotate(
  -90deg,
  reflow: true,

  table(
    columns: (1fr,) + 5 * (auto,),
    inset: (x: 0.6em,),
    stroke: (_, y) => (
      x: 1pt,
      top: if y <= 1 { 1pt } else { 0pt },
      bottom: 1pt,
    ),
    align: (left, right, right, right, right, left),

    table.header(
      [Student Name],
      [Assignment 1], [Assignment 2],
      [Mid-term], [Final Exam],
      [Total Grade],
    ),
    [Jane Smith], [78%], [82%], [75%], [80%], [B],
    [Alex Johnson], [90%], [95%], [94%], [96%], [A+],
    [John Doe], [85%], [90%], [88%], [92%], [A],
    [Maria Garcia], [88%], [84%], [89%], [85%], [B+],
    [Zhang Wei], [93%], [89%], [90%], [91%], [A-],
    [Marina Musterfrau], [96%], [91%], [74%], [69%], [B-],
  ),
)

#lorem(80)
```


ここでは、ISO A5用紙のページ番号付きの2段組み文書を作成しています。表は6列を持ち、[ストローク](#strokes)、整列、間隔のいくつかのカスタマイズが含まれています。しかし最も重要な点は、表が`reflow`引数を`{true}`にした`rotate`関数の呼び出しで囲まれていることです。これにより、表は反時計回りに90度回転します。reflow引数は、表の回転がレイアウトに影響するように必要です。これが省略された場合、Typstは表が回転していないかのようにページをレイアウトします（`{true}`は将来的にデフォルトになる可能性があります）。

この例では、同じサイズの多数の列を作る方法も示しています。最初の`{1fr}`の列に、5つの`{auto}`項目を持つ配列を加えます。この配列は、1つの`{auto}`項目を持つ配列に5を掛けて作成します。1つの項目しかない配列には、単なる括弧で囲まれた式と区別するために末尾のカンマが必要であることに注意してください。

2つ目の例では、ページ全体を回転させ、表が正しい向きのままになる方法を示します。

```example
#set page("a5", numbering: "— 1 —")
>>> #set page(margin: auto)
#show table.cell.where(y: 0): set text(weight: "bold")

#page(flipped: true)[
  #table(
    columns: (1fr,) + 5 * (auto,),
    inset: (x: 0.6em,),
    stroke: (_, y) => (
      x: 1pt,
      top: if y <= 1 { 1pt } else { 0pt },
      bottom: 1pt,
    ),
    align: (left, right, right, right, right, left),

    table.header(
      [Student Name],
      [Assignment 1], [Assignment 2],
      [Mid-term], [Final Exam],
      [Total Grade],
    ),
    [Jane Smith], [78%], [82%], [75%], [80%], [B],
    [Alex Johnson], [90%], [95%], [94%], [96%], [A+],
    [John Doe], [85%], [90%], [88%], [92%], [A],
    [Maria Garcia], [88%], [84%], [89%], [85%], [B+],
    [Zhang Wei], [93%], [89%], [90%], [91%], [A-],
    [Marina Musterfrau], [96%], [91%], [74%], [69%], [B-],
  )

  #pad(x: 15%, top: 1.5em)[
    = Winter 2023/24 results
    #lorem(80)
  ]
]
```

ここでは、同じ表とそれと一緒に組みたいその他のコンテンツを取り、`flipped`引数に`{true}`を渡して[`page`]($page)関数の呼び出しに入れています。これにより、Typstは幅と高さを入れ替えた新しいページを作成し、関数呼び出しの内容を新しいページに配置するよう指示されます。ページ番号も用紙の長辺にあることに注目してください。ページの下部では、[`pad`]($pad)関数を使って段落の幅を制限し、きれいで読みやすい行長を実現しています。

## 表をページ間で改ページする方法 { #table-across-pages }
表は1ページに収めるのが最善です。しかし、行数が多い表もあり、ページ間で改ページするのは避けられないことがあります。幸いなことに、Typstはすぐに使えるページ間での表の改ページに対応しています。[`table.header`]($table.header)関数および[`table.footer`]($table.footer)関数を使っていれば、その内容は各ページの最初の行と最後の行としてそれぞれ繰り返されます。この動作を無効にしたい場合は、いずれかの関数で`repeat`を`{false}`に設定できます。

表を[figure]($figure)の中に配置している場合、デフォルトではページ間で改ページできなくなります。しかし、この動作は変更できます。見てみましょう。

```example
#set page(width: 9cm, height: 6cm)
#show table.cell.where(y: 0): set text(weight: "bold")
#show figure: set block(breakable: true)

#figure(
  caption: [Training regimen for Marathon],
  table(
    columns: 3,
    fill: (_, y) => if y == 0 { gray.lighten(75%) },

    table.header[Week][Distance (km)][Time (hh:mm:ss)],
    [1], [5],  [00:30:00],
    [2], [7],  [00:45:00],
    [3], [10], [01:00:00],
    [4], [12], [01:10:00],
    [5], [15], [01:25:00],
    [6], [18], [01:40:00],
    [7], [20], [01:50:00],
    [8], [22], [02:00:00],
    [...], [...], [...],
    table.footer[_Goal_][_42.195_][_02:45:00_],
  )
)
```

figureは、デフォルトでは改ページできない[ブロック]($block)を自動的に生成します。しかし、figureのブロックをshowルールで再設定して`breakable`にできます。これで、figureは複数ページにわたり、ヘッダーとフッターが繰り返されます。

## 表にデータを取り込む方法 { #importing-data }
他で取得したデータを表に入れる必要がよくあります。Microsoft ExcelやGoogle Sheetsから、Web上のデータセットから、あるいは実験から得たものなどです。幸いなことに、Typstは多くの[一般的なファイル形式]($category/data-loading)を読み込めるため、スクリプトでそのデータを表に取り込めます。

表形式データの最も一般的なファイル形式はCSVです。Excelで _ファイル_ メニューから「名前を付けて保存」を選び、ファイル形式「CSV UTF-8 (Comma-delimited) (.csv)」を選択することでCSVファイルを取得できます。ファイルを保存し、Webアプリを使っている場合は、プロジェクトにアップロードします。

ここでは、ムーアの法則についての表を作成します。この目的のために、[Our World in Dataの「マイクロプロセッサーあたりのトランジスタ数」の年別統計](https://ourworldindata.org/grapher/transistors-per-microprocessor)を使います。「Download」ボタンを押して、生のデータが入ったCSVファイルを取得することから始めましょう。

CLIを使っている場合は、ファイルをプロジェクトまたはTypstが見える場所に必ず移動させてください。それが終わったら、ファイルを開いて構造を確認できます。

```csv
Entity,Code,Year,Transistors per microprocessor
World,OWID_WRL,1971,2308.2417
World,OWID_WRL,1972,3554.5222
World,OWID_WRL,1974,6097.5625
```

ファイルはヘッダーで始まり、4つの列が含まれています。Entity（指標が誰に該当するか）、Code、年、マイクロプロセッサーあたりのトランジスタ数です。各行で変わるのは最後の2つの列だけなので、「Entity」と「Code」は無視できます。

まず、[`csv`]($csv)関数でこのファイルを読み込むことから始めましょう。読み込みたいファイルのファイル名を文字列引数として受け取ります。

```typ
#let moore = csv("moore.csv")
```

ファイル（`moore.csv`という名前と仮定）を読み込み、新しい変数`moore`に[束縛]($scripting/#bindings)しました。これは出力を生成しないので、まだ何も見えません。Typstが何を読み込んだかを確認したい場合は、Webアプリで変数名にカーソルを合わせるか、配列からいくつか項目を出力できます。

```example
#let moore = csv("moore.csv")

#moore.slice(0, 3)
```

`{(0, 3)}`という引数を使うことで、[`slice`]($array.slice)メソッドは配列の最初の3つの項目（インデックス0、1、2）を返します。各行が、1セルあたり1項目の独自の配列になっていることが分かります。

次に、このデータをtable関数で使えるセルの配列に変換するループを書きましょう。

```example
#let moore = csv("moore.csv")

#table(
  columns: 2,
  ..for (.., year, count) in moore {
    (year, count)
  }
)
```

上の例では、CSVファイルの行を反復処理し、各反復で配列を返すforループを使っています。forループの[分配]($scripting/#bindings)機能を使って、各行の最後の2項目以外を破棄します。次に、これらの2項目だけを持つ新しい配列を作成します。Typstは全てのループ反復の配列の結果を連結するため、年の列とトランジスタ数が交互に並ぶ1次元の配列が得られます。次に、この配列をセルとして挿入できます。これには[展開演算子]($arguments/#spreading)（`..`）を使います。配列、または私たちの場合は配列を生成する式の前に2つのドットを付けることで、Typstに配列の項目を位置引数として使うよう指示します。

代わりに、[`map`]($array.map)、[`slice`]($array.slice)、[`flatten`]($array.flatten)の配列メソッドを使って、より関数型のスタイルで書くこともできます。

```typ
#let moore = csv("moore.csv")

#table(
   columns: moore.first().len(),
   ..moore.map(m => m.slice(2)).flatten(),
)
```

この例は前の例と同じ結果になりますが、まず`map`関数を使ってデータの各行を変更します。CSVの各行に対して実行され、その行を置き換える新しい値を返す関数を`map`に渡します。これを使って、`slice`で最初の2列を破棄します。次に、データを`table`関数に展開します。しかし、1次元の配列を渡す必要がある一方で、`moore`の値は2次元です（つまり、各行の値はセルデータの配列を含みます）。そのため、`flatten`を呼び出して1次元の配列に変換します。また、列数もデータ自体から取り出しています。

表のためのきれいなコードができたので、表自体もきれいに作ってみましょう。トランジスタ数は1995年の数百万から2021年の数兆までで、桁数が多すぎて変化が見にくくなっています。データを対数で表示してみることで、より理解しやすくなるかもしれません。

```example
#let moore = csv("moore.csv")
#let moore-log = moore.slice(1).map(m => {
  let (.., year, count) = m
  let log = calc.log(float(count))
  let rounded = str(calc.round(log, digits: 2))
  (year, rounded)
})

#show table.cell.where(x: 0): strong

#table(
   columns: moore-log.first().len(),
   align: right,
   fill: (_, y) => if calc.odd(y) { rgb("D7D9E0") },
   stroke: none,

   table.header[Year][Transistor count ($log_10$)],
   table.hline(stroke: rgb("4D4C5B")),
   ..moore-log.flatten(),
)
```

この例では、まず独自のヘッダーを追加するため、データからヘッダー行を削除します。次に、上記と同じく最後の2列以外を破棄します。これは、配列`m`を[分配]($scripting/#bindings)し、最後の2項目以外を破棄することで行います。続いて、`count`の文字列を浮動小数点数に変換し、その対数を計算して変数`log`に保存します。最後に、それを2桁に丸め、文字列に変換し、変数`rounded`に保存します。そして、`year`と`rounded`を持つ配列を返し、元の行を置き換えます。表には、対数を値に適用したことを読者に伝える独自のヘッダーを追加しています。次に、上記と同じく平坦化したデータを展開します。

また、表を[縞模様](#fills)、最初の行の下の[水平の罫線](#individual-lines)、全てを右に[整列](#alignment)させ、最初の列を太字にしてスタイル設定しました。リンクをクリックすると、関連するガイドのセクションに移動して方法を確認できます。

## table関数を表ではないものに使う場合 { #table-and-grid }
コンテンツの表形式のレイアウトは、このガイド全体の例で示してきたような密接に関連したデータの行列だけでなく、見せ方の目的にも役立ちます。Typstは、レイアウトと見せ方の目的のためのグリッドと、セルの配置自体が情報を伝える表とを区別しています。

この違いを他のソフトウェアに明確にし、テンプレートで表を大幅にスタイル設定できるようにするため、Typstはグリッドと表のレイアウトに対して2つの関数を持っています。

- このガイド全体で説明されてきた[`table`]($table)関数は、表形式データのためのものです。
- [`grid`]($grid)関数は、見せ方の目的とページレイアウトのためのものです。

両方の要素は同じ動作をし、同じ引数を持ちます。このガイドで表について学んだ全ての内容をグリッドにも適用できます。違いはたった3つです。

- [`table.cell`]($table.cell)、[`table.vline`]($table.vline)、[`table.hline`]($table.hline)の代わりに、[`grid.cell`]($grid.cell)、[`grid.vline`]($grid.vline)、[`grid.hline`]($grid.hline)要素を使う必要があります。
- gridのデフォルトは異なります。デフォルトでは罫線を引かず、セル内の間隔（`inset`）も持ちません。
- `figure`のような要素はgridに反応しません。これは、文書構造に意味的な影響を持たないことが想定されているためです。
