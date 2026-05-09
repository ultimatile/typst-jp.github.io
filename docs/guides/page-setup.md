---
description: |
  Typstでページサイズ、余白、ページ番号を設定するための詳しいガイドです。魅力的で見やすいレイアウトを素早く作成する方法を学びましょう。
---

# ページ設定ガイド
ページ設定は、文書が読み手に与える第一印象の大きな部分を占めます。行の長さ、余白、段組みは[見た目](https://practicaltypography.com/page-margins.html)や[可読性](https://designregression.com/article/line-length-revisited-following-the-research)に影響し、適切なヘッダーやフッターは読み手が文書を簡単にたどっていくのに役立ちます。本ガイドでは、ページ、余白、ヘッダー、フッター、ページ番号をコンテンツに合った形にカスタマイズし、執筆を始められるようになるための方法を紹介します。

Typstでは、各ページに幅、高さ、そして四辺の余白があります。上下の余白にはヘッダーとフッターを含められます。ページ設定は全て[`{page}`]($page)要素のsetルールで制御します。このsetルールで変更を加えると、Typstは新しい空ページがその後に確実に続くようにするため、ページ区切りを挿入する場合があります。したがって、[`{page}`]($page)のsetルールは文書の冒頭やテンプレート内で指定するのが最も望ましいです。

```example
#set rect(
  width: 100%,
  height: 100%,
  inset: 4pt,
)
>>> #set text(6pt)
>>> #set page(margin: auto)

#set page(
  paper: "iso-b7",
  header: rect(fill: aqua)[Header],
  footer: rect(fill: aqua)[Footer],
  number-align: center,
)

#rect(fill: aqua.lighten(40%))
```

この例では、ページコンテンツ、ヘッダー、フッターの寸法を可視化しています。ページコンテンツの大きさは、ページサイズ（ISO B7）から各辺のデフォルト余白を差し引いたものです。上下の余白には、ヘッダーとフッターを表す枠線付きの矩形が描かれています。これらは本文に接することはなく、それぞれの余白の30%だけオフセットされています。このオフセットは、[`header-ascent`]($page.header-ascent)および[`footer-descent`]($page.footer-descent)引数で調整できます。

以降では、よくあるページ設定の要件を例とともに詳しく説明します。

## ページサイズと余白のカスタマイズ { #customize-margins }
TypstのデフォルトのページサイズはA4です。地域や用途に応じて、これを変更したい場合があるでしょう。変更するには、[`{page}`]($page)のsetルールに文字列引数を渡して、よく使われるページサイズを指定します。指定できるオプションには、ISO 216シリーズ全般（例：`"a4"`や`"iso-c2"`）、`"us-legal"`や`"us-letter"`のような米国でよく使われるサイズなどがあります。利用可能な全てのオプションについては、[ページのpaper引数]($page.paper)のリファレンスをご覧ください。

```example
>>> #set page(margin: auto)
#set page("us-letter")

This page likes freedom.
```

任意の寸法でページサイズをカスタマイズしたい場合は、代わりに名前付き引数の[`width`]($page.width)と[`height`]($page.height)を指定できます。

```example
>>> #set page(margin: auto)
#set page(width: 12cm, height: 12cm)

This page is a square.
```

### ページの余白を変更する { #change-margins }
余白は良いタイポグラフィに欠かせない要素です。[タイポグラファーは、可読性のために最適な行長は45から75文字に収まる長さだとしています](http://webtypography.net/2.1.2)。余白と[段組み](#columns)は行幅を決める要素です。デフォルトでは、Typstは文書のページサイズに比例した余白を作ります。余白を独自に設定するには、[`{page}`]($page)のsetルールにある[`margin`]($page.margin)引数を使います。

`margin`引数は、全ての余白を同じ幅に設定したい場合は長さを受け取ります。しかし、各辺ごとに異なる余白を設定したい場合も多いでしょう。その場合は、辞書を渡せます。

```example
#set page(margin: (
  top: 3cm,
  bottom: 2cm,
  x: 1.5cm,
))

#lorem(100)
```

ページ余白の辞書は、各辺（`top`、`bottom`、`left`、`right`）に対応するキーを持てますが、例のように余白辞書の`x`キーを設定することで左右の余白をまとめて制御することもできます。同様に、`y`キーを設定することで上下の余白をまとめて調整できます。

余白辞書で全ての辺の余白を指定しない場合、設定されていない辺については以前の余白がそのまま有効になります。これを防ぎ、残りの余白をまとめて共通のサイズに設定するには、`rest`キーを使えます。例えば、`[#set page(margin: (left: 1.5in, rest: 1in))]`は、左の余白を1.5インチに、残りの余白を1インチに設定します。

### 偶数・奇数ページで異なる余白 { #alternating-margins }
偶数ページと奇数ページで左右の余白を切り替えたい場合があります。例えば、本の背側の余白を外側の余白より広く取りたい場合などです。Typstは、ページが綴じの左右どちら側にあるかを把握しています。この情報を利用して、余白辞書の`inside`キーや`outside`キーを設定できます。`inside`の余白は背の側を、`outside`の余白は綴じられた本の小口側を指します。

```typ
#set page(margin: (inside: 2.5cm, outside: 2cm, y: 1.75cm))
```

Typstは、左から右に書かれる文字（左横書き）の文書は左綴じ、右から左に書かれる文字（右横書き）の本は右綴じだと仮定します。ただし、状況によってはこれを変更する必要があります。最初のページが別のアプリで出力される場合、Typstから見ると綴じが反対になります。また、英語のマンガなど、英語が左から右に書かれるにもかかわらず慣例的に右綴じである本もあります。綴じる側を変更し、`inside`と`outside`の位置を明示的に指定するには、[`{page}`]($page)のsetルールで[`binding`]($page.binding)引数を設定します。

```typ
// Produce a book bound on the right,
// even though it is set in Spanish.
#set text(lang: "es")
#set page(binding: right)
```

`binding`が`left`の場合、奇数ページでは`inside`の余白は左側になり、偶数ページではその逆になります。

## ヘッダーとフッターを追加する { #headers-and-footers }
ヘッダーとフッターは、全てのページの上下の余白に挿入されます。独自のヘッダーやフッターを追加することも、単にページ番号だけを挿入することもできます。

ページ番号以上のものが必要な場合、ヘッダーとフッターを挿入する最良の方法は、[`{page}`]($page)のsetルールにある[`header`]($page.header)引数と[`footer`]($page.footer)引数を使うことです。これらの値には任意のコンテンツを渡せます。

```example
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: [
  _Lisa Strassner's Thesis_
  #h(1fr)
  National Academy of Sciences
])

#lorem(150)
```

ヘッダーは、ページ上端と衝突しないようにデフォルトで下揃えになっています。これを変更するには、ヘッダーを[`{align}`]($align)関数で囲みます。

### 特定のページで異なるヘッダーとフッター { #specific-pages }
ページによってヘッダーやフッターを変えたい場合があります。例えば、表紙にはヘッダーとフッターを置きたくないかもしれません。次の例は、最初のページのヘッダーを条件付きで非表示にする方法を示しています。

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: context {
  if counter(page).get().first() > 1 [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
})

#lorem(150)
```

この例は一見難しそうに見えますが、順を追って説明しましょう。`{context}`キーワードを使うことで、ヘッダーが文書内のどこにいるかに依存することをTypstに伝えています。次に、（コンテキストに依存する）現在の位置でページ[カウンター]($counter)が1より大きいかどうかをTypstに尋ねています。ページカウンターは1から始まるため、これによって1ページ目のヘッダーをスキップしています。カウンターは複数のレベルを持てます。この機能は見出しなどで使われますが、ページカウンターは常に単一のレベルしか持たないため、最初のレベルだけを見れば十分です。

もちろん、この例に`else`を追加すれば、最初のページに別のヘッダーを表示することもできます。

### 特定の要素を含むページのヘッダーとフッターを調整する { #specific-elements }
前節で説明した手法は、Typstのラベルを使ってより高度なタスクをこなすように応用できます。例えば、大きな表があるページではヘッダーを省略して雑然とした印象を抑えることができます。ここでは、表に`<big-table>`という[ラベル]($label)を付け、[クエリシステム]($query)を使って現在のページにそのラベルが存在するかを調べます。

```typ
>>> #set page("a5", margin: (x: 2.5cm, y: 3cm))
#set page(header: context {
  let matches = query(<big-table>)
  let current = counter(page).get()
  let has-table = matches.any(m =>
    counter(page).at(m.location()) == current
  )

  if not has-table [
    _Lisa Strassner's Thesis_
    #h(1fr)
    National Academy of Sciences
  ]
})

