use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::num::{NonZeroI64, NonZeroUsize};
use std::ops::{Add, AddAssign};

use comemo::Tracked;
use ecow::{EcoString, EcoVec, eco_format};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use typst_syntax::{Span, Spanned};

use crate::diag::{At, HintedStrResult, SourceDiagnostic, SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{
    Args, Bytes, CastInfo, Context, Dict, FromValue, Func, IntoValue, Reflect, Repr, Str,
    Value, Version, cast, func, ops, repr, scope, ty,
};

/// Create a new [`Array`] from values.
#[macro_export]
#[doc(hidden)]
macro_rules! __array {
    ($value:expr; $count:expr) => {
        $crate::foundations::Array::from($crate::foundations::eco_vec![
            $crate::foundations::IntoValue::into_value($value);
            $count
        ])
    };

    ($($value:expr),* $(,)?) => {
        $crate::foundations::Array::from($crate::foundations::eco_vec![$(
            $crate::foundations::IntoValue::into_value($value)
        ),*])
    };
}

#[doc(inline)]
pub use crate::__array as array;

/// 値の列。
///
/// カンマ区切りの値の列を丸括弧で囲むことで配列を構築できます。
/// 値は同じ型である必要はありません。
///
/// `.at()`メソッドで配列の要素にアクセスし、更新できます。
/// インデックスは0始まりで、負のインデックスは配列の末尾から数えます。
/// [forループ]($scripting/#loops)を用いて配列を反復処理できます。
/// 配列は`+`演算子で連結でき、[join]($scripting/#blocks)で結合し、整数で乗算できます。
///
/// **注:** 長さ1の配列には、`{(1,)}`のように末尾のカンマが必要です。
/// これは`{(1 + 2) * 3}`のような単純な丸括弧の式と区別するためです。
/// 空の配列は`{()}`と書きます。
///
/// # 例
/// ```example
/// #let values = (1, 7, 4, -3, 2)
///
/// #values.at(0) \
/// #(values.at(0) = 3)
/// #values.at(-1) \
/// #values.find(calc.even) \
/// #values.filter(calc.odd) \
/// #values.map(calc.abs) \
/// #values.rev() \
/// #(1, (2, 3)).flatten() \
/// #(("A", "B", "C")
///     .join(", ", last: " and "))
/// ```
#[ty(scope, cast)]
#[derive(Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Array(EcoVec<Value>);

impl Array {
    /// Create a new, empty array.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new vec, with a known capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(EcoVec::with_capacity(capacity))
    }

    /// Return `true` if the length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Extract a slice of the whole array.
    pub fn as_slice(&self) -> &[Value] {
        self.0.as_slice()
    }

    /// Iterate over references to the contained values.
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.0.iter()
    }

    /// Mutably borrow the first value in the array.
    pub fn first_mut(&mut self) -> StrResult<&mut Value> {
        self.0.make_mut().first_mut().ok_or_else(array_is_empty)
    }

    /// Mutably borrow the last value in the array.
    pub fn last_mut(&mut self) -> StrResult<&mut Value> {
        self.0.make_mut().last_mut().ok_or_else(array_is_empty)
    }

    /// Mutably borrow the value at the given index.
    pub fn at_mut(&mut self, index: i64) -> StrResult<&mut Value> {
        let len = self.len();
        self.locate_opt(index, false)
            .and_then(move |i| self.0.make_mut().get_mut(i))
            .ok_or_else(|| out_of_bounds(index, len))
    }

    /// Resolve an index or throw an out of bounds error.
    fn locate(&self, index: i64, end_ok: bool) -> StrResult<usize> {
        self.locate_opt(index, end_ok)
            .ok_or_else(|| out_of_bounds(index, self.len()))
    }

    /// Resolve an index, if it is within bounds.
    ///
    /// `index == len` is considered in bounds if and only if `end_ok` is true.
    fn locate_opt(&self, index: i64, end_ok: bool) -> Option<usize> {
        let wrapped =
            if index >= 0 { Some(index) } else { (self.len() as i64).checked_add(index) };

        wrapped
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v < self.0.len() + end_ok as usize)
    }

    /// Repeat this array `n` times.
    pub fn repeat(&self, n: usize) -> StrResult<Self> {
        let count = self
            .len()
            .checked_mul(n)
            .ok_or_else(|| format!("cannot repeat this array {n} times"))?;

        Ok(self.iter().cloned().cycle().take(count).collect())
    }
}

