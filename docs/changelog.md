---
description: |
  Learn what has changed in the latest Typst releases and move your documents
  forward.
---

# Changelog
## Version 0.10.0 (December 4, 2023) { #v0.10.0 }
- Bibliography management
  - Added support for citation collapsing (e.g. `[[1]-[3]]` instead of
    `[[1], [2], [3]]`) if requested by a CSL style
  - Fixed bug where an additional space would appear after a group of citations
  - Fixed link show rules for links in the bibliography
  - Fixed show-set rules on citations
  - Fixed bibliography-related crashes that happened on some systems
  - Corrected name of the GB/T 7714 family of styles from 7114 to 7714
  - Fixed missing title in some bibliography styles
  - Fixed printing of volumes in some styles
  - Fixed delimiter order for contributors in some styles (e.g. APA)
  - Fixed behavior of alphanumeric style
  - Fixed multiple bugs with GB/T 7714 style
  - Fixed escaping in Hayagriva values
  - Fixed crashes with empty dates in Hayagriva files
  - Fixed bug with spacing around math blocks
  - Fixed title case formatting after verbatim text and apostrophes
  - Page ranges in `.bib` files can now be arbitrary strings
  - Multi-line values in `.bib` files are now parsed correctly
  - Entry keys in `.bib` files now allow more characters
  - Fixed error message for empty dates in `.bib` files
  - Added support for years of lengths other than 4 without leading zeros in
    `.bib` files
  - More LaTeX commands (e.g. for quotes) are now respected in `.bib` files

- Visualization
  - Added support for [patterns]($pattern) as fills and strokes
  - The `alpha` parameter of the [`components`]($color.components) function on
    colors is now a named parameter (**Breaking change**)
  - Added support for the [Oklch]($color.oklch) color space
  - Improved conversions between colors in different color spaces
  - Removed restrictions on [Oklab]($color.oklab) chroma component
  - Fixed [clipping]($block.clip) on blocks and boxes without a stroke
  - Fixed bug with [gradients]($gradient) on math
  - Fixed bug with gradient rotation on text
  - Fixed bug with gradient colors in PDF
  - Fixed relative base of Oklab chroma ratios
  - Fixed Oklab color negation

- Text and Layout
  - CJK text can now be emphasized with the `*` and `_` syntax even when there
    are no spaces
  - Added basic i18n for Greek and Estonian
  - Improved default [figure caption separator]($figure.caption.separator) for
    Chinese, French, and Russian
  - Changed default [figure supplement]($figure.supplement) for Russian to
    short form
  - Fixed [CJK-Latin-spacing]($text.cjk-latin-spacing) before line breaks
    and in [`locate`]($locate) calls
  - Fixed line breaking at the end of links