#lorem(100)
#pagebreak()

#table(
  columns: 2 * (1fr,),
  [A], [B],
  [C], [D],
) <big-table>
```

ここでは、`<big-table>`ラベルの全ての出現箇所を検索しています。次に、それらの表のいずれも現在の位置のページに存在しないことを確認します。存在しなければ、ヘッダーを出力します。この例では、簡潔にするために変数も使っています。先ほどと同様に、`else`を追加してヘッダーを削除する代わりに別のヘッダーを表示することもできます。

## ページ番号を追加してカスタマイズする { #page-numbers }
ページ番号は、読み手が文書を追ったり参照したりするのに役立ちます。ページ番号を挿入する最も簡単な方法は、[`{page}`]($page)のsetルールにある[`numbering`]($page.numbering)引数を使うことです。これに、ページの番号付け方法を示す[_番号付けパターン_]($numbering.numbering)の文字列を渡せます。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1")

This is a numbered page.
```

上記は、考えうる最も単純な例です。フッターの中央に、アラビア数字のページ番号を1つ追加しています。`"1"`以外の文字を指定すれば、別の数字表記を得られます。例えば、`"i"`は小文字のローマ数字を出力します。数字として解釈されない文字はそのまま出力されます。例えば、ページ番号をダッシュで囲むには次のように記述します。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "— 1 —")

