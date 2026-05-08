use std::borrow::{Borrow, Cow};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Deref, Range};

use comemo::Tracked;
use ecow::EcoString;
use serde::{Deserialize, Serialize};
use typst_syntax::{Span, Spanned};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::diag::{At, SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Array, Bytes, Cast, Context, Decimal, Dict, Func, IntoValue, Label, Repr, Type,
    Value, Version, cast, dict, func, repr, scope, ty,
};
use crate::layout::Alignment;

/// Create a new [`Str`] from a format string.
#[macro_export]
#[doc(hidden)]
macro_rules! __format_str {
    ($($tts:tt)*) => {{
        $crate::foundations::Str::from($crate::foundations::eco_format!($($tts)*))
    }};
}

#[doc(hidden)]
pub use ecow::eco_format;

#[doc(inline)]
pub use crate::__format_str as format_str;

/// Unicodeコードポイントの列。
///
/// [forループ]($scripting/#loops)を用いて、文字列の書記素クラスタを反復処理できます。
/// 書記素クラスタは基本的には文字ですが、まとまっているべきもの
/// （例えば複数のコードポイントが組み合わさって構成される国旗の絵文字）はまとまったまま保持します。
/// 文字列は`+`演算子で連結でき、[join]($scripting/#blocks)で結合し、整数で乗算できます。
///
/// Typstは文字列操作のためのユーティリティメソッドを提供しています。
/// これらのメソッドの多く（例：[`split`]($str.split)、[`trim`]($str.trim)、
/// [`replace`]($str.replace)）は_パターン_に対して動作します。
/// パターンは文字列または[regex]($regex)のいずれかにできるため、
/// これらのメソッドは非常に多用途です。
///
/// 全ての長さとインデックスはUTF-8バイト単位で表されます。
/// インデックスは0始まりで、負のインデックスは文字列の末尾から数えます。
///
/// この型のコンストラクターを用いて、値を文字列に変換できます。
///
/// # 例
/// ```example
/// #"hello world!" \
/// #"\"hello\n  world\"!" \
/// #"1 2 3".split() \
/// #"1,2;3".split(regex("[,;]")) \
/// #(regex("\d+") in "ten euros") \
/// #(regex("\d+") in "10 euros")
/// ```
///
/// # エスケープシーケンス { #escapes }
/// マークアップと同様に、文字列内ではいくつかの記号をエスケープできます。
/// - `[\\]`: バックスラッシュ
/// - `[\"]`: 引用符
/// - `[\n]`: 改行
/// - `[\r]`: キャリッジリターン
/// - `[\t]`: タブ
/// - `[\u{1f600}]`: 16進数のUnicodeエスケープシーケンス
#[ty(scope, cast, title = "String")]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Str(EcoString);

impl Str {
    /// Create a new, empty string.
    pub fn new() -> Self {
        Self(EcoString::new())
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Repeat the string a number of times.
    pub fn repeat(&self, n: usize) -> StrResult<Self> {
        if self.0.len().checked_mul(n).is_none() {
            return Err(eco_format!("cannot repeat this string {n} times"));
        }
        Ok(Self(self.0.repeat(n)))
    }

    /// A string slice containing the entire string.
    pub fn as_str(&self) -> &str {
        self
    }

    /// Resolve an index or throw an out of bounds error.
    fn locate(&self, index: i64) -> StrResult<usize> {
        self.locate_opt(index)?
            .ok_or_else(|| out_of_bounds(index, self.len()))
    }

    /// Resolve an index, if it is within bounds and on a valid char boundary.
    ///
    /// `index == len` is considered in bounds.
    fn locate_opt(&self, index: i64) -> StrResult<Option<usize>> {
        let wrapped =
            if index >= 0 { Some(index) } else { (self.len() as i64).checked_add(index) };

        let resolved = wrapped
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v <= self.0.len());

        if resolved.is_some_and(|i| !self.0.is_char_boundary(i)) {
            return Err(not_a_char_boundary(index));
        }

        Ok(resolved)
    }
}