- Math
  - Added [`mid`]($math.mid) function for scaling a delimiter up to the height
    of the surrounding [`lr`]($math.lr) group
  - The [`op`]($math.op) function can now take any content, not just strings
  - Improved documentation for [math alignment]($category/math/#alignment)
  - Fixed swallowing of trailing comma when a symbol is used in a function-like
    way (e.g. `pi(a,b,)`)

- Scripting
  - Any non-identifier dictionary key is now interpreted as an expression: For
    instance, `{((key): value)}` will create a dictionary with a dynamic key
  - The [`stroke`]($stroke) type now has a constructor that converts a value to
    a stroke or creates one from its parts
  - Added constructor for [`arguments`]($arguments) type
  - Added [`calc.div-euclid`]($calc.div-euclid) and
    [`calc.rem-euclid`]($calc.rem-euclid) functions
  - Fixed equality of [`arguments`]($arguments)
  - Fixed [`repr`]($repr) of [`cmyk`]($color.cmyk) colors
  - Fixed crashes with provided elements like figure captions, outline entries,
    and footnote entries

- Tooling and Diagnostics
  - Show rules that match on their own output now produce an appropriate error
    message instead of a crash (this is a first step, in the future they will
    just work)
  - Too highly or infinitely nested layouts now produce error messages instead
    of crashes
  - Added hints for invalid identifiers
  - Added hint when trying to use a manually constructed footnote or outline
    entry
  - Added missing details to autocompletions for types
  - Improved error message when passing a named argument where a positional one
    is expected
  - Jump from click now works on raw blocks

- Export
  - PDF compilation output is now again fully byte-by-byte reproducible if the
    document's [`date`]($document.date) is set manually
  - Fixed color export in SVG
  - Fixed PDF metadata encoding of multiple [authors]($document.author)

- Command line interface
  - Fixed a major bug where `typst watch` would confuse files and fail to pick
    up updates
  - Fetching of the release metadata in `typst update` now respects proxies
  - Fixed bug with `--open` flag on Windows when the path contains a space
  - The `TYPST_FONT_PATHS` environment variable can now contain multiple paths
    (separated by `;` on Windows and `:` elsewhere)
  - Updated embedded New Computer Modern fonts to version 4.7
  - The watching process doesn't stop anymore when the main file contains
    invalid UTF-8

- Miscellaneous Improvements
  - Parallelized image encoding in PDF export
  - Improved the internal representation of content for improved performance
  - Optimized introspection (query, counter, etc.) performance
  - The [document title]($document.title) can now be arbitrary content instead
    of just a string
  - The [`number-align`]($enum.number-align) parameter on numbered lists now
    also accepts vertical alignments
  - Fixed selectors on [quote]($quote) elements
  - Fixed parsing of `[#return]` expression in markup
  - Fixed bug where inline equations were displayed in equation outlines
  - Fixed potential CRLF issue in [`raw`]($raw) blocks
  - Fixed a bug where Chinese numbering couldn't exceed the number 255

- Development
  - Merged `typst` and `typst-library` and extracted `typst-pdf`, `typst-svg`,
    and `typst-render` into separate crates
  - The Nix flake now includes the git revision when running `typst --version`

<contributors from="v0.9.0" to="v0.10.0" />

## Version 0.9.0 (October 31, 2023) { #v0.9.0 }
- Bibliography management
  - New bibliography engine based on [CSL](https://citationstyles.org/)
    (Citation Style Language). Ships with about 100 commonly used citation
    styles and can load custom `.csl` files.
  - Added new [`form`]($cite.form) argument to the `cite` function to produce
    different forms of citations (e.g. for producing a citation suitable for
    inclusion in prose)
  - The [`cite`]($cite) function now takes only a single label/key instead of
    allowing multiple. Adjacent citations are merged and formatted according to
    the citation style's rules automatically. This works both with the reference
    syntax and explicit calls to the `cite` function. (**Breaking change**)
  - The `cite` function now takes a [label]($label) instead of a string
    (**Breaking change**)
  - Added [`full`]($bibliography.full) argument to bibliography function to
    print the full bibliography even if not all works were cited
  - Bibliography entries can now contain Typst equations (wrapped in `[$..$]`
    just like in markup), this works both for `.yml` and `.bib` bibliographies
  - The hayagriva YAML format was improved. See its
    [changelog](https://github.com/typst/hayagriva/blob/main/CHANGELOG.md) for
    more details. (**Breaking change**)
  - A few bugs with `.bib` file parsing were fixed
  - Removed `brackets` argument of `cite` function in favor of `form`

- Visualization
  - Gradients and colors (thanks to [@Dherse](https://github.com/Dherse))
    - Added support for [gradients]($gradient) on shapes and text
    - Supports linear, radial, and conic gradients
    - Added support for defining colors in more color spaces, including
      [Oklab]($color.oklab), [Linear RGB(A)]($color.linear-rgb),
      [HSL]($color.hsl), and [HSV]($color.hsv)
    - Added [`saturate`]($color.saturate), [`desaturate`]($color.desaturate),
      and [`rotate`]($color.rotate) functions on colors
    - Added [`color.map`]($color/#predefined-color-maps) module with predefined
      color maps that can be used with gradients
    - Rename `kind` function on colors to [`space`]($color.space)
    - Removed `to-rgba`, `to-cmyk`, and `to-luma` functions in favor of a new
      [`components`]($color.components) function
  - Improved rendering of [rectangles]($rect) with corner radius and varying
    stroke widths
  - Added support for properly clipping [boxes]($box.clip) and
    [blocks]($block.clip) with a border radius
  - Added `background` parameter to [`overline`]($overline),
    [`underline`]($underline), and [`strike`]($strike) functions
  - Fixed inaccurate color embedding in PDFs
  - Fixed ICC profile handling for images embedded in PDFs

- Text and Layout
  - Added support for automatically adding proper
    [spacing]($text.cjk-latin-spacing) between CJK and Latin text (enabled by
    default)
  - Added support for automatic adjustment of more CJK punctuation
  - Added [`quote`]($quote) element for inserting inline and block quotes with
    optional attributions
  - Added [`raw.line`]($raw.line) element for customizing the display of
    individual lines of raw text, e.g. to add line numbers while keeping proper
    syntax highlighting
  - Added support for per-side [inset]($table.inset) customization to table
    function
  - Added Hungarian and Romanian translations
  - Added support for Czech hyphenation
  - Added support for setting custom [smart quotes]($smartquote)
  - The default [figure separator]($figure.caption.separator) now reacts to the
    currently set language and region
  - Improved line breaking of links / URLs (especially helpful for
    bibliographies with many URLs)
  - Improved handling of consecutive hyphens in justification algorithm
  - Fixed interaction of justification and hanging indent
  - Fixed a bug with line breaking of short lines without spaces when
    justification is enabled
  - Fixed font fallback for hyphen generated by hyphenation
  - Fixed handling of word joiner and other no-break characters during
    hyphenation
  - Fixed crash when hyphenating after an empty line
  - Fixed line breaking of composite emoji like 🏳️‍🌈
  - Fixed missing text in some SVGs
  - Fixed font fallback in SVGs
  - Fixed behaviour of [`to`]($pagebreak.to) argument on `pagebreak` function
  - Fixed `{set align(..)}` for equations
  - Fixed spacing around [placed]($place) elements
  - Fixed coalescing of [`above`]($block.above) and [`below`]($block.below)
    spacing if given in em units and the font sizes differ
  - Fixed handling of `extent` parameter of [`underline`]($underline),
    [`overline`]($overline), and [`strike`]($strike) functions
  - Fixed crash for [floating placed elements]($place.float) with no specified
    vertical alignment
  - Partially fixed a bug with citations in footnotes

- Math
  - Added `gap` argument for [`vec`]($math.vec.gap), [`mat`]($math.mat.gap), and
    [`cases`]($math.cases.gap) function
  - Added `size` argument for [`abs`]($math.abs), [`norm`]($math.norm),
    [`floor`]($math.floor), [`ceil`]($math.ceil), and [`round`]($math.round)
    functions
  - Added [`reverse`]($math.cases.reverse) parameter to cases function
  - Added support for multinomial coefficients to [`binom`]($math.binom)
    function
  - Removed `rotation` argument on [`cancel`]($math.cancel) function in favor of
    a new and more flexible `angle` argument (**Breaking change**)
  - Added `wide` constant, which inserts twice the spacing of `quad`
  - Added `csch` and `sech` [operators]($math.op)
  - `↼`, `⇀`, `↔`, and `⟷` can now be used as [accents]($math.accent)
  - Added `integral.dash`, `integral.dash.double`, and `integral.slash`
    [symbols]($category/symbols/sym)
  - Added support for specifying negative indices for
    [augmentation]($math.mat.augment) lines to position the line from the back
  - Fixed default color of matrix [augmentation]($math.mat.augment) lines
  - Fixed attachment of primes to inline expressions
  - Math content now respects the text [baseline]($text.baseline) setting

- Performance
  - Fixed a bug related to show rules in templates which would effectively
    disable incremental compilation in affected documents
  - Micro-optimized code in several hot paths, which brings substantial
    performance gains, in particular in incremental compilations
  - Improved incremental parsing, which affects the whole incremental
    compilation pipeline
  - Added support for incremental parsing in the CLI
  - Added support for incremental SVG encoding during PDF export, which greatly
    improves export performance for documents with many SVG

- Tooling and Diagnostics
  - Improved autocompletion for variables that are in-scope
  - Added autocompletion for package imports
  - Added autocompletion for [labels]($label)
  - Added tooltip that shows which variables a function captures (when hovering
    over the equals sign or arrow of the function)
  - Diagnostics are now deduplicated
  - Improved diagnostics when trying to apply unary `+` or `-` to types that
    only support binary `+` and `-`
  - Error messages now state which label or citation key isn't present in the
    document or its bibliography
  - Fixed a bug where function argument parsing errors were shadowed by
    function execution errors (e.g. when trying to call
    [`array.sorted`]($array.sorted) and passing the key function as a positional
    argument instead of a named one).

- Export
  - Added support for configuring the document's creation
    [`date`]($document.date). If the `date` is set to `{auto}` (the default),
    the PDF's creation date will be set to the current date and time.
  - Added support for configuring document [`keywords`]($document.keywords)
  - Generated PDFs now contain PDF document IDs
  - The PDF creator tool metadata now includes the Typst version

- Web app
  - Added version picker to pin a project to an older compiler version
    (with support for Typst 0.6.0+)
  - Fixed desyncs between editor and compiler and improved overall stability
  - The app now continues to highlight the document when typing while the
    document is being compiled

- Command line interface
  - Added support for discovering fonts through fontconfig
  - Now clears the screen instead of resetting the terminal
  - Now automatically picks correct file extension for selected output format
  - Now only regenerates images for changed pages when using `typst watch` with
    PNG or SVG export

- Miscellaneous Improvements
  - Added [`version`]($version) type and `sys.version` constant specifying the
    current compiler version. Can be used to gracefully support multiple
    versions.
  - The U+2212 MINUS SIGN is now used when displaying a numeric value, in the
    [`repr`]($repr) of any numeric value and to replace a normal hyphen in text
    mode when before a digit. This improves, in particular, how negative integer
    values are displayed in math mode.
  - Added support for specifying a default value instead of failing for
    `remove` function in [array]($array.remove) and
    [dictionary]($dictionary.remove)
  - Simplified page setup guide examples
  - Switched the documentation from using the word "hashtag" to the word "hash"
    where appropriate
  - Added support for [`array.zip`]($array.zip) without any further arguments
  - Fixed crash when a plugin tried to read out of bounds memory
  - Fixed crashes when handling infinite [lengths]($length)
  - Fixed introspection (mostly bibliography) bugs due to weak page break close
    to the end of the document

- Development
  - Extracted `typst::ide` into separate `typst_ide` crate
  - Removed a few remaining `'static` bounds on `&dyn World`
  - Removed unnecessary dependency, which reduces the binary size
  - Fixed compilation of `typst` by itself (without `typst-library`)
  - Fixed warnings with Nix flake when using `lib.getExe`

<contributors from="v0.8.0" to="v0.9.0" />

## 版本 0.8.0（2023年9月13日）{ #v0.8.0 }
- 脚本
  - 插件（感谢 [@astrale-sharp](https://github.com/astrale-sharp) 和 [@arnaudgolfouse](https://github.com/arnaudgolfouse)）
    - Typst 现在可以加载编译为 WebAssembly 的[插件]($plugin)
    - 任何可以编译为 WebAssembly 的内容都可以作为插件加载
    - 这些插件是完全封装的（无法访问文件系统或网络）
    - 插件可以作为[包]($scripting/#packages)的一部分提供
    - 插件在 Web App 中也可以正常使用
  - 类型现在是一等值（**破坏性更改**）
    - 现在，[类型]($type)本身也是一个值
    - 一些类型可以像函数一样调用（具有构造函数的那些类型），例如 [`int`]($int) 和 [`str`]($str)
    - 类型检查现在采用 `{type(10) == int}` 这样的形式，而不是旧的 `{type(10) == "integer"}`。与旧方式的兼容性将保留一段时间，以便包的作者有时间进行升级，但最终会移除。
    - 方法现在是调用作用域在类型内的函数的语法糖，这意味着 `{"hello".len()}` 等价于 `{str.len("hello")}`
  - 添加了使用 `as` 重命名的 [`import`]($scripting/#modules) 支持
  - 添加了 [`duration`]($duration) 类型
  - 添加了 [CBOR]($cbor) 编码和解码的支持
  - 添加了与数据格式的字节编码和解码函数相关的功能：[`json.decode`]($json.decode)、[`json.encode`]($json.encode) 等
  - 添加了 [`array.intersperse`]($array.intersperse) 函数
  - 添加了 [`str.rev`]($str.rev) 函数
  - 添加了 `calc.tau` 常量
  - 使 [bytes]($bytes) 可连接和可相加
  - 使 [`array.zip`]($array.zip) 函数变为可变元的
  - 修复了当 `mode` 设置为 `{"math"}` 时 [`eval`]($eval) 的错误
  - 修复了字符串的 [`ends-with`]($str.ends-with) 函数的错误
  - 修复了解构与 break、continue 和 return 结合使用时的错误
  - 修复了[双曲线函数]($calc.cosh)的参数类型，它们不再接受角度了（**破坏性更改**）

- 导出
  - 添加了 SVG 导出（感谢 [@Enter-tainer](https://github.com/Enter-tainer)）
  - 修复了 PDF 字体嵌入的错误
  - 添加了支持页面标签的功能，反映在 PDF 中的[页码编号]($page.numbering)样式

- 文本和布局
  - 添加了 [`highlight`]($highlight) 用于文本高亮的功能
  - 添加了 [`polygon.regular`]($polygon.regular) 用于绘制规则多边形的功能
  - 添加了在 [`raw`]($raw) 元素中支持制表符的功能，同时伴随着 [`tab-width`]($raw.tab-size) 参数
  - 布局引擎现在会尽量避免产生 "runts"（最后一行只有一个单词）
  - 添加了芬兰语翻译
  - 为波兰语添加了断字支持
  - 改进了不同类型智能引号的连续处理
  - 修复了页面函数上的 [`number-align`]($page.number-align) 参数的垂直对齐（**破坏性更改**）
  - 修复了计数更新后弱页面断页
  - 修复了当文本字体设置为 "New Computer Modern" 时，SVG 中文本缺失的问题
  - 修复了中文的翻译问题
  - 修复了在显示规则中的空文本导致崩溃的问题
  - 修复了数字和逗号之间有换行符时的前导空格
  - 修复了在列和其他容器中浮动元素的位置
  - 修复了只包含单个方块的块的大小

- 数学
  - 添加了对[增广矩阵]($math.mat.augment)的支持
  - 移除了对 `|` 和 `||` 等分隔符自动匹配的支持，因为会有太多误报。您可以使用函数如 [`abs`]($math.abs) 或 [`norm`]($math.norm) 或显式的 [`lr`]($math.lr) 调用来代替（**破坏性更改**）
  - 修复了数学中带小数点数字后的间距问题
  - 修复了下标中素数的错误
  - 修复了弱间距
  - 修复了数学中包含换行符的文本会导致崩溃的问题

- 工具和诊断
  - 在尝试调用存储在字典中的函数时，现在会给出提示，提示需要额外的括号
  - 在引用未编号方程式时，现在会给出提示
  - 为某些诊断添加了更多细节（例如 SVG 解码失败时）

- 命令行界面
  - 添加了 `typst update` 命令以自动更新 CLI
    （感谢 [@jimvdl](https://github.com/jimvdl)）
  - 添加了包和更新的下载进度指示器
  - 添加了 `--format` 参数以明确指定输出格式
  - CLI 现在通过环境变量尊重代理配置，并具有新的 `--cert` 选项来设置自定义 CA 证书
  - 修复了字段不存在时，传递 `--one` 给 `typst query` 会导致崩溃的问题

- 其他改进
  - 添加了[页面设置指南]($guides/page-setup-guide)
  - 添加了 [`figure.caption`]($figure.caption) 函数，可用于更简单的图题自定义（**破坏性更改**，因为 `it.caption` 现在在图表显示规则和手动大纲中呈现完整的图题与补充）
  - 将 `caption-pos` 参数移动到 `figure.caption` 函数中，并将其重命名为 `position`（**破坏性更改**）
  - 为 `figure.caption` 函数添加了 [`separator`]($figure.caption.separator) 参数
  - 添加了组合和/或和之前/之后的[选择器]($selector)支持
  - 包现在可以指定它们所需的[最小编译器版本](https://github.com/typst/packages#package-format)
  - 修复了标记中方法调用可以移到它们自己的行中的解析器错误（在标记中继续工作）
  - 修复了参考文献的句子和标题大小写转换中的错误
  - 修复了字母数字和作者-标题参考文献样式的补充
  - 修复了 APA 参考文献样式中的 off-by-one 错误

- 开发
  - 使 `Span` 和 `FileId` 更具类型安全性，以便 `World` 的实现者必须处理所有错误条件

<contributors from="v0.7.0" to="v0.8.0" />

## Version 0.7.0（2023年8月7日）{#v0.7.0}
- 文本和布局
  - 通过图像函数的 [`placement`]($figure.placement) 参数添加了对浮动图像的支持
  - 通过放置函数的 [`float`]($place.float) 参数添加了对任意浮动内容的支持
  - 添加了加载 `.sublime-syntax` 文件作为原始块的高亮 [语法]($raw.syntaxes) 的支持
  - 添加了加载 `.tmTheme` 文件作为原始块的高亮 [主题]($raw.theme) 的支持
  - 为文本函数的 `top-edge` 和 `bottom-edge` 参数添加了 `_bounds_` 选项，用于创建紧密边界框
  - 移除了无意义的 `top-` 和 `bottom-edge` 选项，例如底部边缘的 _ascender_ (**破坏性更改**)
  - 为文本函数添加了 [`script`]($text.script) 参数
  - 为智能引号函数添加了 [`alternative`]($smartquote.alternative) 参数
  - 添加了日语的基础国际化支持
  - 除了 `no` 外，还添加了对 `nb` 和 `nn` 语言代码的断词支持
  - 修复了容器中 [放置元素]($place) 的定位问题
  - 修复了因优化的换行而导致容器溢出的问题

- 导出
  - 大幅改进了将 SVG 图像导出为 PDF 的功能。非常感谢 [@LaurenzV](https://github.com/LaurenzV) 在此方面的工作
  - 在 PDF 导出中添加了对 RGBA 颜色的 alpha 通道的支持
  - 修复了 PNG 导出中 PPI（每英寸像素数）的错误

- 数学
  - 改进了素数的布局（例如在 `[$a'_1$]` 中）
  - 改进了多重素数的显示（例如在 `[$a''$]` 中）
  - 改进了 [根]($math.root) 的布局
  - 默认情况下，将关系改为默认显示为 [限制]($math.limits)（例如在 `[$a ->^x b$]` 中）
  - 大型运算符和分隔符现在始终垂直居中
  - 方程中的 [方框]($box) 现在默认坐落在基线上，而不是垂直居中。值得注意的是，这不会影响 [块]($block)，因为它们不是内联元素。
  - 添加了 [弱间距]($h.weak) 的支持
  - 添加了 OpenType 字符变体的支持
  - 添加了自定义内容的 [数学类别]($math.class) 的支持
  - 修复了 `.`, `\/`, 和 `...` 周围的间距问题
  - 修复了闭合分隔符与大型运算符之间的间距问题
  - 修复了数学字体重量选择的错误
  - 符号和运算符（**破坏性更改**）
    - 添加了 `id`、`im` 和 `tr` 文本 [运算符]($math.op)
    - 将 `ident` 重命名为 `equiv`，并添加别名 `eq.triple`，并移除了 `ident.strict`，改用 `eq.quad`
    - 将 `ast.sq` 重命名为 `ast.square`，将 `integral.sq` 重命名为 `integral.square`
    - 将 `.eqq` 修饰符重命名为 `.equiv`（并将 `.neqq` 重命名为 `.nequiv`），用于 `tilde`、`gt`、`lt`、`prec` 和 `succ`
    - 添加了 `emptyset` 作为 `nothing` 的别名
    - 添加了 `lt.curly` 和 `gt.curly` 作为 `prec` 和 `succ` 的别名
    - 添加了 `aleph`、`beth` 和 `gimmel` 作为 `alef`、`bet` 和 `gimel` 的别名

- 脚本
  - 字段
    - 为 [长度]($length) 添加了 `abs` 和 `em` 字段
    - 为 [相对长度]($relative) 添加了 `ratio` 和 `length` 字段
    - 为 [2d 对齐]($align.alignment) 添加了 `x` 和 `y` 字段
    - 为 [笔画]($stroke) 添加了 `paint`、`thickness`、`cap`、`join`、`dash` 和 `miter-limit` 字段
  - 存取器和实用方法
    - 添加了 [`dedup`]($array.dedup) 数组方法
    - 添加了 [长度]($length) 的 `pt`、`mm`、`cm` 和 `inches` 方法
    - 添加了 [角度]($angle) 的 `deg` 和 `rad` 方法
    - 添加了 [颜色]($color) 的 `kind`、`hex`、`rgba`、`cmyk` 和 `luma` 方法
    - 添加了 [方向]($stack.dir) 的 `axis`、`start`、`end` 和 `inv` 方法
    - 添加了 [对齐]($align.alignment) 的 `axis` 和 `inv` 方法
    - 添加了 [2d 对齐]($align.alignment) 的 `inv` 方法
    - 在数组的 [`enumerate`]($array.enumerate) 方法上添加了 `start` 参数
  - 添加了 [`color.mix`]($color.mix) 函数
  - 为 [`eval`]($eval) 函数添加了 `mode` 和 `scope` 参数
  - 添加了 [`bytes`]($bytes) 类型，用于保存大字节缓冲区
    - 在读取函数中添加了 [`encoding`]($read.encoding) 参数，以读取字节而不是字符串
    - 添加了 [`image.decode`]($image.decode) 函数，用于直接从字符串或字节解码图像
    - 添加了 [`bytes`]($bytes) 函数，用于将字符串或整数数组转换为字节
    - 添加了 [`array`]($array) 函数，用于将字节转换为整数数组
    - 添加了使用 [`str`]($str) 函数将字节转换为字符串的支持

- 工具和诊断
  - 添加了对编译器警告的支持
  - 当编译由于强烈使用内省功能而在五次尝试内不收敛时，添加了警告
  - 对空强调 (`__` 和 `**`) 添加了警告
  - 改进了无效字段分配的错误消息
  - 改进了单个 `#` 后的错误消息
  - 当关键字用于期望标识符的位置时，改进了错误消息
  - 修复了模块中的函数的参数自动补全
  - 现在，导入自动补全仅在键入冒号之前显示最新的包版本
  - 修复了包含空格的字典键的自动补全
  - 修复了 for 循环的自动补全

- 命令行界面
  - 添加了 `typst query` 子命令，用于在命令行上执行查询
  - 不再支持在命令之前使用 `--root` 和 `--font-paths` 参数（**破坏性更改**）
  - 现在，本地和缓存的包存储在形式为 `[namespace}/{name}/{version}]` 的目录中，而不再是 `[namespace}/{name}-{version}]`（**破坏性更改**）
  - 现在，显式给定的字体（通过 `--font-paths`）优先于系统和嵌入字体
  - 修复了在某些文本编辑器中 `typst watch` 不起作用的问题
  - 修复了显示的编译时间（现在包括导出）

- 其他改进
  - 在标题中添加了 [`bookmarked`]($heading.bookmarked) 参数，用于控制标题是否成为PDF大纲的一部分
  - 添加了 [`caption-pos`]($figure.caption.position) 参数，用于控制图像标题的位置
  - 添加了 [`metadata`]($metadata) 函数，用于将任意值暴露给内省系统
  - 修复了 [`state`]($state) 被识别为 `(key, init)` 对而不仅仅是其 `key` 的问题
  - 改进了 [枚举]($enum) 的缩进逻辑。现在，它们只需要比标记的起始缩进多一个空格缩进。因此，即使是长标记如 `12.` 也只需要 2 个空格缩进。
  - 修复了 [`raw`]($raw) 块的缩进逻辑问题
  - 修复了字典的解析问题

- 开发
  - 将解析器和语法树提取到 `typst-syntax` 包中
  - 如果 Typst 的依赖关系中有相同的 [bug](https://github.com/typst/typst/issues/1842)，则 Typst 依赖项的 `World::today` 实现可能需要修复

<contributors from="v0.6.0" to="v0.7.0" />

## Version 0.6.0（2023年6月30日）{#v0.6.0}
- 包管理
  - Typst 现在内置了[包管理]($scripting/#packages)
  - 您可以导入[已发布的]($packages)社区包或创建并使用
    [系统本地](https://github.com/typst/packages#local-packages)包
  - 已发布的包也受 Web 应用程序支持

- 数学
  - 在数学模式中添加了对字形的[光学尺寸变体](https://en.wikipedia.org/wiki/Optical_size)的支持
  - 添加了参数，以根据方程式是以[`display`]($math.display)或
    [`inline`]($math.inline)风格设置的情况下有条件地启用[`limits`]($math.limits)
  - 添加了 `gt.eq.slant` 和 `lt.eq.slant` 符号
  - 在数学模式中增加了阶乘的优先级（`[$1/n!$]` 现在可以正常工作了）
  - 改进了数学模式中的[下划线]($math.underline)和[上划线]($math.overline)
  - 修复了在显示规则中使用[`limits`]($math.limits)函数的问题
  - 修复了方程式中的换行问题

- 文本和布局
  - 添加了支持交替的页面[页边距]($page.margin)，使用 `inside` 和 `outside` 键
  - 添加了指定页面[`binding`]($page.binding)的支持
  - 为 pagebreak 函数添加了 [`to`]($pagebreak.to) 参数，以跳到下一个偶数页或奇数页
  - 为更多语言（TR、SQ、TL）添加了基本的国际化支持
  - 修复了分页时缺失表格行的问题
  - 修复了[下划线]($underline)的问题
  - 修复了多余的表格线问题
  - 修复了在换行后智能引号的问题
  - 修复了与文本布局相关的崩溃问题

- 命令行界面
  - **破坏性更改：** 添加了 `--root`/`TYPST_ROOT` 目录必须包含输入文件的要求，因为它指定了 _项目_ 根目录。现有设置中使用 `TYPST_ROOT` 来模拟包管理的应更改为使用[本地包](https://github.com/typst/packages#local-packages)
  - **破坏性更改：** 现在拒绝在项目根目录之外访问文件
  - 添加了对本地包和按需包下载的支持
  - 现在监视根目录和所有包内的所有相关文件
  - 现在显示编译时间

- 其他改进
  - 添加了 [`outline.entry`]($outline.entry) 以自定义大纲条目与显示规则
  - 为错误消息添加了一些提示
  - 为[`raw`]($raw)高亮添加了一些缺失的语法
  - 改进了在 PNG 导出和 Web 应用程序中渲染旋转图像的效果
  - 使[脚注]($footnote)可重复使用和可引用
  - 修复了[`locate`]($locate)中的引用和参考文献的问题
  - 修复了文档中时态不一致的问题

- 开发
  - 添加了[贡献指南](https://github.com/typst/typst/blob/main/CONTRIBUTING.md)
  - 重新设计了 `World` 接口以适应包管理，并使其更简单实现（对于实现者的**破坏性更改**）

<contributors from="v0.5.0" to="v0.6.0" />

## Version 0.5.0（2023年6月9日）{#v0.5.0}
- 文本和布局
  - 为更多语言添加了[`raw`]($raw)语法高亮
  - 添加了对韩语[编号]($numbering)的支持
  - 为更多语言（NL、SV、DA）添加了基本国际化支持
  - 改进了东亚语言的分行
  - 扩展了大纲[`indent`]($outline.indent)属性的功能
  - 修复了列中的脚注
  - 修复了[脚注]($footnote)的分页错误
  - 修复了在列表、表格和图表中处理脚注的错误
  - 修复了CJK标点符号调整的问题
  - 修复了带有圆角矩形的崩溃问题
  - 修复了[`line`]($line)元素的对齐问题

- 数学
  - **破坏性更改：** 数学[附件]($math.attach)的语法规则得到改进：`[$f^abs(3)$]`现在解析为 `[$f^(abs(3))$]` 而不是 `[$(f^abs)(3)$]`。要消除歧义，请添加一个空格：`[$f^zeta (3)$]`。
  - 为数学（例如，[`display`]($math.display)）添加了[强制大小]($category/math/sizes)命令
  - 为[`equation`]($math.equation)添加了[`supplement`]($math.equation.supplement)参数，被[引用]($ref)使用
  - 新[符号]($category/symbols/sym)：`bullet`、`xor`、`slash.big`、`sigma.alt`、`tack.r.not`、`tack.r.short`、`tack.r.double.not`
  - 修复了矩阵中的符号错误
  - 修复了[`attach`]($math.attach)函数中的崩溃

- 脚本
  - 添加了新的[`datetime`]($datetime)类型和[`datetime.today`]($datetime.today)以获取当前日期
  - 添加了[`str.from-unicode`]($str.from-unicode)和[`str.to-unicode`]($str.to-unicode)函数
  - 在内容上添加了[`fields`]($content.fields)方法
  - 为[`str`]($str)函数添加了`base`参数
  - 添加了[`calc.exp`]($calc.exp)和[`calc.ln`]($calc.ln)
  - 改进了特定基数的[`calc.pow`]($calc.pow)和[`calc.log`]($calc.log)的精度
  - 修复了字典的[删除]($dictionary.remove)顺序
  - 修复了字符串[$str.at]和内容[$content.at]的`.at(default: ..)`问题
  - 修复了对带有样式元素的字段访问
  - 移除了已弃用的`calc.mod`函数

- 命令行界面
  - 通过 `typst compile source.typ output-{n}.png` 添加了PNG导出。如果文档有多个页面，则输出路径必须包含 `[{n}]`。
  - 添加了 `--diagnostic-format=short` 以进行类似Unix的简短诊断
  - 如果 stderr 不是 TTY，则不再发出颜色代码
  - 当以不存在的文件调用时，现在设置了正确的退出状态
  - 不再忽略Typst文件中的UTF-8 BOM

- 其他改进
  - 改进了不匹配分隔符的错误
  - 改进了长度比较失败的错误消息
  - 修复了在Apple Preview中图像未显示的问题
  - 修复了PDF大纲的多个错误
  - 修复了在[`hide`]($hide)中的引用和其他可搜索元素的错误
  - 修复了Nix flake问题

<contributors from="v0.4.0" to="v0.5.0" />
## Version 0.4.0（2023年5月20日）{#v0.4.0}
- 脚注
  - 实现了对脚注的支持
  - [`footnote`]($footnote) 函数插入脚注
  - [`footnote.entry`]($footnote.entry) 函数可用于自定义脚注列表
  - 现在支持 `{"chicago-notes"}` [引用样式]($cite.style)

- 文档
  - 添加了[LaTeX用户指南]($guides/guide-for-latex-users)
  - 现在显示可选参数的默认值
  - 在“本页内容”中添加了更丰富的大纲
  - 初步支持搜索关键字：现在可以通过“目录”找到[大纲]($outline)函数。欢迎提出更多关键字的建议！
  - 修复了搜索结果排序问题
  - 修复了许多其他小问题

- 数学
  - **破坏性更改**：方程中的对齐点 (`&`) 现在在左对齐和右对齐之间交替
  - 添加了对使用Unicode编写根号的支持：例如，`[$root(x+y)$]` 现在也可以写成 `[$√(x+y)$]`
  - 修复了不均匀的垂直[`attachment`]($math.attach)对齐
  - 修复了装饰元素的间距（例如，[canceled]($math.cancel)运算符周围的间距）
  - 修复了可伸缩符号的样式
  - 添加了 `tack.r.double`、`tack.l.double`、`dotless.i` 和 `dotless.j` [符号]($category/symbols/sym)
  - 修复了符号的显示规则（例如，`{show sym.tack: set text(blue)}`）
  - 修复了应该在之前版本中进行的从 `ast.op` 到 `ast` 的重命名遗漏

- 脚本
  - 添加了函数作用域：现在，函数可以在其自己的作用域中保存相关定义，类似于模块。例如，新的 [`assert.eq`]($assert.eq) 函数是 [`assert`]($assert) 函数作用域的一部分。请注意，目前仅内置函数支持函数作用域。
  - 添加了 [`assert.eq`]($assert.eq) 和 [`assert.ne`]($assert.ne) 函数，用于更简单的相等性和不等性断言，并提供更有帮助的错误消息
  - 在它们各自的函数作用域中公开了 [list]($list.item)、[enum]($enum.item) 和 [term list]($terms.item) 项目
  - 在 [strings]($str.at)、[arrays]($array.at)、[dictionaries]($dictionary.at) 和 [content]($content.at) 上的 `at` 方法现在支持指定默认值
  - 添加了将函数传递给 [`replace`]($str.replace) 的支持，每次匹配时调用函数
  - 修复了[替换]($str.replace)字符串：现在，它们被完全原样插入，而不再支持以前（不打算的）魔术美元符号语法来捕获组
  - 修复了在解构模式中的尾随占位符问题
  - 修复了参数解构中下划线的问题
  - 修复了嵌套模式和在无效模式上悬停时的崩溃
  - 在将类型转换为 [integer]($int) 或 [float]($float) 失败时提供更好的错误消息

- 文本和布局
  - 实现了复杂的CJK标点符号调整
  - 禁用了CJK标点的[悬挂]($text.overhang)
  - 为繁体中文添加了基本翻译
  - 修复了原始块内文本的[对齐]($raw.align)（现在保持文本自身的左对齐，例如通过图像将原始块居中对齐）
  - 添加了支持通过数组而不是函数配置表格单元[对齐]($table.align)和[填充]($table.fill)
  - 修复了自动图像[`kind`]($figure.kind)检测
  - 使[枚举编号]($enum.number-align)的对齐可配置，默认为 `end`
  - 图像现在可以通过块中的显示设置规则进行分页
  - RTL语言中智能引号的初始修复

- 导出
  - 修复了PDF导出中的连字：它们现在可以复制和搜索
  - 导出的PDF现在嵌入了具有ICC配置文件的图像
  - 修复了零厚度笔画的导出

- Web应用
  - 项目现在可以包含文件夹
  - 添加了通过拖放到文件面板上传的功能
  - 现在可以将文件面板中的文件拖放到编辑器中以将它们插入Typst文件
  - 您现在可以直接从计算机中复制粘贴图像和其他文件到编辑器中
  - 添加了重新发送确认电子邮件的按钮
  - 在深色模式下添加了反转预览颜色的选项
  - 在加载屏幕和帮助菜单中添加了提示信息。欢迎提出更多建议！
  - 为YAML文件添加了语法高亮
  - 允许在许多按钮上使用中间鼠标按钮单击以导航到新标签页
  - 允许更多项目名称
  - 修复了覆盖的Vim模式键绑定
  - 修复了关于文件上传等方面的许多错误

- 杂项改进
  - 提高了计数器、状态和查询的性能
  - 改进了更高效的增量解析以进行重新编译
  - 现在支持 `.yaml` 扩展名，除了 `.yml` 用于参考文献
  - CLI现在只在输出是TTY时发出转义代码
  - 对于 `typst` crate 的用户：`Document` 现在再次是 `Sync`，而 `World` 不再需要 `'static`

<contributors from="v0.3.0" to="v0.4.0" />

## Version 0.3.0 (2023年4月26日) { #v0.3.0 }
- **重大变更:**
  - 重命名了一些符号：以前的 `dot.op` 现在只是 `dot`，而基本的点是 `dot.basic`。`ast` 和 `tilde` 也是如此。
  - 将 `mod` 重命名为 [`rem`]($calc.rem) 以更准确地反映行为。在下一次更新之前，将保留 `mod` 作为宽限期。
  - 单独的下划线不再是有效的标识符，现在只能在模式中使用
  - 从 [`query`]($query) 中删除了 `before` 和 `after` 参数。现在可以通过灵活的[选择器]($selector)组合方法来处理这些。
  - 添加了对[附件]($math.attach)（下标、上标）的支持，它们位于基本符号之前。`top` 和 `bottom` 参数已重命名为 `t` 和 `b`。

- 新功能
  - 增加了对更复杂的[笔画]($stroke)的支持（可配置的端点、连接和虚线模式）
  - 添加了用于方程的 [`cancel`]($math.cancel) 函数
  - 在参数列表和赋值中添加了[解构]($scripting/#bindings)的支持
  - 为图像函数添加了 [`alt`]($image.alt) 文本参数
  - 添加了从 TOML 文件加载数据的 [`toml`]($toml) 函数
  - 为数组添加了 [`zip`]($array.zip)、[`sum`]($array.sum) 和 [`product`]($array.product) 方法
  - 添加了 `fact`、`perm`、`binom`、`gcd`、`lcm`、`atan2`、`quo`、`trunc` 和 `fract` [计算]($category/calculate)

- 改进
  - SVG 中的文本现在可以正常显示
  - Typst 现在生成了一个PDF目录大纲
  - [引用]($ref) 现在在显示规则中提供了被引用元素作为字段
  - 优化了换行算法，以获得更好的中文对齐
  - 位置现在是有效的选择器种类
  - 添加了一些代数符号
  - 添加了西班牙智能引号支持
  - 添加了 [`selector`]($selector) 函数，将选择器类似的值转换为可以调用组合方法的选择器
  - 改进了一些错误消息
  - 大纲和参考文献标题现在可以使用显示设置规则进行样式设置
  - 数字运算现在产生错误，而不是溢出

- Bug 修复
  - 修复了行内方程、引用和其他元素后面的标点符号之前的错误断行
  - 修复了关于[参数接收器]($arguments)的错误
  - 修复了线条厚度为零的问题
  - 修复了数学中隐藏和显示规则的问题
  - 修复了矩阵中的对齐问题
  - 修复了方程中的一些对齐错误
  - 修复了网格单元格的对齐问题
  - 在全局对齐设置存在的情况下，修复了列表标记和枚举标记的对齐
  - 修复了[路径]($path)的闭合
  - 修复了与图形引用相关的编译器崩溃
  - 数学中现在忽略单个尾随换行，就像在文本中一样

- 命令行界面
  - 现在可以使用环境变量 `TYPST_FONT_PATHS` 和 `TYPST_ROOT` 设置字体路径和编译根目录
  - `typst fonts` 的输出现在包括了嵌入式字体

- 开发
  - 添加了用于调试和优化的工具
  - 添加了 `--update` 标志和 `UPDATE_EXPECT` 环境变量以更新测试的参考图像
  - 现在可以使用 `--subtest` 运行特定的子测试
  - 测试现在在多个线程上运行

<contributors from="v0.2.0" to="v0.3.0" />

## Version 0.2.0 (2023年4月11日) { #v0.2.0 }
- **重大变更:**
  - 移除了在[for循环]($scripting/#loops)中遍历索引和值的支持。现在通过解包和枚举来处理。`map` 方法也是如此。
  - [字典]($dictionary) 现在按插入顺序进行迭代，而不是按字母顺序。

- 新功能
  - 为 let 绑定添加了[解包语法]($scripting/#bindings)，允许类似 `{let (1, 2) = array}` 这样的操作。
  - 添加了 [`enumerate`]($array.enumerate) 方法
  - 添加了 [`path`]($path) 函数用于绘制贝塞尔路径
  - 添加了 [`layout`]($layout) 函数以访问周围页面或容器的大小
  - 在 [`sorted`]($array.sorted) 方法中添加了 `key` 参数

- 命令行界面
  - 修复了 `--open` 标志阻塞程序的问题
  - 新的 Computer Modern 字体现在嵌入到二进制文件中
  - 可以通过将 `GEN_ARTIFACTS` 环境变量设置为目标目录，然后构建 Typst 来生成 shell 自动完成和 man 页面

- 杂项改进
  - 修复了大纲中的页码显示问题
  - 为更多语言添加了基本的国际化支持
    (阿拉伯语、挪威卑尔根语、捷克语、挪威尼诺斯克语、波兰语、斯洛文尼亚语、西班牙语、乌克兰语、越南语)
  - 添加了一些编号模式（伊霍拉、中文）
  - 添加了 `sinc` [运算符]($math.op)
  - 修复了数学无法使用 [`hide`]($hide) 隐藏的错误
  - 修复了盒子、块和形状的大小问题
  - 修复了一些翻译问题
  - 修复了 [`cal`]($math.cal) 和 [`frak`]($math.frak) 样式中 "R" 的反转问题
  - 修复了数学中的一些样式问题
  - 修复了引用到标题的补充问题
  - 修复了某些情况下标识符的语法高亮显示问题
  - [比例]($ratio) 现在可以与更多类型相乘，并使用 [`float`]($float) 函数转换为[浮点数]($float)

<contributors from="v0.1.0" to="v0.2.0" />

## 版本 0.1.0 (2023年4月4日) { #v0.1.0 }
- **重大变更:**
  - 当使用 CLI 时，现在必须使用子命令:
    - 使用 `typst compile file.typ` 或 `typst c file.typ` 创建 PDF
    - 使用 `typst watch file.typ` 或 `typst w file.typ` 编译并监视
    - 使用 `typst fonts` 列出所有字体
  - 手动计数器现在从零开始。阅读[此处]($counter)的 "如何设置" 部分了解更多详情。
  - [参考文献样式]($bibliography.style) `{"author-date"}` 和 `{"author-title"}` 已重命名为 `{"chicago-author-date"}` 和 `{"chicago-author-title"}`。

- 图表改进
  - 图表现在可以自动检测其内容并调整其行为。例如，包含表格的图表现在会自动加上前缀 "表 X" 并具有单独的计数器。
  - 图表的补充部分（例如 "图" 或 "表"）现在可以自定义。
  - 此外，图表现在可以完全自定义，因为显示规则提供了对自动解析的种类、补充和计数器的访问权限。

- 参考文献改进
  - [`bibliography`]($bibliography) 现在还接受多个参考文献路径（作为数组）。
  - BibLaTeX 文件的解析现在更加宽松（接受非数字的版本、页数、卷数、日期和 Jabref 样式的注释；修复了缩写解析）。
  - 标签和引用现在可以包含 `:` 和 `.`，除了末尾。
  - 修复了 APA 参考文献排序问题。

- 绘图新增功能
  - 添加了 [`polygon`]($polygon) 函数用于绘制多边形。
  - 在 [boxes]($box.clip) 和 [blocks]($block.clip) 中添加了剪裁支持。

- 命令行界面
  - 如果存在错误，现在返回非零状态码。
  - 现在默认监视根目录，而不是当前目录。
  - 默认情况下，现在将 PDF 文件放在输入文件旁边。
  - 现在接受更多类型的输入文件（例如 `/dev/stdin`）。
  - 添加了 `--open` 标志，可以直接打开 PDF。

- 其他改进
  - 添加了 [`yaml`]($yaml) 函数，用于从 YAML 文件加载数据。
  - 为更多语言添加了基本的国际化支持（意大利语、俄语、中文、法语、葡萄牙语）。
  - 添加了对希伯来语的编号支持。
  - 添加了支持以 2、8 和 16 为基数的[整数]($int)。
  - 添加了双括号和拉普拉斯运算符的符号。
  - [`link`]($link) 函数现在接受[标签]($label)。
  - 现在链接语法允许更多字符。
  - 改进了日语和中文文本的对齐。
  - 计算函数在处理非实数结果时更一致。
  - 替换了不推荐使用的尖括号。
  - 将最大函数调用深度从 256 减少到 64。
  - 修复了在段落以样式化文本开头时 [`first-line-indent`]($par.first-line-indent) 不被应用的问题。
  - 修复了等式中一元操作符周围的额外空格，例如 `{block(above: 1cm, below: 1cm, ..)}` 中的问题。
  - 修复了数学中文本操作符的样式问题。
  - 修复了对带有单个反引号的原始块中的语言标签的无效解析。
  - 修复了在某些标记元素之后进行无效自动完成的问题。

<contributors from="v23-03-28" to="v0.1.0" />

以下是日期为 2023 年 3 月 28 日的更新内容的中文翻译：

markdown
Copy code
## 2023年3月28日 { #_ }
- **重大变更:**
  - 枚举现在需要在它们的标记后面加一个空格，也就是说，`[1.ok]` 现在必须写成 `[1. ok]`。
  - 更改了[术语列表]($terms)的默认样式：不再包含冒号，并且缩进更多。

- 命令行界面
  - 为 CLI 添加了 `--font-path` 参数。
  - 在 CLI 二进制文件中嵌入了默认字体。
  - 修复了如果未安装 `git` 则 CLI 无法构建的问题。

- 其他改进
  - 添加了禁用 [矩阵]($math.mat) 和 [向量]($math.vec) 分隔符的支持。通常使用 `[#set math.mat(delim: none)]` 或单独使用 `[$mat(delim: #none, 1, 2; 3, 4)$]`。
  - 为术语列表添加了 [`separator`]($terms.separator) 参数。
  - 为等式添加了 [`round`]($math.round) 函数。
  - 编号现在允许零。要重置计数器，您可以编写 `[#counter(..).update(0)]`。
  - 添加了 [`location`]($location) 类型上的 `{page()}` 和 `{position()}` 方法的文档。
  - 添加了双点、三点和四点重音符号的符号。
  - 为挪威博克马尔语添加了智能引号支持。
  - 添加了 Nix flake。
  - 修复了 IEEE 样式下的参考文献排序问题。
  - 修复了数学中小数的解析：`[$1.2/3.4$]`。
  - 修复了分数中不平衡分隔符的解析问题：`[$1/(2 (x)$]`。
  - 修复了将数字意外解析为枚举，例如 `[1.2]`。
  - 修复了页面填充和页眉的组合问题。
  - 修复了如果在具有自动宽度的页面中使用 [`repeat`]($repeat) 会导致编译器崩溃的问题。
  - 修复了具有显式分隔符的[矩阵]($math.mat)。
  - 修复了术语列表的 [`indent`]($terms.indent) 属性。
  - 大量文档修复。
  - 参考文献中的链接现在受到链接样式的影响。
  - 修复了在 Web 应用程序中悬停在注释上的问题。

<contributors from="v23-03-21" to="v23-03-28" />

## 2023年3月21日 { #_ }
- 引用和参考文献管理
  - [参考文献]($bibliography) 和 [引用]($cite)（目前支持的样式包括 APA、芝加哥作者日期、IEEE 和 MLA）
  - 您现在可以使用 `[@label]` 来[引用]($ref)章节、图形、公式和文献。
  - 您可以通过标签使元素具有引用功能：
    - `[= Introduction <intro>]`
    - `[$ A = pi r^2 $ <area>]`

- 文档不同部分之间的交互检测系统
  - [`counter`]($counter) 函数
    - 访问和修改页面、标题、图形和方程的计数器
    - 定义并使用自定义计数器
    - 时间旅行：查找文档中其他位置（例如，在构建图表时，您可以确定在给定图表的任何位置的图表计数器的值）的计数器值。
    - 计数器按布局顺序计数，而不是按代码顺序计数。
  - [`state`]($state) 函数
    - 管理文档中的任意状态
    - 时间旅行：查找文档中任何位置的状态值
    - 状态按布局顺序修改，而不是按代码顺序。
  - [`query`]($query) 函数
    - 查找元素或标签的所有出现位置，可以是整个文档或某个位置之前/之后。
    - 链接到元素，查找其在页面上的位置并访问其字段。
    - 示例用例：自定义图表或包含当前章节标题的页面页眉。
  - [`locate`]($locate) 函数
    - 确定自身在最终布局中的位置。
    - 可以访问以获取 `page` 和 `x`、`y` 坐标。
    - 可以与计数器和状态一起使用，以查找其在该位置的值。
    - 可以与查询一起使用，以查找其位置之前或之后的元素。

- 新的 [`measure`]($measure) 函数
  - 测量元素的布局尺寸。
  - 与新的 [`style`]($style) 函数一起使用，该函数使您能够基于将要插入的内容的样式上下文生成不同的内容（因为这会影响内容的测量尺寸）。

- 暴露的内容表示
  - 内容不再是不透明的。
  - 内容可以进行比较以确定是否相等。
  - 内容元素的树可以通过代码进行遍历。
  - 可以在悬停提示中或使用 [`repr`]($repr) 查看内容。
  - 内容上的新方法: `func`、`has`、`at` 和 `location`。
  - 现在可以设置元素上的所有可选字段。
  - 更统一的字段名称（例如，`heading.title` 变成了 `heading.body`，`list.items` 变成了 `list.children`，还有其他一些更改）。

- 进一步的改进
  - 添加了 [`figure`]($figure) 函数。
  - 在方程函数上添加了 [`numbering`]($math.equation.numbering) 参数。
  - 在页面函数上添加了 [`numbering`]($page.numbering) 和 [`number-align`]($page.number-align) 参数。
  - 页面函数的 [`header`]($page.header) 和 [`footer`]($page.footer) 参数不再接受函数。如果要根据页码自定义它们，请改用新的 [`numbering`]($page.numbering) 参数或 [`counter`]($counter) 函数。
  - 添加了 [`footer-descent`]($page.footer-descent) 和 [`header-ascent`]($page.header-ascent) 参数。
  - 改进了页眉和页脚的默认对齐方式。
  - 修复了阿拉伯语元音的位置。
  - 修复了 PDF 字体嵌入问题。
  - 将 `math.formula` 重命名为 [`math.equation`]($math.equation)。
  - 现在必须将字体系列作为命名参数：`[#set text(font: "..")]`。
  - 添加了 [悬挂缩进]($par.hanging-indent) 的支持。
  - 将段落 `indent` 重命名为 [`first-line-indent`]($par.first-line-indent)。
  - 当基数为 `2` 或 `10` 时，[对数]($calc.log) 更加准确。
  - 改进了一些错误消息。
  - 修复了[`terms`]($terms) 列表的布局。

- Web 应用程序改进
  - 添加了模板库。
  - 添加了插入标题、方程、原始块和引用的按钮。
  - 通过单击预览面板中的内容（适用于文本、方程、图像等）可以跳转到其源。
  - 您现在可以上传自己的字体并在项目中使用它们。
  - 悬停调试和自动完成现在考虑了多个文件，并在显示规则中工作。
  - 悬停工具提示现在会自动折叠多个连续相等的值。
  - 当输入时，预览现在会自动滚动到正确的位置。
  - 预览区域中的链接现在可以点击。
  - 工具栏、预览和编辑器现在都可以隐藏。
  - 添加了原始块语言标签的自动完成。
  - 在 SVG 文件中添加了自动完成。
  - 新的返回按钮替代了四点按钮。
  - 大量错误修复。

## 2023年2月25日 { #_ }
- 字体变更
  - 新的默认字体：Linux Libertine
  - 原始块的新默认字体：DejaVu Sans Mono
  - 数学公式的新默认字体：New Computer Modern Math 的 Book 版本
  - 提供了许多新的数学字体
  - 删除了 Latin Modern 字体，采用了 New Computer Modern 字体系列
  - 移除了不必要的小型大写字母字体，因为它们已经可以通过相应的主要字体和 [`smallcaps`]($smallcaps) 函数访问
- 改进了标题的默认间距
- 添加了 [`panic`]($panic) 函数
- 为字符串添加了 [`clusters`]($str.clusters) 和 [`codepoints`]($str.codepoints) 方法
- 在 [`set document`]($document.author) 中支持多个作者
- 修复了访问字符串的位置不是字符边界时的崩溃问题
- 修复了在 `[#"abc"]` 末尾插入反斜杠时的增量解析问题
- 修复了一些字体系列的名称（包括 Noto Sans Symbols 和 New Computer Modern 等系列）
- 修复了字体系列的自动完成
- 改进了用户自定义函数的增量编译

## 2023年2月15日 { #_ }
- [盒子]($box) 和 [块]($block) 现在具有 `fill`、`stroke`、`radius` 和 `inset` 属性
- 块现在可以显式设置大小，固定高度的块仍然可以跨页面分页
- 块现在可以配置为可分页或不可分页，使用 [`breakable`]($block.breakable) 属性
- 现在可以为嵌套的枚举配置 [编号样式]($enum.numbering)
- 现在可以为嵌套的列表配置 [标记]($list.marker)
- [`eval`]($eval) 函数现在需要代码而不是标记，并返回任意值。仍然可以通过用括号括起字符串来评估标记。
- Typst 生成的 PDF 现在包含 XMP 元数据
- PDF 输出中现在禁用了链接框
- 表格在分页之前不再生成小的空单元格
- 修复了原始块高亮显示的错误问题

## 2023年2月12日 { #_ }
- 形状、图像和变换（移动/旋转/缩放/重复）现在都是块级元素。要将它们整合到段落中，使用 [`box`]($box) 与其他元素一样。
- "everything" 展示规则现在需要冒号：写成 `{show: it => ..}`，而不是 `{show it => ..}`。这样可以防止中间状态破坏整个文档。
- 数学公式中的非数学内容，如形状或表格，现在在垂直方向上居中显示。
- 容器内的孤行和孤字防止支持
- 列表、网格和表格支持 [RTL（从右到左）]($text.dir)
- 盒子和形状现在支持显式的 `{auto}` 大小设置
- 盒子支持分数宽度（例如 `{1fr}`）
- 修复列跳到下一页的错误问题
- 修复列表项没有行首间距的问题
- 修复列表、正方形和网格自动列的相对大小问题
- 修复 [`place`]($place) 函数中的相对位移问题
- 修复行没有大小的问题
- 修复`{set document(..)}` 后跟内容的错误问题
- 修复 `{not in}` 操作的解析问题
- 修复数学中的悬停工具提示问题
- 修复当大纲存在时，标题显示规则可能不包含分页符的错误问题
- 在 [`box`]($box) 上添加了 [`baseline`]($box.baseline) 属性
- 在数学中添加了 [`tg`]($math.op) 和 [`ctg`]($math.op) 运算符
- 为 [`cases`]($math.cases) 函数添加了分隔符设置
- 在接受函数自动完成时现在包括括号

## 2023年2月2日 { #_ }
- 合并文本和数学符号，重命名了一些符号（包括将 `infty` 重命名为 `infinity`，并使用别名 `oo`）
- 修复了缺失的斜体映射
- 数学斜体校正现在应用正确
- 括号现在在 `[$zeta(x/2)$]` 中按比例缩放
- 修复了大型根指数的位置
- 修复了 `[$abs(-x)$]` 中的间距问题
- 修复了数学中文本和标识符之间的不一致性
- 当定位上标时，现在会忽略重音符号
- 修复了矩阵中的垂直对齐问题
- 修复了 `raw` 显示规则中的 `text` 设置规则
- 标题和列表标记现在解析一致
- 允许在内容中直接使用任意数学公式

## 2023年1月30日 { #_ }
[查看发布博客帖子。](https://typst.app/blog/2023/january-update)
- 在标记/数学中的新表达式语法
  - 不再可以直接嵌入块到标记中
  - 与其他表达式一样，现在需要以井号（#）开头
  - 井号（#）可用于更多的表达式，包括字面量（`[#"string"]`）以及字段访问和方法调用，无需空格：`[#emoji.face]`
- 新的导入语法
  - `[#import "module.typ"]` 创建名为 `module` 的绑定
  - `[#import "module.typ": a, b]` 或 `[#import "module.typ": *]` 以导入项目
  - `[#import emoji: face, turtle]` 以从已绑定的模块导入
- 新的符号处理方式
  - 移除了符号表示法
  - 现在符号位于模块中：`{sym}`，`{emoji}` 和 `{math}`
  - 数学模块也重新导出了 `{sym}` 的所有内容
  - 通过字段访问进行修改，仍然不受顺序影响
  - 不再允许未知修饰符
  - 通过 `symbol` 函数支持自定义符号定义
  - 现在在文档中列出了符号
- 新的 `{math}` 模块
  - 包含所有与数学相关的函数
  - 直接位于数学中的变量和函数调用（无需井号）将访问此模块，但也可以访问本地变量
  - 可以在代码中明确使用，例如 `[#set math.vec(delim: "[")]`
- 数学中的分隔符匹配
  - 任何开放分隔符都匹配任何闭合分隔符
  - 当匹配时，它们会自动缩放
  - 为了防止缩放，可以对它们进行转义
  - 为了强制匹配两个分隔符，请使用 `lr` 函数
  - 匹配的分隔符之间可以发生换行
  - 分隔符也可以不平衡
  - 您还可以使用 `lr` 函数手动将括号（或只有一个括号）缩放到特定大小
- 具有对齐的多行数学
  - 反斜杠（\）字符插入换行符
  - `&` 字符定义对齐点
  - 对齐点还适用于下标、矢量、分情况和矩阵
  - 支持多个对齐点
- 更多功能丰富的数学函数调用
  - 数学中的函数调用现在可以使用带有井号的代码表达式
  - 现在还可以接受命名参数
  - 在数学函数调用内部，分号将前面的参数转换为数组，以支持矩阵：`[$mat(1, 2; 3, 4)$]`
- 数学中的任意内容
  - 现在可以在数学中嵌入文本、图像和其他任意内容
  - 数学现在还支持字体回退以支持例如 CJK 和表情符号
- 更多数学功能
  - 新的文本运算符：`op` 函数，`lim`，`max` 等
  - 新的矩阵函数：`mat`
  - 使用 `root` 函数的新的 n 元根：`[$root(3, x)$]`
  - 新的下标上标、上下括号、上下线
  - 新的 `abs` 和 `norm` 函数
  - 新的快捷方式：`[|`，`|]` 和 `||`
  - 新的 `attach` 函数，可通过 `script` 和 `limit` 进行覆盖附件
  - 数学中的手动间距，使用 `h`，`thin`，`med`，`thick` 和 `quad`
  - 符号和其他内容现在可以像函数一样使用，例如 `[$zeta(x)$]`
  - 添加了 Fira Math 字体，移除了 Noto Sans Math 字体
  - 通过 `[#show math.formula: set text("Fira Math")]` 支持替代数学字体
- 更多库改进
  - 新的 `calc` 模块，`abs`，`min`，`max`，`even`，`odd` 和 `mod` 移动到该模块
  - `{assert}` 函数上的新的 `message` 参数
  - 字典上的 `pairs` 方法现在返回长度为 2 的数组数组，而不是使用闭包
  - 方法调用 `{dict.at("key")}` 如果 `"key"` 不存在，现在总是失败，以前允许在赋值中使用。替代方法是 `{dict.key = x}` 和 `{dict.insert("key", x)}`
- 更智能的编辑器功能
  - 本地变量的自动补全
  - 值可用的方法的自动补全
  - 符号和模块的自动补全
  - 导入的自动补全
  - 在标识符上悬停以查看其值
- 更多编辑器改进
  - 新的离线指示器
  - 所有按钮都有工具提示
  - 改进的账户保护
  - 将状态指示器移动到错误列表按钮内
- 更多修复
  - 增量解析器的多个错误修复
  - 修复了闭包参数捕获问题
  - 修复了大量数学错误
  - 性能、文件管理和编辑可靠性的错误修复
  - 添加了登录后重新导航到原始导航页面的重定向