This is a — numbered — page.
```

文字列に2つ目の数字文字を入れることで、総ページ数を追加できます。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(numbering: "1 of 1")

This is one of many numbered pages.
```

ここで渡せる引数についてさらに詳しく知りたい場合は、[`{numbering}`関数のリファレンス]($numbering.numbering)をご覧ください。

ページ番号を右寄せまたは左寄せにする必要がある場合は、[`{page}`]($page)のsetルールにある[`number-align`]($page.number-align)引数を使います。このプロパティでは、偶数ページと奇数ページで配置を切り替えることは現在サポートされていません。これを実現するには、独自のフッターを指定し、ヘッダーやフッターを条件付きで省略する節で説明した方法でページカウンターをクエリする必要があります。

### ページ番号付きのカスタムフッター
フッターに、ページ番号以外のコンテンツを追加したい場合があります。しかし、フッターを一度指定すると、[`{page}`]($page)のsetルールにある[`numbering`]($page.numbering)引数は無視されます。本節では、ページ番号などを含むカスタムフッターを追加する方法を説明します。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: context [
  *American Society of Proceedings*
  #h(1fr)
  #counter(page).display(
    "1/1",
    both: true,
  )
])

This page has a custom footer.
```

まず、左側に強調されたテキストを追加し、行を埋めるために自由なスペースを追加します。次に、`counter(page)`を呼び出してページカウンターを取得し、その`display`関数を使って現在の値を表示します。また、`both`を`{true}`に設定することで、番号付けパターンを現在のページ番号と最終ページ番号の_両方_に適用します。

ページ番号をさらに工夫することもできます。例えば、各ページに円を挿入してみましょう。

```example
>>> #set page("iso-b6", margin: 1.75cm)
#set page(footer: context [
  *Fun Typography Club*
  #h(1fr)
  #let (num,) = counter(page).get()
  #let circles = num * (
    box(circle(
      radius: 2pt,
      fill: navy,
    )),
  )
  #box(
    inset: (bottom: 1pt),
    circles.join(h(1pt))
  )
])

This page has a custom footer.
```

この例では、ページ数を使って[円]($circle)の配列を作成しています。円はブロックなので、そのまま並べると段落区切りができてしまいますが、[ボックス]($box)で囲むことで同じ行に配置できるようにしています。この[配列]($array)の長さは、現在のページ番号によって変わります。

そして、フッターの右側に1ptの間隔を空けて円を挿入します。配列のjoinメソッドは、配列の異なる値を1つの値に[_結合_]($scripting/#blocks)しようと試み、引数で指定した値をその間に挟み込みます。今回の場合、円とその間のスペースを含む単一のコンテンツ値が得られ、これをalign関数で使えます。最後に、もう1つボックスを使って、テキストと円が同じ行を共有できるようにし、[`inset`引数]($box.inset)で円を少し上に持ち上げ、テキストとうまく揃うようにしています。

### ページ番号をリセットしてページをスキップする { #skip-pages }
文書のどこかでページ番号をリセットしたい場合はあるでしょうか。例えば、表紙の後で初めて1ページ目を始めたい、あるいは最終的な印刷物にページを差し込むため、いくつかページ番号をスキップしたい、といった場合です。

ページ番号を変更する正しい方法は、ページ[カウンター]($counter)を操作することです。最も単純な操作は、カウンターを1に戻すことです。

```typ
#counter(page).update(1)
```

この行は、ページカウンターを1にリセットします。これはページの先頭に置く必要があります。そうでないとページ区切りが生じてしまうからです。関数を渡すことで、現在の値からカウンターを更新することもできます。

```typ
#counter(page).update(n => n + 5)
```

この例では、5ページをスキップしています。`n`はページカウンターの現在の値で、`n + 5`は関数の戻り値です。

ページカウンターの値ではなく、実際のページ番号を取得する必要がある場合は、[`here`]($here)関数の戻り値に対して[`page`]($location.page)メソッドを使えます。

```example
#counter(page).update(n => n + 5)