#[scope]
impl Array {
    /// 値を配列に変換します。
    ///
    /// この関数はコレクション的な値を配列に変換することのみを目的としており、
    /// 個別の要素から配列を生成するためのものではない点に注意してください。
    /// 個別の要素から配列を生成するには、代わりに配列構文`(1, 2, 3)`
    /// （単一要素の配列の場合は`(1,)`）を用いてください。
    ///
    /// ```example
    /// #let hi = "Hello 😃"
    /// #array(bytes(hi))
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 配列に変換する値。
        value: ToArray,
    ) -> Array {
        value.0
    }

    /// 配列内の値の数。
    #[func(title = "Length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 配列の最初の要素を返します。代入の左辺としても使えます。
    /// 配列が空の場合、デフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn first(
        &self,
        /// 配列が空の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.0.first().cloned().or(default).ok_or_else(array_is_empty)
    }

    /// 配列の最後の要素を返します。代入の左辺としても使えます。
    /// 配列が空の場合、デフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn last(
        &self,
        /// 配列が空の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.0.last().cloned().or(default).ok_or_else(array_is_empty)
    }

    /// 配列の指定されたインデックスの要素を返します。代入の左辺としても使えます。
    /// インデックスが範囲外の場合、デフォルト値を返します。
    /// デフォルト値が指定されていない場合はエラーで失敗します。
    #[func]
    pub fn at(
        &self,
        /// 要素を取得するインデックス。負の場合は末尾から数えます。
        index: i64,
        /// インデックスが範囲外の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.locate_opt(index, false)
            .and_then(|i| self.0.get(i).cloned())
            .or(default)
            .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
    }

    /// 配列の末尾に値を追加します。
    #[func]
    pub fn push(
        &mut self,
        /// 配列の末尾に挿入する値。
        value: Value,
    ) {
        self.0.push(value);
    }

    /// 配列の最後の要素を削除して返します。
    /// 配列が空の場合はエラーで失敗します。
    #[func]
    pub fn pop(&mut self) -> StrResult<Value> {
        self.0.pop().ok_or_else(array_is_empty)
    }

    /// 配列の指定されたインデックスに値を挿入し、それ以降の全ての要素を右にずらします。
    /// インデックスが範囲外の場合はエラーで失敗します。
    ///
    /// 配列の要素を置換するには、[`at`]($array.at)を用いてください。
    #[func]
    pub fn insert(
        &mut self,
        /// 要素を挿入するインデックス。負の場合は末尾から数えます。
        index: i64,
        /// 配列に挿入する値。
        value: Value,
    ) -> StrResult<()> {
        let i = self.locate(index, true)?;
        self.0.insert(i, value);
        Ok(())
    }

    /// 配列から指定されたインデックスの値を削除して返します。
    #[func]
    pub fn remove(
        &mut self,
        /// 要素を削除するインデックス。負の場合は末尾から数えます。
        index: i64,
        /// インデックスが範囲外の場合に返すデフォルト値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        self.locate_opt(index, false)
            .map(|i| self.0.remove(i))
            .or(default)
            .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
    }

    /// 配列の部分スライスを抽出します。
    /// 開始または終了インデックスが範囲外の場合はエラーで失敗します。
    #[func]
    pub fn slice(
        &self,
        /// 開始インデックス（その位置を含む）。負の場合は末尾から数えます。
        start: i64,
        /// 終了インデックス（その位置を含まない）。
        /// 省略された場合、配列の末尾までのスライス全体が抽出されます。
        /// 負の場合は末尾から数えます。
        #[default]
        end: Option<i64>,
        /// 抽出する要素数。`end`位置として`start + count`を渡すのと同等です。
        /// `end`と同時には指定できません。
        #[named]
        count: Option<i64>,
    ) -> StrResult<Array> {
        let start = self.locate(start, true)?;
        let end = end.or(count.map(|c| start as i64 + c));
        let end = self.locate(end.unwrap_or(self.len() as i64), true)?.max(start);
        Ok(self.0[start..end].into())
    }

    /// 配列が指定された値を含むかどうか。
    ///
    /// このメソッドには専用の構文もあります。`{(1, 2, 3).contains(2)}`の代わりに
    /// `{2 in (1, 2, 3)}`と書けます。
    #[func]
    pub fn contains(
        &self,
        /// 検索する値。
        value: Value,
    ) -> bool {
        self.0.contains(&value)
    }

    /// 与えられた関数が`{true}`を返す要素を検索し、最初のマッチを返します。
    /// マッチがない場合は`{none}`を返します。
    #[func]
    pub fn find(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。ブール値を返さなければなりません。
        searcher: Func,
    ) -> SourceResult<Option<Value>> {
        for item in self.iter() {
            if searcher
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(searcher.span())?
            {
                return Ok(Some(item.clone()));
            }
        }
        Ok(None)
    }

    /// 与えられた関数が`{true}`を返す要素を検索し、最初のマッチのインデックスを返します。
    /// マッチがない場合は`{none}`を返します。
    #[func]
    pub fn position(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。ブール値を返さなければなりません。
        searcher: Func,
    ) -> SourceResult<Option<i64>> {
        for (i, item) in self.iter().enumerate() {
            if searcher
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(searcher.span())?
            {
                return Ok(Some(i as i64));
            }
        }

        Ok(None)
    }

    /// 数の列からなる配列を生成します。
    ///
    /// 位置引数を1つだけ渡した場合、それは範囲の`end`として解釈されます。
    /// 2つ渡した場合、それらは範囲の`start`と`end`を表します。
    ///
    /// この関数は、array関数のスコープとグローバルの両方で利用できます。
    ///
    /// ```example
    /// #range(5) \
    /// #range(2, 5) \
    /// #range(20, step: 4) \
    /// #range(21, step: 4) \
    /// #range(5, 2, step: -1)
    /// ```
    #[func]
    pub fn range(
        args: &mut Args,
        /// 範囲の始まり（その値を含む）。
        #[external]
        #[default]
        start: i64,
        /// 範囲の終わり（その値を含まない）。
        #[external]
        end: i64,
        /// 生成される数の間隔。
        #[named]
        #[default(NonZeroI64::new(1).unwrap())]
        step: NonZeroI64,
    ) -> SourceResult<Array> {
        let first = args.expect::<i64>("end")?;
        let (start, end) = match args.eat::<i64>()? {
            Some(second) => (first, second),
            None => (0, first),
        };

        let step = step.get();

        let mut x = start;
        let mut array = Self::new();

        while x.cmp(&end) == 0.cmp(&step) {
            array.push(x.into_value());
            x += step;
        }

        Ok(array)
    }

    /// 元の配列のうち、与えられた関数が真を返す要素のみからなる新しい配列を生成します。
    #[func]
    pub fn filter(
        &self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。ブール値を返さなければなりません。
        test: Func,
    ) -> SourceResult<Array> {
        let mut kept = EcoVec::new();
        for item in self.iter() {
            if test
                .call(engine, context, [item.clone()])?
                .cast::<bool>()
                .at(test.span())?
            {
                kept.push(item.clone())
            }
        }
        Ok(kept.into())
    }

    /// 元の配列の全ての要素を与えられた関数で変換した新しい配列を生成します。
    #[func]
    pub fn map(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。
        mapper: Func,
    ) -> SourceResult<Array> {
        self.into_iter()
            .map(|item| mapper.call(engine, context, [item]))
            .collect()
    }

    /// 値とそれぞれのインデックスを並べた新しい配列を返します。
    ///
    /// 返される配列は、長さ2の配列の形式の`(index, value)`ペアからなります。
    /// これらはlet束縛またはforループで[分割代入]($scripting/#bindings)できます。
    ///
    /// ```example
    /// #for (i, value) in ("A", "B", "C").enumerate() {
    ///   [#i: #value \ ]
    /// }
    ///
    /// #("A", "B", "C").enumerate(start: 1)
    /// ```
    #[func]
    pub fn enumerate(
        self,
        /// 返されるリストの最初のペアに対して返されるインデックス。
        #[named]
        #[default(0)]
        start: i64,
    ) -> StrResult<Array> {
        self.into_iter()
            .enumerate()
            .map(|(i, value)| {
                Ok(array![
                    start
                        .checked_add_unsigned(i as u64)
                        .ok_or("array index is too large")?,
                    value
                ]
                .into_value())
            })
            .collect()
    }

    /// 配列を他の配列と組み合わせます。
    ///
    /// 配列の配列を返します。`i`番目の内側の配列は、各元配列の`i`番目の要素全てを含みます。
    ///
    /// 組み合わせる配列の長さが異なる場合、最短の配列の最後の要素まで組み合わされ、
    /// 残りの要素は全て無視されます。
    ///
    /// この関数は可変長引数を取ります。
    /// 一度に複数の配列を組み合わせられます。
    /// `{(1, 2).zip(("A", "B"), (10, 20))}`は
    /// `{((1, "A", 10), (2, "B", 20))}`を返します。
    #[func]
    pub fn zip(
        self,
        args: &mut Args,
        /// 全ての配列が同じ長さでなければならないかどうか。
        /// 例えば`{(1, 2).zip((1, 2, 3), exact: true)}`はエラーを生成します。
        #[named]
        #[default(false)]
        exact: bool,
        /// 組み合わせる配列。
        #[external]
        #[variadic]
        others: Vec<Array>,
    ) -> SourceResult<Array> {
        let remaining = args.remaining();

        // Fast path for one array.
        if remaining == 0 {
            return Ok(self.into_iter().map(|item| array![item].into_value()).collect());
        }

        // Fast path for just two arrays.
        if remaining == 1 {
            let Spanned { v: other, span: other_span } =
                args.expect::<Spanned<Array>>("others")?;
            if exact && self.len() != other.len() {
                bail!(
                    other_span,
                    "second array has different length ({}) from first array ({})",
                    other.len(),
                    self.len()
                );
            }
            return Ok(self
                .into_iter()
                .zip(other)
                .map(|(first, second)| array![first, second].into_value())
                .collect());
        }

        // If there is more than one array, we use the manual method.
        let mut out = Self::with_capacity(self.len());
        let arrays = args.all::<Spanned<Array>>()?;
        if exact {
            let errs = arrays
                .iter()
                .filter(|sp| sp.v.len() != self.len())
                .map(|Spanned { v, span }| {
                    SourceDiagnostic::error(
                        *span,
                        eco_format!(
                            "array has different length ({}) from first array ({})",
                            v.len(),
                            self.len()
                        ),
                    )
                })
                .collect::<EcoVec<_>>();
            if !errs.is_empty() {
                return Err(errs);
            }
        }

        let mut iterators =
            arrays.into_iter().map(|i| i.v.into_iter()).collect::<Vec<_>>();

        for this in self {
            let mut row = Self::with_capacity(1 + iterators.len());
            row.push(this.clone());

            for iterator in &mut iterators {
                let Some(item) = iterator.next() else {
                    return Ok(out);
                };

                row.push(item);
            }

            out.push(row.into_value());
        }

        Ok(out)
    }

    /// 累積関数を用いて、全ての要素を単一の値に畳み込みます。
    ///
    /// ```example
    /// #let array = (1, 2, 3, 4)
    /// #array.fold(0, (acc, x) => acc + x)
    /// ```
    #[func]
    pub fn fold(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 開始時の初期値。
        init: Value,
        /// 畳み込み関数。
        /// 2つの引数を取らなければなりません。1つは累積値、もう1つは要素です。
        folder: Func,
    ) -> SourceResult<Value> {
        let mut acc = init;
        for item in self {
            acc = folder.call(engine, context, [acc, item])?;
        }
        Ok(acc)
    }

    /// 全ての要素を合計します（加算可能な全ての型に対して動作します）。
    #[func]
    pub fn sum(
        self,
        /// 配列が空の場合に返す値。配列が空になり得る場合は指定する必要があります。
        #[named]
        default: Option<Value>,
    ) -> HintedStrResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter
            .next()
            .or(default)
            .ok_or("cannot calculate sum of empty array with no default")?;
        for item in iter {
            acc = ops::add(acc, item)?;
        }
        Ok(acc)
    }

    /// 全ての要素の積を計算します（乗算可能な全ての型に対して動作します）。
    #[func]
    pub fn product(
        self,
        /// 配列が空の場合に返す値。配列が空になり得る場合は指定する必要があります。
        #[named]
        default: Option<Value>,
    ) -> HintedStrResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter
            .next()
            .or(default)
            .ok_or("cannot calculate product of empty array with no default")?;
        for item in iter {
            acc = ops::mul(acc, item)?;
        }
        Ok(acc)
    }

    /// 与えられた関数が配列内のいずれかの要素に対して`{true}`を返すかどうか。
    #[func]
    pub fn any(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。ブール値を返さなければなりません。
        test: Func,
    ) -> SourceResult<bool> {
        for item in self {
            if test.call(engine, context, [item])?.cast::<bool>().at(test.span())? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// 与えられた関数が配列内の全ての要素に対して`{true}`を返すかどうか。
    #[func]
    pub fn all(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 各要素に適用する関数。ブール値を返さなければなりません。
        test: Func,
    ) -> SourceResult<bool> {
        for item in self {
            if !test.call(engine, context, [item])?.cast::<bool>().at(test.span())? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// 全てのネストされた配列を単一のフラットな配列に統合します。
    #[func]
    pub fn flatten(self) -> Array {
        let mut flat = EcoVec::with_capacity(self.0.len());
        for item in self {
            if let Value::Array(nested) = item {
                flat.extend(nested.flatten());
            } else {
                flat.push(item);
            }
        }
        flat.into()
    }

    /// 同じ要素を逆順に並べた新しい配列を返します。
    #[func(title = "Reverse")]
    pub fn rev(self) -> Array {
        self.into_iter().rev().collect()
    }

    /// 指定された値が現れる位置で配列を分割します。
    ///
    /// ```example
    /// #(1, 1, 2, 3, 2, 4, 5).split(2)
    /// ```
    #[func]
    pub fn split(
        &self,
        /// 分割位置の値。
        at: Value,
    ) -> Array {
        self.as_slice()
            .split(|value| *value == at)
            .map(|subslice| Value::Array(subslice.iter().cloned().collect()))
            .collect()
    }

    /// 配列内の全ての要素を1つに統合します。
    #[func]
    pub fn join(
        self,
        /// 配列の各要素の間に挿入する値。
        #[default]
        separator: Option<Value>,
        /// 最後の2要素の間で用いる代替の区切り文字。
        #[named]
        last: Option<Value>,
        /// 配列が空の場合に返す値。
        #[named]
        default: Option<Value>,
    ) -> StrResult<Value> {
        let len = self.0.len();

        if let Some(result) = default
            && len == 0
        {
            return Ok(result);
        }

        let separator = separator.unwrap_or(Value::None);

        let mut last = last;
        let mut result = Value::None;
        for (i, value) in self.into_iter().enumerate() {
            if i > 0 {
                if i + 1 == len && last.is_some() {
                    result = ops::join(result, last.take().unwrap())?;
                } else {
                    result = ops::join(result, separator.clone())?;
                }
            }

            result = ops::join(result, value)?;
        }

        Ok(result)
    }

    /// 隣接する要素の間に区切り値のコピーを配置した配列を返します。
    ///
    /// ```example
    /// #("A", "B", "C").intersperse("-")
    /// ```
    #[func]
    pub fn intersperse(
        self,
        /// 隣接する各要素の間に配置される値。
        separator: Value,
    ) -> Array {
        // TODO: Use once stabilized:
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
        let size = match self.len() {
            0 => return Array::new(),
            n => (2 * n) - 1,
        };
        let mut vec = EcoVec::with_capacity(size);
        let mut iter = self.into_iter();

        if let Some(first) = iter.next() {
            vec.push(first);
        }

        for value in iter {
            vec.push(separator.clone());
            vec.push(value);
        }

        Array(vec)
    }

    /// 先頭から始まり単一の余りのチャンクで終わるように、配列を重なりのないチャンクに分割します。
    ///
    /// 最後のチャンクを除く全てのチャンクは`chunk-size`個の要素を持ちます。
    /// `exact`を`{true}`に設定した場合、要素数が`chunk-size`未満の余りは破棄されます。
    ///
    /// ```example
    /// #let array = (1, 2, 3, 4, 5, 6, 7, 8)
    /// #array.chunks(3) \
    /// #array.chunks(3, exact: true)
    /// ```
    #[func]
    pub fn chunks(
        self,
        /// 各チャンクが含み得る要素の最大数。
        chunk_size: NonZeroUsize,
        /// サイズが`chunk-size`未満の余りを保持するかどうか。
        #[named]
        #[default(false)]
        exact: bool,
    ) -> Array {
        let to_array = |chunk| Array::from(chunk).into_value();
        if exact {
            self.0.chunks_exact(chunk_size.get()).map(to_array).collect()
        } else {
            self.0.chunks(chunk_size.get()).map(to_array).collect()
        }
    }

    /// 配列上で`window-size`個の要素のスライディングウィンドウを返します。
    ///
    /// 配列の長さが`window-size`未満の場合、空の配列を返します。
    ///
    /// ```example
    /// #let array = (1, 2, 3, 4, 5, 6, 7, 8)
    /// #array.windows(5)
    /// ```
    #[func]
    pub fn windows(
        self,
        /// 各ウィンドウが含む要素数。
        window_size: NonZeroUsize,
    ) -> Array {
        self.0
            .windows(window_size.get())
            .map(|window| Array::from(window).into_value())
            .collect()
    }

    /// 必要に応じて与えられたキー関数によって、配列をソートしたものを返します。
    /// 用いるソートアルゴリズムは安定です。
    ///
    /// 2つの値を比較できなかった場合や、（指定されている場合に）キー関数や比較関数が
    /// エラーを発生させた場合、エラーを返します。
    ///
    /// 複数の基準で同時にソートするには（例えば一部の基準が同値の場合）、キー関数で
    /// 配列を返すことができます。結果は辞書順になります。
    ///
    /// ```example
    /// #let array = (
    ///   (a: 2, b: 4),
    ///   (a: 1, b: 5),
    ///   (a: 2, b: 3),
    /// )
    /// #array.sorted(key: it => (it.a, it.b))
    /// ```
    #[func]
    pub fn sorted(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        span: Span,
        /// 指定された場合、配列の各要素にこの関数を適用してソート基準のキーを決定します。
        #[named]
        key: Option<Func>,
        /// 指定された場合、配列内の任意の2要素を比較する際にこの関数を用います。
        ///
        /// 関数は配列内の2つの要素を比較用に受け取り、それらの順序を示すブール値を返します。
        /// `{true}`は要素が正しい順序にあることを示し、`{false}`は要素を入れ替えるべきことを示します。
        /// ソートの安定性を保つには、2つの要素が等しい場合、関数は`{true}`を返すべきです。
        ///
        /// この関数が要素を適切に順序付けない場合（例えば`{(x, y)}`と`{(y, x)}`の両方、
        /// または`{(x, x)}`に対して`{false}`を返す場合）、結果の配列の順序は未規定です。
        ///
        /// `key`と併用する場合、`by`には要素ではなくキーが渡されます。
        ///
        /// ```example
        /// #(
        ///   "sorted",
        ///   "by",
        ///   "decreasing",
        ///   "length",
        /// ).sorted(
        ///   key: s => s.len(),
        ///   by: (l, r) => l >= r,
        /// )
        /// ```
        #[named]
        by: Option<Func>,
    ) -> SourceResult<Array> {
        match by {
            Some(by) => {
                let mut are_in_order = |mut x, mut y| {
                    if let Some(f) = &key {
                        // We rely on `comemo`'s memoization of function
                        // evaluation to not excessively reevaluate the key.
                        x = f.call(engine, context, [x])?;
                        y = f.call(engine, context, [y])?;
                    }
                    match by.call(engine, context, [x, y])? {
                        Value::Bool(b) => Ok(b),
                        x => {
                            bail!(
                                span,
                                "expected boolean from `by` function, got {}",
                                x.ty(),
                            )
                        }
                    }
                };
                // If a comparison function is provided, we use `glidesort`
                // instead of the standard library sorting algorithm to prevent
                // panics in case the comparison function does not define a
                // valid order (see https://github.com/typst/typst/pull/5627).
                let mut result = Ok(());
                let mut vec = self.0.into_iter().enumerate().collect::<Vec<_>>();
                glidesort::sort_by(&mut vec, |(i, x), (j, y)| {
                    // Because we use booleans for the comparison function, in
                    // order to keep the sort stable, we need to compare in the
                    // right order.
                    if i < j {
                        // If `x` and `y` appear in this order in the original
                        // array, then we should change their order (i.e.,
                        // return `Ordering::Greater`) iff `y` is strictly less
                        // than `x` (i.e., `compare(x, y)` returns `false`).
                        // Otherwise, we should keep them in the same order
                        // (i.e., return `Ordering::Less`).
                        match are_in_order(x.clone(), y.clone()) {
                            Ok(false) => Ordering::Greater,
                            Ok(true) => Ordering::Less,
                            Err(err) => {
                                if result.is_ok() {
                                    result = Err(err);
                                }
                                Ordering::Equal
                            }
                        }
                    } else {
                        // If `x` and `y` appear in the opposite order in the
                        // original array, then we should change their order
                        // (i.e., return `Ordering::Less`) iff `x` is strictly
                        // less than `y` (i.e., `compare(y, x)` returns
                        // `false`). Otherwise, we should keep them in the same
                        // order (i.e., return `Ordering::Less`).
                        match are_in_order(y.clone(), x.clone()) {
                            Ok(false) => Ordering::Less,
                            Ok(true) => Ordering::Greater,
                            Err(err) => {
                                if result.is_ok() {
                                    result = Err(err);
                                }
                                Ordering::Equal
                            }
                        }
                    }
                });
                result.map(|()| vec.into_iter().map(|(_, x)| x).collect())
            }

            None => {
                let mut key_of = |x: Value| match &key {
                    // We rely on `comemo`'s memoization of function evaluation
                    // to not excessively reevaluate the key.
                    Some(f) => f.call(engine, context, [x]),
                    None => Ok(x),
                };
                // If no comparison function is provided, we know the order is
                // valid, so we can use the standard library sort and prevent an
                // extra allocation.
                let mut result = Ok(());
                let mut vec = self.0;
                vec.make_mut().sort_by(|a, b| {
                    match (key_of(a.clone()), key_of(b.clone())) {
                        (Ok(a), Ok(b)) => ops::compare(&a, &b).unwrap_or_else(|err| {
                            if result.is_ok() {
                                result = Err(err).at(span);
                            }
                            Ordering::Equal
                        }),
                        (Err(e), _) | (_, Err(e)) => {
                            if result.is_ok() {
                                result = Err(e);
                            }
                            Ordering::Equal
                        }
                    }
                });
                result.map(|()| vec.into())
            }
        }
    }

    /// 配列内の全ての要素を重複排除します。
    ///
    /// 全ての重複要素を取り除いた新しい配列を返します。
    /// 重複する各要素のうち最初の要素のみが保持されます。
    ///
    /// ```example
    /// #(3, 3, 1, 2, 3).dedup()
    /// ```
    #[func(title = "Deduplicate")]
    pub fn dedup(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// 指定された場合、配列の各要素にこの関数を適用して重複排除の基準となるキーを決定します。
        ///
        /// ```example
        /// #("apple", "banana", " apple ").dedup(key: s => s.trim())
        /// ```
        #[named]
        key: Option<Func>,
    ) -> SourceResult<Array> {
        let mut out = EcoVec::with_capacity(self.0.len());
        let mut key_of = |x: Value| match &key {
            // NOTE: We are relying on `comemo`'s memoization of function
            // evaluation to not excessively reevaluate the `key`.
            Some(f) => f.call(engine, context, [x]),
            None => Ok(x),
        };

        // This algorithm is O(N^2) because we cannot rely on `HashSet` since:
        // 1. We would like to preserve the order of the elements.
        // 2. We cannot hash arbitrary `Value`.
        'outer: for value in self {
            let key = key_of(value.clone())?;
            if out.is_empty() {
                out.push(value);
                continue;
            }

            for second in out.iter() {
                if ops::equal(&key, &key_of(second.clone())?) {
                    continue 'outer;
                }
            }

            out.push(value);
        }

        Ok(Self(out))
    }

    /// ペアの配列を辞書に変換します。
    /// 各ペアの最初の値がキー、2番目の値が値となります。
    ///
    /// 同じキーが複数回現れた場合、最後の値が選択されます。
    ///
    /// ```example
    /// #(
    ///   ("apples", 2),
    ///   ("peaches", 3),
    ///   ("apples", 5),
    /// ).to-dict()
    /// ```
    #[func]
    pub fn to_dict(self) -> StrResult<Dict> {
        self.into_iter()
            .map(|value| {
                let value_ty = value.ty();
                let pair = value.cast::<Array>().map_err(|_| {
                    eco_format!("expected (str, any) pairs, found {}", value_ty)
                })?;
                if let [key, value] = pair.as_slice() {
                    let key = key.clone().cast::<Str>().map_err(|_| {
                        eco_format!("expected key of type str, found {}", value.ty())
                    })?;
                    Ok((key, value.clone()))
                } else {
                    bail!("expected pairs of length 2, found length {}", pair.len());
                }
            })
            .collect()
    }

    /// reduce演算を繰り返し適用することで、要素を単一の値に集約します。
    ///
    /// 配列が空の場合は`{none}`を返します。そうでなければ集約の結果を返します。
    ///
    /// reduce関数は2つの引数を取るクロージャです。1つは「累積値」、もう1つは要素です。
    ///
    /// 少なくとも1つの要素を持つ配列の場合、これは配列の最初の要素を初期累積値とした
    /// [`array.fold`]と同じであり、それ以降の各要素を畳み込みます。
    ///
    /// ```example
    /// #let array = (2, 1, 4, 3)
    /// #array.reduce((acc, x) => calc.max(acc, x))
    /// ```
    #[func]
    pub fn reduce(
        self,
        engine: &mut Engine,
        context: Tracked<Context>,
        /// reduce関数。2つの引数を取らなければなりません。
        /// 1つは累積値、もう1つは要素です。
        reducer: Func,
    ) -> SourceResult<Value> {
        let mut iter = self.into_iter();
        let mut acc = iter.next().unwrap_or_default();
        for item in iter {
            acc = reducer.call(engine, context, [acc, item])?;
        }
        Ok(acc)
    }
}

/// 配列にキャスト可能な値。
pub struct ToArray(Array);

cast! {
    ToArray,
    v: Array => Self(v),
    v: Bytes => Self(v.iter().map(|&b| Value::Int(b.into())).collect()),
    v: Version => Self(v.values().iter().map(|&v| Value::Int(v as i64)).collect())
}

impl Debug for Array {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl Repr for Array {
    fn repr(&self) -> EcoString {
        let max = 40;
        let mut pieces: Vec<_> = self
            .iter()
            .take(max)
            .map(|value| eco_format!("{}", value.repr()))
            .collect();
        if self.len() > max {
            pieces.push(eco_format!(".. ({} items omitted)", self.len() - max));
        }
        repr::pretty_array_like(&pieces, self.len() == 1).into()
    }
}

impl Add for Array {
    type Output = Self;

    fn add(mut self, rhs: Array) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Array {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0);
    }
}

impl Extend<Value> for Array {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = ecow::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<EcoVec<Value>> for Array {
    fn from(v: EcoVec<Value>) -> Self {
        Array(v)
    }
}

impl From<&[Value]> for Array {
    fn from(v: &[Value]) -> Self {
        Array(v.into())
    }
}

impl<T> Reflect for Vec<T> {
    fn input() -> CastInfo {
        Array::input()
    }

    fn output() -> CastInfo {
        Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value)
    }
}

impl<T: Reflect, const N: usize> Reflect for SmallVec<[T; N]> {
    fn input() -> CastInfo {
        Array::input()
    }

    fn output() -> CastInfo {
        Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: IntoValue, const N: usize> IntoValue for SmallVec<[T; N]> {
    fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        value.cast::<Array>()?.into_iter().map(Value::cast).collect()
    }
}

impl<T: FromValue, const N: usize> FromValue for SmallVec<[T; N]> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        value.cast::<Array>()?.into_iter().map(Value::cast).collect()
    }
}

/// 1つの要素、または配列として提供される複数の要素。
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OneOrMultiple<T>(pub Vec<T>);

impl<T: Reflect> Reflect for OneOrMultiple<T> {
    fn input() -> CastInfo {
        T::input() + Array::input()
    }

    fn output() -> CastInfo {
        T::output() + Array::output()
    }

    fn castable(value: &Value) -> bool {
        Array::castable(value) || T::castable(value)
    }
}

impl<T: IntoValue + Clone> IntoValue for OneOrMultiple<T> {
    fn into_value(self) -> Value {
        self.0.into_value()
    }
}

impl<T: FromValue> FromValue for OneOrMultiple<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        if T::castable(&value) {
            return Ok(Self(vec![T::from_value(value)?]));
        }
        if Array::castable(&value) {
            return Ok(Self(
                Array::from_value(value)?
                    .into_iter()
                    .map(|value| T::from_value(value))
                    .collect::<HintedStrResult<_>>()?,
            ));
        }
        Err(Self::error(&value))
    }
}

impl<T> Default for OneOrMultiple<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

/// The error message when the array is empty.
#[cold]
fn array_is_empty() -> EcoString {
    "array is empty".into()
}

/// The out of bounds access error message.
#[cold]
fn out_of_bounds(index: i64, len: usize) -> EcoString {
    eco_format!("array index out of bounds (index: {index}, len: {len})")
}

/// The out of bounds access error message when no default value was given.
#[cold]
fn out_of_bounds_no_default(index: i64, len: usize) -> EcoString {
    eco_format!(
        "array index out of bounds (index: {index}, len: {len}) \
         and no default value was specified",
    )
}
