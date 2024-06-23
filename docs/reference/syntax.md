---
description: |
   Typstの構文に関するコンパクトなリファレンスです。詳細については、言語のマークアップモード、数式モード、およびコードモードを参照してください。
---

# 構文

Typstはマークアップ言語です。
これは、シンプルな構文を使用して一般的なレイアウトタスクを簡単に行えるということです。
Typstの軽量なマークアップ構文は、文書を簡単かつ自動的にスタイリングできるsetルールとshowルールによって補完されています。
これらすべては、組み込み関数およびユーザー定義関数を備えた、緊密に統合されたスクリプト言語によって支えられています。

## モード　{#modes}

Typstには3種類の構文モードがあります:マークアップモード、数学モード、そしてコードモードです。
Typst文書では、マークアップモードがデフォルトであり、数学モードでは数式を書くことができ、コードモードではTypstのスクリプト機能を利用することができます。

以下の表を参照し、いつでも特定のモードに切り替えることができます。

| 新たなモード | 構文                         | 例                              |
| ------------ | ---------------------------- | ------------------------------- |
| コード       | コードの前に`#`を付ける      | `[Number: #(1 + 2)]`            |
| 数学         | 式を`[$..$]`で囲む           | `[$-x$ is the opposite of $x$]` |
| マークアップ | マークアップを`[[..]]`で囲む | `{let name = [*Typst!*]}`       |


一度`#`でコードモードに入ると、途中でマークアップモードや数式モードに切り替えない限り、さらにハッシュを使う必要はありません。

## マークアップ { #markup }

Typstは、最も一般的な文書要素に対する組み込みのマークアップを提供します。
ほとんどの構文要素は、対応する関数のショートカットに過ぎません。
以下の表は、利用可能なすべてのマークアップと、その構文と使用法について詳しく学ぶための最適なページへのリンクを示しています。