// This returns one even though the
// page counter was incremented by 5.
#context here().page()
```

また、[`page-numbering`]($location.page-numbering)メソッドを使うと、`here`が返すロケーションからページ番号付けパターンを取得できます。

## 段組みを追加する { #columns }
段組みを文書に追加することで、可読性のある行長を保ちつつ、より多くの内容を1ページに収められます。段は、何らかの空白で区切られたテキストの縦のブロックです。この空白は罫間と呼ばれます。

コンテンツを段組みでレイアウトするには、[`{page}`]($page.columns)のsetルールに望む段数を指定するだけです。段の間隔を調整するには、[`columns`関数]($columns)に対するsetルールで`gutter`パラメーターを指定します。

```example
>>> #set page(height: 120pt)
#set page(columns: 2)
#set columns(gutter: 12pt)

#lorem(30)
```

科学論文では、タイトルと要旨は1段で、本文は2段で組まれることがよくあります。この効果を実現するには、Typstの[`place`関数]($place)で`{float: true}`と`{scope: "parent"}`を指定することで、2段組みのレイアウトから一時的に抜け出せます。

```example:single
>>> #set page(height: 180pt)
#set page(columns: 2)
#set par(justify: true)

#place(
  top + center,
  float: true,
  scope: "parent",
  text(1.4em, weight: "bold")[
    Impacts of Odobenidae
  ],
)

== About seals in the wild
#lorem(80)
```

_フロート配置_とは、要素を段やページの上端または下端に押し出し、残りのコンテンツがその間に流し込まれる配置のことです。これは[図表]($figure.placement)にもよく使われます。

### 文書中のどこでも段組みを使う { #columns-anywhere }
入れ子のレイアウト内、例えば矩形の中で段組みを作るには、[`columns`関数]($columns)を直接使えます。ただし、この関数は入れ子のレイアウト内でのみ使うのが本当に望ましいです。ページレベルでは、ページレベルのフロート、脚注、行番号などとの相互作用が良いため、ページのsetルールのほうが望ましいです。

```example
#rect(
  width: 6cm,
  height: 3.5cm,
  columns(2, gutter: 12pt)[
    In the dimly lit gas station,
    a solitary taxi stood silently,
    its yellow paint fading with
    time. Its windows were dark,
    its engine idle, and its tires
    rested on the cold concrete.
  ]
)
```

### 段の高さを揃える
文書の最終ページの段の長さに大きな差があると、不揃いで見栄えの悪いレイアウトになりがちです。そのため、タイポグラファーは最終ページの段の長さを揃えることがよくあります。この効果は段の高さを揃える（balancing columns）と呼ばれます。Typstは段の高さの自動調整にはまだ対応していません。しかし、マークアップの適切な箇所に[`[#colbreak()]`]($colbreak)を置いて、所望の段区切りを手動で作ることで、手作業で段の高さを揃えられます。


## 一時的な変更
別の設定のページを1ページだけ挿入したいだけであれば、ページ設定を上書きする必要はありません。例えば、大きな表を入れるために横向きに反転したページを挿入したり、表紙のために余白や段組みを変更したりしたい場合などです。このような場合は、[`{page}`]($page)を関数として呼び出し、コンテンツを引数として渡し、上書きしたい設定を他の引数として指定できます。これにより、上書きされた設定で必要な数だけ新しいページが挿入され、その上にコンテンツが配置されます。Typstは呼び出し後、setルールで指定されたページ設定に戻ります。

```example
>>> #set page("a6")
#page(flipped: true)[
  = Multiplication table

  #table(
    columns: 5 * (1fr,),
    ..for x in range(1, 10) {
      for y in range(1, 6) {
        (str(x*y),)
      }
    }
  )
]
```
