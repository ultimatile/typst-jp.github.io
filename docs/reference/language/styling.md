---
description: Typst で文書のスタイル設定をするために必要な概念
---

# スタイル設定

Typstには柔軟なスタイル設定機能を持ち、出力される文書に対して自動的に任意のスタイル設定を適用します。
_setルール_では要素の基本プロパティを設定できます。
しかし、やりたいこと全てに対応するプロパティがあらかじめ実装されているとは限りません。
このため、Typstは要素の外観を完全に再定義できる_showルール_もサポートしています。

## setルール { #set-rules }

setルールを使うと、要素の外観をカスタマイズできます。
これらは、`{set}`キーワード（マークアップでは`[#set]`）を前に置いた[要素関数]($function/#element-functions)への[関数呼び出し]($function)として記述されます。
setルールに指定できるのは、その関数のオプションのパラメーターだけです。
どのパラメーターがオプションであるかは、各関数のドキュメントを参照してください。
以下の例では、2つのsetルールを使って、[フォント]($text.font)と[見出し番号]($heading.numbering)を変更しています。

```example
#set heading(numbering: "I.")
#set text(
  font: "New Computer Modern"
)

= Introduction
With set rules, you can style
your document.
```

setルールは、そのまま記述するとファイルの最後まで適用されます。
ブロックの中にネストすると、そのブロックの終わりまで適用されます。
ブロックを使えば、setルールの効果を指定した部分に限定できます。
以下では、contentブロックを用いてスコープすることで、特定のリストのスタイルのみを変更しています。

```example
This list is affected: #[
  #set list(marker: [--])
  - Dash
]

This one is not:
- Bullet
```

ときには、setルールを条件付きで設定したい場合もあるでしょう。
その場合には_set-if_ルールを使用します。

```example
#let task(body, critical: false) = {
  set text(red) if critical
  [- #body]
}

#task(critical: true)[Food today?]
#task(critical: false)[Work deadline]
```

## showルール { #show-rules }

showルールを使えば、特定の種類の要素の外観を詳細に設定できます。
showルールの基本的な記述方法は、show-setルールです。
`{show}` キーワードの後に [セレクター]($selector)、コロン、setルールと続けて記述します。
セレクターの基本的な記述方法は [要素関数]($function/#element-functions)を置くことであり、setルールは選択された要素にのみ適用されます。
下の例では、見出しは紺色になり、他のテキストは黒色のままです。

```example
#show heading: set text(navy)

= This is navy-blue
But this stays black.
```

show-setルールを使うと、異なる関数のプロパティを組み合わせて、さまざまな効果を実現できます。
しかし、依然として表現可能なものはTypstで事前に定義されたものに限定されます。
最大限の柔軟性を求める場合は、要素のフォーマット方法を一から定義する_変換型_showルールを記述できます。
このようなshowルールを記述するには、コロン以降のsetルールを任意の[関数]($function)に置き換えます。
この関数は対象の要素を受け取り、任意のコンテンツを返すことができます。
この関数は、[名前なし関数構文]($function/#unnamed)を用いて`{it => ..}`のようにインラインで定義されることがよくあります。
関数の引数は慣習的に`it`という名前にすることが多いです。

関数に渡される要素で利用可能な[フィールド]($scripting/#fields)は、それぞれの要素関数のパラメーターに一致します。
以下では、ファンタジー百科事典向けに見出しを整形するshowルールを定義しています。

このshowルール自体は、タイトルの周りにチルダ文字を追加し（これは、非改行空白を意味してしまうのを避けるためバックスラッシュでエスケープする必要があります）、タイトルを斜体で強調表示し、その後ろに見出しカウンターを表示します。

この例では、中央揃えと別のフォントも適用したいと考えました。
これらのsetルールを既存のshowルールの中に追加してもよかったのですが、代わりに独立したshow-setルールとして追加しました。
こうすることで、これらのルールは文書内のあとに続くshow-setルールによって上書き可能なまま保たれ、スタイル設定を構成可能なまま維持できるため、これは推奨される方法です。
一方、変換型showルールの中に書かれたsetルールは、後から上書きすることができません。

```example
#set heading(numbering: "(I)")
#show heading: set align(center)
#show heading: set text(font: "Inria Serif")
#show heading: it => block[
  \~
  #emph(it.body)
  #counter(heading).display(it.numbering)
  \~
]

= Dragon
With a base health of 15, the dragon is the most
powerful creature.

= Manticore
While less powerful than the dragon, the manticore
gets extra style points.
```

setルールと同様に、showルールは、現在のブロック内またはファイルの終わりまで有効です。

関数の代わりに、showルールのコロン右側は、要素に直接置換されるべきリテラル文字列またはコンテンツブロックを取ることもできます。
またshowルールのコロン左側は、以下に示すように、変換を適用する対象を定義する_セレクター_を受け取ることができます。

- **全て：** `{show: rest => ..}` \
  showルール以降の全てを変換する。
  個別の関数呼び出しでラップすることなく、複雑なレイアウトを文書全体に適用するのに便利です。

- **文字列：** `{show "Text": ..}` \
  設定した文字列に対して、スタイル変更や文字の置き換えを行います。

- **正規表現：** `{show regex("\w+"): ..}` \
  正規表現にマッチする文字列に対して、スタイル変更や文字の置き換えを行います。
  正規表現については[regex 関数]($regex)を参照してください。

- **関数やフィールド：** `{show heading.where(level: 1): ..}` \
  指定されたフィールドを持つ要素のみを変換します。
  例えば、レベル1の見出しのスタイルだけを変更したい場合などに有効です。

- **ラベル：** `{show <intro>: ..}` \
  指定されたラベルを持つ要素に対して適用する。
  ラベルについては[labelタイプ]($label)を参照してください。

```example
#show "Project": smallcaps
#show "badly": "great"

We started Project in 2019
and are still working on it.
Project is progressing badly.
```