| 名称             | 例                       | 参照                                 |
| ---------------- | ------------------------ | ------------------------------------ |
| 段落区切り       | 空行                     | [`parbreak`]($parbreak)              |
| 強調(太字)       | `[*strong*]`             | [`strong`]($strong)                  |
| 強調(イタリック) | `[_emphasis_]`           | [`emph`]($emph)                      |
| rawテキスト      | ``[`print(1)`]``         | [`raw`]($raw)                        |
| リンク           | `[https://typst.app/]`   | [`link`]($link)                      |
| ラベル           | `[<intro>]`              | [`label`]($label)                    |
| 参照             | `[@intro]`               | [`ref`]($ref)                        |
| 見出し           | `[= Heading]`            | [`heading`]($heading)                |
| 箇条書きリスト   | `[- item]`               | [`list`]($list)                      |
| 番号付きリスト   | `[+ item]`               | [`enum`]($enum)                      |
| 用語リスト       | `[/ Term: description]`  | [`terms`]($terms)                    |
| 数学             | `[$x^2$]`                | [Math]($category/math)               |
| 改行             | `[\]`                    | [`linebreak`]($linebreak)            |
| スマートクオート | `['single' or "double"]` | [`smartquote`]($smartquote)          |
| 短縮記号         | `[~, ---]`               | [Symbols]($category/symbols/sym)     |
| コード構文       | `[#rect(width: 1cm)]`    | [Scripting]($scripting/#expressions) |
| 文字エスケープ   | `[Tweet at us \#ad]`     | [Below](#escapes)                    |
| コメント         | `[/* block */, // line]` | [Below](#comments)                   |

## 数学モード { #math }

数学モードは、数式を組版するために使用される特別なマークアップモードです。
数式を `[$]` の文字で囲むことによって、数学モードに入ることができます。
これはマークアップモードとコードモードの両方で機能します。
数式が少なくとも一つのスペースで始まり終わる場合、その数式は独自のブロックに組版されます（例:`[$ x^2 $]`）。
インライン数式は、スペースを省略することで作成できます（例:`[$x^2$]`）。
以下に、数式モードに特有の構文の概要を示します：

| 名称                    | 例                      | 参照                                 |
| ----------------------- | ----------------------- | ------------------------------------ |
| インライン数式          | `[$x^2$]`               | [Math]($category/math)               |
| ブロック数式            | `[$ x^2 $]`             | [Math]($category/math)               |
| 下付き添え字            | `[$x_1$]`               | [`attach`]($category/math/attach)    |
| 上付き添え字            | `[$x^2$]`               | [`attach`]($category/math/attach)    |
| 分数                    | `[$1 + (a+b)/5$]`       | [`frac`]($math.frac)                 |
| 改行                    | `[$x \ y$]`             | [`linebreak`]($linebreak)            |
| 揃え位置                | `[$x &= 2 \ &= 3$]`     | [Math]($category/math)               |
| 変数アクセス            | `[$#x$, $pi$]`          | [Math]($category/math)               |
| フィールドアクセス      | `[$arrow.r.long$]`      | [Scripting]($scripting/#fields)      |
| 暗黙の乗算              | `[$x y$]`               | [Math]($category/math)               |
| 短縮記号                | `[$->, !=$]`            | [Symbols]($category/symbols/sym)     |
| 数式内のテキスト/文字列 | `[$a "is natural"$]`    | [Math]($category/math)               |
| 数学関数呼び出し        | `[$floor(x)$]`          | [Math]($category/math)               |
| コード構文              | `[$#rect(width: 1cm)$]` | [Scripting]($scripting/#expressions) |
| 文字エスケープ          | `[$x\^2$]`              | [Below](#escapes)                    |
| コメント                | `[$/* comment */$]`     | [Below](#comments)                   |

## コードモード { #code }

コードブロックや式の中では、新しい式は先頭に`#`を付けずに始めることができます。
多くの構文要素は式に特有のものです。
以下に、コードモードで利用可能なすべての構文の一覧表を示します:


| 名称                 | 例                          | 参照                              |
| -------------------- | ----------------------------- | ------------------------------------- |
| 变量访问             | `{x}`                         | [Scripting]($scripting/#blocks)       |
| 字面常量             | `{1pt, "hey"}`                | [Scripting]($scripting/#expressions)  |
| 代码块               | `{{ let x = 1; x + 2 }}`      | [Scripting]($scripting/#blocks)       |
| 文档内容块           | `{[*Hello*]}`                 | [Scripting]($scripting/#blocks)       |
| 括号表达式           | `{(1 + 2)}`                   | [Scripting]($scripting/#blocks)       |
| 数组                 | `{(1, 2, 3)}`                 | [Array]($array)                       |
| 字典                 | `{(a: "hi", b: 2)}`           | [Dictionary]($dictionary)             |
| 一元运算符           | `{-x}`                        | [Scripting]($scripting/#operators)    |
| 二元运算符           | `{x + y}`                     | [Scripting]($scripting/#operators)    |
| 赋值                 | `{x = 1}`                     | [Scripting]($scripting/#operators)    |
| 字段访问             | `{x.y}`                       | [Scripting]($scripting/#fields)       |
| 方法调用             | `{x.flatten()}`               | [Scripting]($scripting/#methods)      |
| 函数调用             | `{min(x, y)}`                 | [Function]($function)                 |
| 匿名函数             | `{(x, y) => x + y}`           | [Function]($function)                 |
| let 绑定             | `{let x = 1}`                 | [Scripting]($scripting/#bindings)     |
| 命名函数             | `{let f(x) = 2 * x}`          | [Function]($function)                 |
| set 规则             | `{set text(14pt)}`            | [Styling]($styling/#set-rules)        |
| set-if 规则          | `{set text(..) if .. }`       | [Styling]($styling/#set-rules)        |
| show-set 规则        | `{show par: set block(..)}`   | [Styling]($styling/#show-rules)       |
| 函数式 show 规则     | `{show raw: it => {..}}`      | [Styling]($styling/#show-rules)       |
| show-everything 规则 | `{show: columns.with(2)}`     | [Styling]($styling/#show-rules)       |
| 条件表语句           | `{if x == 1 {..} else {..}}`  | [Scripting]($scripting/#conditionals) |
| for 循环             | `{for x in (1, 2, 3) {..}}`   | [Scripting]($scripting/#loops)        |
| while 循环           | `{while x < 10 {..}}`         | [Scripting]($scripting/#loops)        |
| 循环流程控制         | `{break, continue}`           | [Scripting]($scripting/#loops)        |
| 函数返回             | `{return x}`                  | [Function]($function)                 |
| include 模块         | `{include "bar.typ"}`         | [Scripting]($scripting/#modules)      |
| import 模块          | `{import "bar.typ"}`          | [Scripting]($scripting/#modules)      |
| 从模块内 import 条目 | `{import "bar.typ": a, b, c}` | [Scripting]($scripting/#modules)      |
| 注释                 | `[/* block */, // line]`      | [Below](#comments)                    |

## 注释 { #comments }

Typst 会忽略注释，最终生成的文档不会包含它们。
它们通常被用于剔除旧版本，或者添加标注说明。
如果一行开头是 `//`，这行就会被认为是注释：

```example
// our data barely supports
// this claim

We show with $p < 0.05$
that the difference is
significant.
```

也可以通过 `/*` 和 `*/` 来包裹注释，这种方式，注释可以分布于多行：

```example
Our study design is as follows:
/* Somebody write this up:
   - 1000 participants.
   - 2x2 data design. */
```

## 转义序列 { #escapes }

转义序列可以用来插入难于输入的特殊字符，或者 Typst 内有特殊含义的字符。
前缀一个反斜杠转义一个字符，转移序列如果是十六进制，
比如 `[\u{1f600}]`，就会插入一个 Unicode 码点。
这些类型的转义序列也作用于[字符串]($str)中。

```example
I got an ice cream for
\$1.50! \u{1f600}
```