#[scope]
impl Str {
    /// 値を文字列に変換します。
    ///
    /// - 整数は10進数で書式設定されます。これは省略可能な`base`引数で上書きできます。
    /// - 浮動小数点数は10進数で書式設定され、指数表記にはなりません。
    /// - 負の整数と浮動小数点数はASCIIのマイナス記号（"-" U+002D）ではなく、
    ///   Unicodeのマイナス記号（"−" U+2212）で書式設定されます。
    /// - ラベルから名前が抽出されます。
    /// - バイト列はUTF-8としてデコードされます。
    ///
    /// Unicodeコードポイントとの相互変換が必要な場合は、
    /// [`to-unicode`]($str.to-unicode)と[`from-unicode`]($str.from-unicode)関数を参照してください。
    ///
    /// ```example
    /// #str(10) \
    /// #str(4000, base: 16) \
    /// #str(2.7) \
    /// #str(1e8) \
    /// #str(<intro>)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 文字列に変換する値。
        value: ToStr,
        /// 整数を表示する基数（2から36の間）。
        #[named]
        #[default(Spanned::new(10, Span::detached()))]
        base: Spanned<i64>,
    ) -> SourceResult<Str> {
        Ok(match value {
            ToStr::Str(s) => {
                if base.v != 10 {
                    bail!(base.span, "base is only supported for integers");
                }
                s
            }
            ToStr::Int(n) => {
                if base.v < 2 || base.v > 36 {
                    bail!(base.span, "base must be between 2 and 36");
                }
                repr::format_int_with_base(n, base.v).into()
            }
        })
    }

    /// UTF-8でエンコードされた文字列のバイト長。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 文字列の最初の書記素クラスタを抽出します。
    ///
    /// 文字列が空の場合、指定されたデフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn first(
        &self,
        /// 文字列が空の場合に返すデフォルト値。
        #[named]
        default: Option<Str>,
    ) -> StrResult<Str> {
        self.0
            .graphemes(true)
            .next()
            .map(Into::into)
            .or(default)
            .ok_or_else(string_is_empty)
    }

    /// 文字列の最後の書記素クラスタを抽出します。
    ///
    /// 文字列が空の場合、指定されたデフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn last(
        &self,
        /// 文字列が空の場合に返すデフォルト値。
        #[named]
        default: Option<Str>,
    ) -> StrResult<Str> {
        self.0
            .graphemes(true)
            .next_back()
            .map(Into::into)
            .or(default)
            .ok_or_else(string_is_empty)
    }

    /// 指定されたインデックス以降の最初の書記素クラスタを抽出します。
    /// インデックスが範囲外の場合、デフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn at(
        &self,
        /// バイトインデックス。負の場合は末尾から数えます。
        index: i64,
        /// インデックスが範囲外の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        let len = self.len();
        self.locate_opt(index)?
            .and_then(|i| self.0[i..].graphemes(true).next().map(|s| s.into_value()))
            .or(default)
            .ok_or_else(|| no_default_and_out_of_bounds(index, len))
    }

    /// 文字列の部分文字列を抽出します。
    /// 開始または終了インデックスが範囲外の場合、エラーで失敗します。
    #[func]
    pub fn slice(
        &self,
        /// 開始バイトインデックス（その位置を含む）。負の場合は末尾から数えます。
        start: i64,
        /// 終了バイトインデックス（その位置を含まない）。
        /// 省略された場合、文字列の末尾までのスライス全体が抽出されます。
        /// 負の場合は末尾から数えます。
        #[default]
        end: Option<i64>,
        /// 抽出するバイト数。`end`位置として`start + count`を渡すのと同等です。
        /// `end`と同時には指定できません。
        #[named]
        count: Option<i64>,
    ) -> StrResult<Str> {
        let start = self.locate(start)?;
        let end = end.or(count.map(|c| start as i64 + c));
        let end = self.locate(end.unwrap_or(self.len() as i64))?.max(start);
        Ok(self.0[start..end].into())
    }

    /// 文字列の書記素クラスタを部分文字列の配列として返します。
    #[func]
    pub fn clusters(&self) -> Array {
        self.as_str().graphemes(true).map(|s| Value::Str(s.into())).collect()
    }

    /// 文字列のUnicodeコードポイントを部分文字列の配列として返します。
    #[func]
    pub fn codepoints(&self) -> Array {
        self.chars().map(|c| Value::Str(c.into())).collect()
    }

    /// 文字を対応するコードポイントに変換します。
    ///
    /// ```example
    /// #"a".to-unicode() \
    /// #("a\u{0300}"
    ///    .codepoints()
    ///    .map(str.to-unicode))
    /// ```
    #[func]
    pub fn to_unicode(
        /// 変換する文字。
        character: char,
    ) -> u32 {
        character as u32
    }

    /// Unicodeコードポイントを対応する文字列に変換します。
    ///
    /// ```example
    /// #str.from-unicode(97)
    /// ```
    #[func]
    pub fn from_unicode(
        /// 変換するコードポイント。
        value: u32,
    ) -> StrResult<Str> {
        let c: char = value
            .try_into()
            .map_err(|_| eco_format!("{value:#x} is not a valid codepoint"))?;
        Ok(c.into())
    }

    /// 文字列を指定されたUnicode正規化形式に正規化します。
    ///
    /// Unicodeの結合文字を含む文字列を操作する際に役立ちます。
    ///
    /// ```typ
    /// #assert.eq("é".normalize(form: "nfd"), "e\u{0301}")
    /// #assert.eq("ſ́".normalize(form: "nfkc"), "ś")
    /// ```
    #[func]
    pub fn normalize(
        &self,
        #[named]
        #[default(UnicodeNormalForm::Nfc)]
        form: UnicodeNormalForm,
    ) -> Str {
        match form {
            UnicodeNormalForm::Nfc => self.nfc().collect(),
            UnicodeNormalForm::Nfd => self.nfd().collect(),
            UnicodeNormalForm::Nfkc => self.nfkc().collect(),
            UnicodeNormalForm::Nfkd => self.nfkd().collect(),
        }
    }
    /// 文字列が指定されたパターンを含むかどうか。
    ///
    /// このメソッドには専用の構文もあります。`{"abcd".contains("bc")}`の代わりに
    /// `{"bc" in "abcd"}`と書けます。
    #[func]
    pub fn contains(
        &self,
        /// 検索するパターン。
        pattern: StrPattern,
    ) -> bool {
        match pattern {
            StrPattern::Str(pat) => self.0.contains(pat.as_str()),
            StrPattern::Regex(re) => re.is_match(self),
        }
    }

    /// 文字列が指定されたパターンで始まるかどうか。
    #[func]
    pub fn starts_with(
        &self,
        /// 文字列の先頭としてあり得るパターン。
        pattern: StrPattern,
    ) -> bool {
        match pattern {
            StrPattern::Str(pat) => self.0.starts_with(pat.as_str()),
            StrPattern::Regex(re) => re.find(self).is_some_and(|m| m.start() == 0),
        }
    }

    /// 文字列が指定されたパターンで終わるかどうか。
    #[func]
    pub fn ends_with(
        &self,
        /// 文字列の末尾としてあり得るパターン。
        pattern: StrPattern,
    ) -> bool {
        match pattern {
            StrPattern::Str(pat) => self.0.ends_with(pat.as_str()),
            StrPattern::Regex(re) => {
                let mut start_byte = 0;
                while let Some(mat) = re.find_at(self, start_byte) {
                    if mat.end() == self.0.len() {
                        return true;
                    }

                    // There might still be a match overlapping this one, so
                    // restart at the next code point.
                    let Some(c) = self[mat.start()..].chars().next() else { break };
                    start_byte = mat.start() + c.len_utf8();
                }
                false
            }
        }
    }

    /// 文字列内で指定されたパターンを検索し、最初のマッチを文字列として返します。
    /// マッチがない場合は`{none}`を返します。
    #[func]
    pub fn find(
        &self,
        /// 検索するパターン。
        pattern: StrPattern,
    ) -> Option<Str> {
        match pattern {
            StrPattern::Str(pat) => self.0.contains(pat.as_str()).then_some(pat),
            StrPattern::Regex(re) => re.find(self).map(|m| m.as_str().into()),
        }
    }

    /// 文字列内で指定されたパターンを検索し、最初のマッチのインデックスを整数として返します。
    /// マッチがない場合は`{none}`を返します。
    #[func]
    pub fn position(
        &self,
        /// 検索するパターン。
        pattern: StrPattern,
    ) -> Option<usize> {
        match pattern {
            StrPattern::Str(pat) => self.0.find(pat.as_str()),
            StrPattern::Regex(re) => re.find(self).map(|m| m.start()),
        }
    }

    /// 文字列内で指定されたパターンを検索し、最初のマッチに関する詳細を含む辞書を返します。
    /// マッチがない場合は`{none}`を返します。
    ///
    /// 返される辞書には次のキーが含まれます。
    /// - `start`: マッチの開始オフセット
    /// - `end`: マッチの終了オフセット
    /// - `text`: マッチしたテキスト
    /// - `captures`: マッチした各キャプチャグループに対応する文字列を含む配列。
    ///   配列の最初の要素は、マッチ全体ではなく最初のキャプチャを含みます。
    ///   `pattern`がキャプチャグループを持つregexでない限り空になります。
    ///
    /// ```example:"Shape of the returned dictionary"
    /// #let pat = regex("not (a|an) (apple|cat)")
    /// #"I'm a doctor, not an apple.".match(pat) \
    /// #"I am not a cat!".match(pat)
    /// ```
    ///
    /// ```example:"Different kinds of patterns"
    /// #assert.eq("Is there a".match("for this?"), none)
    /// #"The time of my life.".match(regex("[mit]+e"))
    /// ```
    #[func]
    pub fn match_(
        &self,
        /// 検索するパターン。
        pattern: StrPattern,
    ) -> Option<Dict> {
        match pattern {
            StrPattern::Str(pat) => {
                self.0.match_indices(pat.as_str()).next().map(match_to_dict)
            }
            StrPattern::Regex(re) => re.captures(self).map(captures_to_dict),
        }
    }

    /// 文字列内で指定されたパターンを検索し、全てのマッチに関する詳細を含む辞書の配列を返します。
    /// 返される辞書の詳細については[上記]($str.match)を参照してください。
    ///
    /// ```example
    /// #"Day by Day.".matches("Day")
    /// ```
    #[func]
    pub fn matches(
        &self,
        /// 検索するパターン。
        pattern: StrPattern,
    ) -> Array {
        match pattern {
            StrPattern::Str(pat) => self
                .0
                .match_indices(pat.as_str())
                .map(match_to_dict)
                .map(Value::Dict)
                .collect(),
            StrPattern::Regex(re) => re
                .captures_iter(self)
                .map(captures_to_dict)
                .map(Value::Dict)
                .collect(),
        }
    }

    /// 与えられたパターンの出現を、（先頭から）最大`count`回まで置換文字列または関数で置換します。
    /// `count`が与えられない場合、全ての出現が置換されます。
    #[func]
    pub fn replace(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 検索するパターン。
        pattern: StrPattern,
        /// マッチを置換する文字列、または、各マッチに対応する辞書を受け取り、
        /// それぞれの置換文字列を返す関数。
        ///
        /// 関数に渡される辞書は[`match`]($str.match)が返す辞書と同じ形式です。
        replacement: Replacement,
        /// 指定された場合、パターンの最初の`count`個のマッチのみが置換されます。
        #[named]
        count: Option<usize>,
    ) -> SourceResult<Str> {
        // Heuristic: Assume the new string is about the same length as
        // the current string.
        let mut output = EcoString::with_capacity(self.as_str().len());

        // Replace one match of a pattern with the replacement.
        let mut last_match = 0;
        let mut handle_match = |range: Range<usize>, dict: Dict| -> SourceResult<()> {
            // Push everything until the match.
            output.push_str(&self[last_match..range.start]);
            last_match = range.end;

            // Determine and push the replacement.
            match &replacement {
                Replacement::Str(s) => output.push_str(s),
                Replacement::Func(func) => {
                    let piece = func
                        .call(engine, context, [dict])?
                        .cast::<Str>()
                        .at(func.span())?;
                    output.push_str(&piece);
                }
            }

            Ok(())
        };

        // Iterate over the matches of the `pattern`.
        let count = count.unwrap_or(usize::MAX);
        match &pattern {
            StrPattern::Str(pat) => {
                for m in self.match_indices(pat.as_str()).take(count) {
                    let (start, text) = m;
                    handle_match(start..start + text.len(), match_to_dict(m))?;
                }
            }
            StrPattern::Regex(re) => {
                for caps in re.captures_iter(self).take(count) {
                    // Extract the entire match over all capture groups.
                    let m = caps.get(0).unwrap();
                    handle_match(m.start()..m.end(), captures_to_dict(caps))?;
                }
            }
        }

        // Push the remainder.
        output.push_str(&self[last_match..]);
        Ok(output.into())
    }

    /// 文字列の片側または両側からパターンのマッチを1回または繰り返し削除し、
    /// 結果の文字列を返します。
    #[func]
    pub fn trim(
        &self,
        /// 検索するパターン。`{none}`の場合は空白文字を削除します。
        #[default]
        pattern: Option<StrPattern>,
        /// `{start}`または`{end}`を指定すると、文字列の先頭または末尾のみを削除します。
        /// 省略した場合、両側を削除します。
        #[named]
        at: Option<StrSide>,
        /// パターンのマッチを繰り返し削除するか、一度のみ削除するかを指定します。
        /// 既定値は`{true}`です。
        #[named]
        #[default(true)]
        repeat: bool,
    ) -> Str {
        let mut start = matches!(at, Some(StrSide::Start) | None);
        let end = matches!(at, Some(StrSide::End) | None);

        let trimmed = match pattern {
            None => match at {
                None => self.0.trim(),
                Some(StrSide::Start) => self.0.trim_start(),
                Some(StrSide::End) => self.0.trim_end(),
            },
            Some(StrPattern::Str(pat)) => {
                let pat = pat.as_str();
                let mut s = self.as_str();
                if repeat {
                    if start {
                        s = s.trim_start_matches(pat);
                    }
                    if end {
                        s = s.trim_end_matches(pat);
                    }
                } else {
                    if start {
                        s = s.strip_prefix(pat).unwrap_or(s);
                    }
                    if end {
                        s = s.strip_suffix(pat).unwrap_or(s);
                    }
                }
                s
            }
            Some(StrPattern::Regex(re)) => {
                let s = self.as_str();
                let mut last = None;
                let mut range = 0..s.len();

                for m in re.find_iter(s) {
                    // Does this match follow directly after the last one?
                    let consecutive = last == Some(m.start());

                    // As long as we're at the beginning or in a consecutive run
                    // of matches, and we're still trimming at the start, trim.
                    start &= m.start() == 0 || consecutive;
                    if start {
                        range.start = m.end();
                        start &= repeat;
                    }

                    // Reset end trim if we aren't consecutive anymore or aren't
                    // repeating.
                    if end && (!consecutive || !repeat) {
                        range.end = m.start();
                    }

                    last = Some(m.end());
                }

                // Is the last match directly at the end?
                if last.is_some_and(|last| last < s.len()) {
                    range.end = s.len();
                }

                &s[range.start..range.start.max(range.end)]
            }
        };

        trimmed.into()
    }

    /// 指定されたパターンのマッチで文字列を分割し、結果として得られるパーツの配列を返します。
    ///
    /// 空文字列を区切り文字として使うと、文字列内の全ての文字
    /// （つまりUnicodeコードポイント）と、文字列の先頭および末尾で分割します。
    /// これは実際には、結果のパーツのリストが先頭と末尾に空文字列を含むことを意味します。
    #[func]
    pub fn split(
        &self,
        /// 分割位置のパターン。既定では空白文字です。
        #[default]
        pattern: Option<StrPattern>,
    ) -> Array {
        let s = self.as_str();
        match pattern {
            None => s.split_whitespace().map(|v| Value::Str(v.into())).collect(),
            Some(StrPattern::Str(pat)) => {
                s.split(pat.as_str()).map(|v| Value::Str(v.into())).collect()
            }
            Some(StrPattern::Regex(re)) => {
                re.split(s).map(|v| Value::Str(v.into())).collect()
            }
        }
    }

    /// 文字列を反転します。
    #[func(title = "Reverse")]
    pub fn rev(&self) -> Str {
        let mut s = EcoString::with_capacity(self.0.len());
        for grapheme in self.as_str().graphemes(true).rev() {
            s.push_str(grapheme);
        }
        s.into()
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl Repr for Str {
    fn repr(&self) -> EcoString {
        self.as_ref().repr()
    }
}

impl Repr for EcoString {
    fn repr(&self) -> EcoString {
        self.as_ref().repr()
    }
}

impl Repr for str {
    fn repr(&self) -> EcoString {
        let mut r = EcoString::with_capacity(self.len() + 2);
        r.push('"');
        for c in self.chars() {
            match c {
                '\0' => r.push_str(r"\u{0}"),
                '\'' => r.push('\''),
                '"' => r.push_str(r#"\""#),
                _ => r.extend(c.escape_debug()),
            }
        }
        r.push('"');
        r
    }
}

impl Repr for char {
    fn repr(&self) -> EcoString {
        EcoString::from(*self).repr()
    }
}

impl Add for Str {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Str {
    fn add_assign(&mut self, rhs: Self) {
        self.0.push_str(rhs.as_str());
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self
    }
}

impl From<char> for Str {
    fn from(c: char) -> Self {
        Self(c.into())
    }
}

impl From<&str> for Str {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<EcoString> for Str {
    fn from(s: EcoString) -> Self {
        Self(s)
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<Cow<'_, str>> for Str {
    fn from(s: Cow<str>) -> Self {
        Self(s.into())
    }
}

impl FromIterator<char> for Str {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Str> for EcoString {
    fn from(str: Str) -> Self {
        str.0
    }
}

impl From<Str> for String {
    fn from(s: Str) -> Self {
        s.0.into()
    }
}

cast! {
    char,
    self => Value::Str(self.into()),
    string: Str => {
        let mut chars = string.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => c,
            _ => bail!("expected exactly one character"),
        }
    },
}

cast! {
    &str,
    self => Value::Str(self.into()),
}

cast! {
    EcoString,
    self => Value::Str(self.into()),
    v: Str => v.into(),
}

cast! {
    String,
    self => Value::Str(self.into()),
    v: Str => v.into(),
}

/// 文字列にキャスト可能な値。
pub enum ToStr {
    /// そのまま使える文字列値。
    Str(Str),
    /// 与えられた基数で書式設定される整数。
    Int(i64),
}

cast! {
    ToStr,
    v: i64 => Self::Int(v),
    v: f64 => Self::Str(repr::display_float(v).into()),
    v: Decimal => Self::Str(format_str!("{}", v)),
    v: Version => Self::Str(format_str!("{}", v)),
    v: Bytes => Self::Str(v.to_str().map_err(|_| "bytes are not valid utf-8")?),
    v: Label => Self::Str(v.resolve().as_str().into()),
    v: Type => Self::Str(v.long_name().into()),
    v: Str => Self::Str(v),
}

/// Unicode正規化形式。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum UnicodeNormalForm {
    /// 正規合成。例えばアクセント付き文字が単一のUnicodeコードポイントに変換されます。
    #[string("nfc")]
    Nfc,
    /// 正規分解。例えばアクセント付き文字がベース文字とダイアクリティカルマークに分離されます。
    #[string("nfd")]
    Nfd,
    /// NFCと同様ですが、Unicode互換分解を用います。
    #[string("nfkc")]
    Nfkc,
    /// NFDと同様ですが、Unicode互換分解を用います。
    #[string("nfkd")]
    Nfkd,
}

/// Convert an item of std's `match_indices` to a dictionary.
fn match_to_dict((start, text): (usize, &str)) -> Dict {
    dict! {
        "start" => start,
        "end" => start + text.len(),
        "text" => text,
        "captures" => Array::new(),
    }
}

/// Convert regex captures to a dictionary.
fn captures_to_dict(cap: regex::Captures) -> Dict {
    let m = cap.get(0).expect("missing first match");
    dict! {
        "start" => m.start(),
        "end" => m.end(),
        "text" => m.as_str(),
        "captures" =>  cap.iter()
            .skip(1)
            .map(|opt| opt.map_or(Value::None, |m| m.as_str().into_value()))
            .collect::<Array>(),
    }
}

/// The out of bounds access error message.
#[cold]
fn out_of_bounds(index: i64, len: usize) -> EcoString {
    eco_format!("string index out of bounds (index: {}, len: {})", index, len)
}

/// The out of bounds access error message when no default value was given.
#[cold]
fn no_default_and_out_of_bounds(index: i64, len: usize) -> EcoString {
    eco_format!(
        "no default value was specified and string index out of bounds (index: {}, len: {})",
        index,
        len
    )
}

/// The char boundary access error message.
#[cold]
fn not_a_char_boundary(index: i64) -> EcoString {
    eco_format!("string index {} is not a character boundary", index)
}

/// The error message when the string is empty.
#[cold]
fn string_is_empty() -> EcoString {
    "string is empty".into()
}

/// A regular expression.
///
/// Can be used as a [show rule selector]($styling/#show-rules) and with
/// [string methods]($str) like `find`, `split`, and `replace`.
///
/// [See here](https://docs.rs/regex/latest/regex/#syntax) for a specification
/// of the supported syntax.
///
/// # Example
/// ```example
/// // Works with string methods.
/// #"a,b;c".split(regex("[,;]"))
///
/// // Works with show rules.
/// #show regex("\d+"): set text(red)
///
/// The numbers 1 to 10.
/// ```
#[ty(scope)]
#[derive(Debug, Clone)]
pub struct Regex(regex::Regex);

impl Regex {
    /// Create a new regular expression.
    pub fn new(re: &str) -> StrResult<Self> {
        regex::Regex::new(re).map(Self).map_err(|err| eco_format!("{err}"))
    }
}

#[scope]
impl Regex {
    /// Create a regular expression from a string.
    #[func(constructor)]
    pub fn construct(
        /// The regular expression as a string.
        ///
        /// Most regex escape sequences just work because they are not valid Typst
        /// escape sequences. To produce regex escape sequences that are also valid in
        /// Typst (e.g. `[\\]`), you need to escape twice. Thus, to match a verbatim
        /// backslash, you would need to write `{regex("\\\\")}`.
        ///
        /// If you need many escape sequences, you can also create a raw element
        /// and extract its text to use it for your regular expressions:
        /// ```{regex(`\d+\.\d+\.\d+`.text)}```.
        regex: Spanned<Str>,
    ) -> SourceResult<Regex> {
        Self::new(&regex.v).at(regex.span)
    }
}

impl Deref for Regex {
    type Target = regex::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Repr for Regex {
    fn repr(&self) -> EcoString {
        eco_format!("regex({})", self.0.as_str().repr())
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}

/// 文字列内で検索可能なパターン。
#[derive(Debug, Clone)]
pub enum StrPattern {
    /// 単なる文字列。
    Str(Str),
    /// regex。
    Regex(Regex),
}

cast! {
    StrPattern,
    self => match self {
        Self::Str(v) => v.into_value(),
        Self::Regex(v) => v.into_value(),
    },
    v: Str => Self::Str(v),
    v: Regex => Self::Regex(v),
}

/// 文字列の一方の側。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StrSide {
    /// 文字列の論理的な先頭。言語によって左または右のいずれかになります。
    Start,
    /// 文字列の論理的な末尾。
    End,
}

cast! {
    StrSide,
    v: Alignment => match v {
        Alignment::START => Self::Start,
        Alignment::END => Self::End,
        _ => bail!("expected either `start` or `end`"),
    },
}

/// マッチした[`Str`]に対する置換。
pub enum Replacement {
    /// マッチを置換する文字列。
    Str(Str),
    /// Dict -> Str型の関数（`captures_to_dict`または`match_to_dict`を参照）。
    /// その出力がマッチに対して挿入されます。
    Func(Func),
}

cast! {
    Replacement,
    self => match self {
        Self::Str(v) => v.into_value(),
        Self::Func(v) => v.into_value(),
    },
    v: Str => Self::Str(v),
    v: Func => Self::Func(v)
}
