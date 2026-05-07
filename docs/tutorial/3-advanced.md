---
description: Typstチュートリアル
---

# 高度なスタイリング
このチュートリアルの前の2つの章では、Typstで文書を書く方法とその書式を変更する方法を学びました。
それらの章を通して書いたレポートが優れた評価を得たため、指導教員はそれをもとに学会論文を書いてほしいと考えています！
もちろん、論文は学会のスタイルガイドに従わなければなりません。
どうすればそれを達成できるか見てみましょう。

始める前に、チームを作成して、そのチームに教員を招待して追加しましょう。
まず、エディターの左上にある戻るアイコンでアプリのダッシュボードに戻ります。
次に、左のツールバーのプラスアイコンを選択し、チームを作成します。
最後に、新しいチームをクリックし、チーム名の横にあるmanage teamをクリックして設定に進みます。
これで教員をメールで招待することができます。

![The team settings](3-advanced-team-settings.png)

次に、プロジェクトをチームに移動します。
プロジェクトを開き、左のツールバーの歯車アイコンを選んで設定に行き、Ownerのドロップダウンから新しいチームを選択します。
変更を保存するのを忘れないでください！

あなたの教員もプロジェクトを編集することができ、お互いにリアルタイムで変更を確認できます。
公式の[Discordサーバー](https://discord.gg/2uDybryKPe)に参加して他のユーザーを見つけ、一緒にチームを組んでみることも可能です！

## 学会ガイドライン { #guidelines }
レイアウトのガイドラインは学会ウェブサイトに掲載されております。
ここでは以下の条件であった場合を考えましょう。

- フォントは11ptのセリフ体
- タイトルは17ptで太字
- アブストラクトは1段組みで本文は2段組み
- アブストラクトは中央揃え
- 本文は両端揃え
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示
- 第2レベルの見出しは斜体で、本文と同じ大きさ
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載

これらのうち、多くの項目については既に対応方法を知っていますが、いくつかについては新しい記法を学ぶ必要があります。

## setルール { #set-rules }
まず、文書のsetルールを書くことから始めましょう。

```example
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(right)[
    A Fluid Dynamic Model for
    Glacier Flow
  ],
  numbering: "1",
)
#set par(justify: true)
#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#lorem(600)
```

ここで行われていることの大半は、すでに分かりでしょう。
テキストサイズを`{11pt}`に、フォントをLibertinus Serifに設定しています。
また、段落の両端揃えを有効にし、ページサイズをUSレターとしています。

ここで、`header`は新しい引数で、各ページの上部の余白に置くコンテンツを設定できます。
ヘッダーには、学会のスタイルガイドで要求されているように、論文のタイトルを指定します。
`align`関数を用いてそのテキストを右寄せにします。

最後に `numbering` 引数について説明します。
ここでは、ページ番号の付け方を定義する[numbering pattern]($numbering)を指定できます。
例えば`{"1"}`と設定すると、Typstは素のページ番号のみを表示します。
また`{"(1/1)"}`と設定すると、カッコで囲まれた現在のページと総ページ数が表示されるでしょう。
さらに、カスタム関数を用意して完全に好みの書式にすることも可能です。

## タイトルとアブストラクトの作成 { #title-and-abstract }
それでは、タイトルとアブストラクトを追加しましょう。
Typstには[`title`]($title)関数が用意されています。まずはタイトルを引数として与えるところから始めましょう。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

タイトルがすでに太字になっており、周囲にいくらかの空白があることがわかります。
しかし、左揃えになっており、ちょうど17ptの大きさでもありません。
そのため、外観を調整する必要があります。
title関数にはフォントやテキストサイズを設定するための引数はありません。
代わりに、これらのプロパティは`text`関数と`align`関数で定義されています。

<div class="info-box">

`title`関数が挿入したものと、等号で作成した見出しとの違いは何でしょうか？

見出しは第1レベルのものでも文書中に複数回現れる可能性がありますが、タイトルは通常文書の冒頭に1回だけ現れます。
両者を区別することで、Typstはスクリーンリーダーなどの支援技術のユーザーに対して文書をアクセシブルにできます。
</div>

ある要素のプロパティを、別の種類の要素の中で設定したい場合、show-setルールを使用できます。
まず`show`を使って、設定したい要素を選択します。
これを_セレクター_と呼びます。
続けてコロンを書き、セレクターに一致する要素に適用すべきsetルールを記述します。
まとめると、構文は以下のようになります。

```typ
#show your-selector: set some-element(/* ... */)
```

思い出してみましょう。タイトルを中央揃えにし、17ptの大きさにしたいのでした。
そのため、以下のように2つのshow-setルールが必要です。

- セレクターを`title`、ルールを`{set text(size: 17pt)}`としたもの
- セレクターを`title`、ルールを`{set align(center)}`としたもの

これで例は次のようになります。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
#show title: set text(size: 17pt)
#show title: set align(center)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]
```

よさそうですね。著者リストも追加しましょう。
今回は指導教員と一緒にこの論文を書いているので、自分と教員の名前を加えます。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]

#grid(
  columns: (1fr, 1fr),
  align(center)[
    Therese Tungsten \
    Artos Institute \
    #link("mailto:tung@artos.edu")
  ],
  align(center)[
    Dr. John Doe \
    Artos Institute \
    #link("mailto:doe@artos.edu")
  ]
)
```

著者情報が記載された2つのブロックが隣り合わせにレイアウトされています。
このレイアウトを作るために[`grid`]($grid)関数を使っています。
これにより、各列の大きさや、どのコンテンツをどのセルに入れるかを正確に制御することができます。
`columns`引数には、[相対長さ]($relative)または[割合]($fraction)の配列を渡します。
この場合、2つの等しい割合のサイズを渡し、使用可能なスペースを2つの等しい列に分割するように指示します。
次に、grid関数に2つのコンテンツ引数を渡しました。
ひとつは主著者であるあなたの情報で、もうひとつは指導教員の情報です。
ここでも `align` 関数を使用して、コンテンツを列の中央に配置しています。
grid関数はセルを指定するコンテンツ引数を任意の数で受け取れます。
行は自動的に追加されますが、`rows`引数で手動でサイズを指定することも可能です。

著者とタイトルを見ると、両者の間隔が少し近すぎます。
これは、タイトルの下の間隔を設定するための別のshow-setルールを使うことで解決できます。
タイトルやグリッド、段落、その他Typstがページの上から下に配置する全ての要素は_ブロック_と呼ばれます。
各ブロックは[`block`]($block)関数で制御されます。
この関数は、ブロックどうしの距離やブロックが改ページを含むことができるかどうかなどの挙動を制御します。
つまり、タイトルを選択し、ブロックの間隔を設定するもう1つのshow-setルールを記述できます。

```example
>>> #set page(width: 300pt, margin: 30pt)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show title: set text(size: 17pt)
#show title: set align(center)
#show title: set block(below: 1.2em)

#title[
  A Fluid Dynamic Model
  for Glacier Flow
]

#grid(
<<<   // ...
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
)
```

このshow-setルールにより、タイトルの下の間隔を上書きしました。
`em`単位を使っています。これにより、長さをフォントサイズの倍数として表現できます。
ここでは、タイトルと著者リストの間隔をフォントサイズのちょうど1.2倍にするために使用しました。

それでは、アブストラクトを追加しましょう。
学会は、アブストラクトを両端揃えなしで中央揃えに設定することを望んでいることを思い出してください。

```example:0,0,612,317.5
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(right + horizon)[
>>>     A Fluid Dynamic Model for
>>>     Glacier Flow
>>>   ],
>>>   numbering: "1",
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title[
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ]
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
<<< ...

#align(center)[
  #set par(justify: false)
  *Abstract* \
  #lorem(80)
]
>>> #lorem(600)
```
できました！特筆すべき点は、`align`のコンテンツ引数の中にあるsetルールを使って、アブストラクトの両端揃えをオフにしたことです。
これは、最初のsetルールの後に指定されたにもかかわらず、文書の残りの部分には影響しません。
コンテンツ・ブロック内で設定されたものは、そのブロック内のコンテンツにのみ影響します。

もう1つの調整として、ヘッダーとtitle要素の引数におけるタイトルの重複を取り除くという案が考えられます。
両者がタイトルを共有しているため、文書のメタデータを保持するための場所にタイトルを格納できると便利でしょう。
そうすると、両方の場所からタイトルを取得する方法も必要になります。
前者については、`document`要素が役立ちます。
これをsetルールで使うことで、タイトル、説明、キーワードといった文書のメタデータを格納できます。

```typ
#set document(title: [A Fluid Dynamic Model for Glacier Flow])
```

PDFをエクスポートする際、ここで設定したタイトルはPDFリーダーのタイトルバーに表示されます。
オペレーティングシステムもこのタイトルを使って検索でファイルを見つけられるようにします。
さらに、これは文書のアクセシビリティ向上に寄与し、アクセシビリティに重点を置いたPDF規格であるPDF/UAに準拠する場合には必須です。

次に、メインのタイトルとヘッダーで設定した値を取得する方法が必要です。
`title`関数は`document`要素と連携するように設計されているため、引数なしで呼び出すとタイトルを表示するだけになります。
ヘッダーの場合は、もう少し明示的に指定する必要があります。
Typstにはそこへタイトルを挿入したいことを知る術がないため、手動でそう指示する必要があります。

_コンテキスト_を使えば、これまでに要素に設定した値の内容を取得できます。
`{context}`キーワードを使うと、document要素のtitleプロパティを含め、任意の要素の任意のプロパティにアクセスできます。
使い方は次のようになります。

```example:single
#set document(title: [
  A Fluid Dynamic Model
  for Glacier Flow
])

<<< ...

#set page(
>>> "us-letter",
>>> margin: auto,
  header: align(
    right + horizon,
    // Retrieve the document
    // element's title property.
    context document.title,
  ),
<<<   ...
>>> numbering: "1",
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

>>> #show title: set text(size: 17pt)
>>>
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
#title()

<<< ...

>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   align(center)[
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   align(center)[
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
>>>
>>> #align(center)[
>>>   #set par(justify: false)
>>>   *Abstract* \
>>>   #lorem(80)
>>> ]
>>>
>>> #lorem(600)
```

まず、title関数を空の丸括弧で呼び出していることに注目してください。
引数が渡されなかったため、上でdocument要素に設定したものがデフォルトとして使用されます。
空の丸括弧と空の角括弧の違いは重要です。空の丸括弧は何も渡していないことを示しますが、空の角括弧は1つの引数（空のコンテンツブロック）を渡していることを意味します。
そのように呼び出した場合、タイトルには表示されるコンテンツがなくなります。

次に、ヘッダーに注目してください。
角括弧でタイトルを記述する代わりに、contextキーワードを使ってドキュメントのタイトルにアクセスしました。
これにより、上で設定したものがそのまま挿入されます。
contextの役割はプロパティへのアクセスに限られません。
contextを使うことで、ある要素が文書中に存在するかどうかをチェックしたり、他の要素の物理的な寸法を測定したりすることなどができます。
contextを使うと、エンドユーザーの好みに応じて反応する強力なテンプレートを構築できます。

<div class="info-box">

<details>
<summary>
なぜ要素のプロパティにアクセスするのにcontextキーワードが必要なのでしょうか？
</summary>

通常、変数にアクセスするときには、その値が何であるかを正確に把握しています。

- 変数はTypstに組み込まれた定数（例えば`[#sym.pi]`）かもしれません
- 変数は引数で定義されるかもしれません
- 変数は現在のスコープで定義あるいは上書きされるかもしれません

しかし、それだけでは不十分な場合もあります。
このチュートリアルの章では、タイトルを含むページヘッダーを挿入しました。
ヘッダーにはコンテンツを1つだけ渡しているにもかかわらず、ページごとに異なるヘッダーが必要になる場合があります。
例えば、章の名前を表示したり、ページ番号を使用したりしたいことがあるでしょう。
contextを使うと、Typstに対して、挿入された場所を見て、直近の見出しや現在のページ番号、その他必要なものを探してから処理を行うように指示する1つのcontextブロックを書けます。
つまり、同じcontextブロックでも、異なるページに挿入されると異なる出力を生成することができるのです。

詳細については、このチュートリアルを完了した後で[contextのドキュメント]($context)を参照してください。
</details>
</div>

## 段組みと見出しの追加 { #columns-and-headings }
上の論文は、残念ながら文字が単調にぎっしり詰まっていて読みにくい見た目をしています。
これを修正するために、見出しを追加し、2段組のレイアウトに変更してみましょう。
幸いなことに、setルールで`page`に`column`引数を追加することで簡単に行えます。

引数リストに`{columns: 2}`を加えることで、文書全体を2段組みとなります。
しかし、これではタイトルと著者、アブストラクトにも影響が出てしまいます。
それらを1段組みのままに維持するためには、[`{place}`]($place)関数を呼び出して囲みましょう。
place関数は引数として配置とコンテンツを受け取ります。
オプション引数である`{scope}`引数を使えば、現在の段組みとその親（ページ）のどちらに対して配置するかを決めることが可能です。
これらに加えて、もうひとつ設定することがあります。
オプション引数がない場合、`{place}`はそのコンテンツを文書の流れから外し、他のレイアウトに影響を与えることなく、他のコンテンツの上に配置します。

```example
#place(
  top + center,
  rect(fill: black),
)
#lorem(30)
```

もしここで`{place}`を使わなければ、黒塗りの長方形は独立した行になるはずですが、
`{place}`を使うと、それに続く数行のテキストの上に重なります。
同様に、テキスト側もこの長方形がないかのように振る舞います。
この動作を変更するには、引数`{float: true}`を渡してください。
これにより`{place}`でページの上部または下部に配置されたアイテムが、他のコンテンツと重ならないように設定できます。

```example:single
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
#set page(
>>> margin: auto,
  paper: "us-letter",
  header: align(
    right + horizon,
    context document.title,
  ),
  numbering: "1",
  columns: 2,
)
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
>>> #show title: set text(size: 17pt)
>>> #show title: set align(center)
>>> #show title: set block(below: 1.2em)
>>>
>>> #title()
>>>
>>> #grid(
>>>   columns: (1fr, 1fr),
>>>   [
>>>     Therese Tungsten \
>>>     Artos Institute \
>>>     #link("mailto:tung@artos.edu")
>>>   ],
>>>   [
>>>     Dr. John Doe \
>>>     Artos Institute \
>>>     #link("mailto:doe@artos.edu")
>>>   ]
>>> )
<<<   ...

  #par(justify: false)[
    *Abstract* \
    #lorem(80)
  ]
]

= Introduction
#lorem(300)

= Related Work
#lorem(200)
```

この例では、`{place}` 関数の `clearance` 引数も使用しています。
これにより、[`{v}`]($v)関数を使用する代わりに、本文との間にスペースを設けています。
また、コンテンツはcenter引数を継承しているため、各パーツごとに行っていた明示的な `{align(center, ...)}` 呼び出しも削除できます。

最後に見出しのスタイルの設定をしましょう。
ガイドラインに従うために、見出しは中央揃えにして、小さな大文字を使わなければなりません。
`heading`関数はそのような設定を提供していないため、独自の見出しshowルールを書く必要があります。

- A show-set rule to make headings center-aligned
- A show-set rule to make headings 13pt large and use the regular weight
- A show rule to wrap the headings in a call to the `smallcaps` function

```example:50,250,265,270
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
#show heading: set align(center)
#show heading: set text(
  size: 13pt,
  weight: "regular",
)
#show heading: smallcaps

<<< ...
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]

= Introduction
<<< ...
>>> #lorem(35)

== Motivation
<<< ...
>>> #lorem(45)
```

これは素晴らしい！
全ての見出しに適用されるshowルールを使用しました。
最後のshowルールでは、見出し全体に`smallcaps`関数を適用しました。
次の例で見るように、カスタムルールを提供して見出しのデフォルトの外観を完全に上書きすることもできます。

残る唯一の問題は、全ての見出しが同じ見た目になってしまっていることです。
「Motivation」や「Problem Statement」のサブセクションは、斜体のラン・イン見出しであるべきですが、現状ではセクション見出しと区別できません。
これは、showルールに`where`セレクターを使うことで解決できます。
これは見出し（およびその他の要素）に対して呼び出せる[メソッド]($scripting/#methods)で、要素をプロパティでフィルタリングできます。
これを使って、セクション見出しとサブセクション見出しを区別できます。

```example:50,250,265,245
>>> #set document(title: [
>>>   A Fluid Dynamic Model
>>>   for Glacier Flow
>>> ])
>>>
>>> #set page(
>>>   "us-letter",
>>>   margin: auto,
>>>   header: align(
>>>     right + horizon,
>>>     context document.title,
>>>   ),
>>>   numbering: "1",
>>>   columns: 2,
>>> )
>>> #set par(justify: true)
>>> #set text(font: "Libertinus Serif", 11pt)
>>>
#show heading.where(level: 1): set align(center)
#show heading.where(level: 1): set text(size: 13pt, weight: "regular")
#show heading.where(level: 1): smallcaps

#show heading.where(level: 2): set text(
  size: 11pt,
  weight: "regular",
  style: "italic",
)
#show heading.where(level: 2): it => {
  it.body + [.]
}
>>>
>>> #place(
>>>   top + center,
>>>   float: true,
>>>   scope: "parent",
>>>   clearance: 2em,
>>> )[
>>>   #show title: set text(size: 17pt)
>>>   #show title: set align(center)
>>>   #show title: set block(below: 1.2em)
>>>
>>>   #title()
>>>
>>>   #grid(
>>>     columns: (1fr, 1fr),
>>>     [
>>>       Therese Tungsten \
>>>       Artos Institute \
>>>       #link("mailto:tung@artos.edu")
>>>     ],
>>>     [
>>>       Dr. John Doe \
>>>       Artos Institute \
>>>       #link("mailto:doe@artos.edu")
>>>     ]
>>>   )
>>>
>>>   #par(justify: false)[
>>>     *Abstract* \
>>>     #lorem(80)
>>>   ]
>>> ]
>>>
>>> = Introduction
>>> #lorem(35)
>>>
>>> == Motivation
>>> #lorem(45)
```

この例ではまず、`{.where(level: 1)}`を使ってセレクターをより具体的にすることで、これまでのルールを第1レベルの見出しに限定しました。
次に、第2レベルの見出しのためのshow-setルールを追加しました。
最後に、カスタム関数を伴うshowルールが必要になります。
見出しはデフォルトでコンテンツをブロックで囲みます。
これにより、見出しが独立した行になる効果が生まれます。
しかし、ここでは見出しを本文に続けて表示したいので、このブロックを取り除くために独自のshowルールを提供する必要があります。

このルールには、見出しを引数として受け取る関数を渡します。
この引数は慣習的に`it`と呼ばれますが、別の名前にすることもできます。
この引数はコンテンツとして利用でき、そのまま使うとデフォルトの見出し全体が表示されます。
別の方法として、独自の見出しを構築したい場合は、`body`、`numbering`、`level`などのフィールドを利用してカスタムな見た目を組み立てられます。
ここでは見出しの本文の末尾にピリオドを付けて表示するだけにとどめ、組み込みのshowルールが生成するブロックは省略しています。
このshowルールでは`it.numbering`を明示的に使用していないため、この見出しは見出しの番号付けなどに関するsetルールに反応しなくなる点に注意してください。
このようなshowルールを書きつつ文書をカスタマイズ可能なまま保ちたい場合には、これらのフィールドを考慮する必要があります。

これは素晴らしい！
第1レベルと第2レベルの見出しにそれぞれ選択的に適用される2つのshowルールを書きました。
`where`セレクタを使用して、見出しをレベルでフィルタリングしました。
そして、サブセクションの見出しを本文と改行せずにレンダリングしました。
また、サブセクションの見出しの最後にピリオドを自動的に追加してます。

ここで、学会のスタイルガイドを確認しましょう。

- フォントは11ptのセリフ体 ✓
- タイトルは17ptで太字 ✓
- アブストラクトは1段組みで本文は2段組み ✓
- アブストラクトは中央揃え ✓
- 本文は両端揃え ✓
- 第1レベルのセクションの見出しは13ptで中央に配置し、小さな大文字で表示 ✓
- 第2レベルの見出しは斜体で、本文と同じ大きさ ✓
- ページはUSレターサイズとし、下中央にページ番号を付け、各ページの右上に論文のタイトルを記載 ✓

これで、全てのスタイルに準拠し、論文を学会に提出できます！完成した論文は次のようになっています。
<img
  src="3-advanced-paper.png"
  alt="The finished paper"
  style="box-shadow: 0 4px 12px rgb(89 85 101 / 20%); width: 500px; max-width: 100%; display: block; margin: 24px auto;"
>

## まとめ
このセクションでは、ヘッダーとフッターの作成方法、関数とスコープを使用してローカルにスタイルをオーバーライドする方法、[`grid`]関数を使用してより複雑なレイアウトを作成する方法、個々の関数と文書全体のshowルールを記述する方法を学びました。
また、contextを使って要素のプロパティにアクセスする方法も学びました。
また、[`where`セレクタ]($styling/#show-rules)を使用して、見出しをそのレベルによってフィルタリングする方法も学びました。

結果として論文は大成功でした！
あなたはその学会にて同じ志を持つ研究者にたくさん出会い、来年同じ学会で発表したいプロジェクトを計画しています。
その際に、同じスタイルガイドを使って新しい論文を書く必要があるため、あなたやあなたのチームのために、時間を節約できるテンプレートを作りたいと思うのではないでしょうか？

次のセクションでは、複数の文書で再利用できるテンプレートの作成方法を学びます。
これはより高度なトピックですので、今すぐには手が出せないという方は、後ほどお気軽にお越しください。
