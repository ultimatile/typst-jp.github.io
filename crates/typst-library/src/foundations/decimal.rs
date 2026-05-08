use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Neg;
use std::str::FromStr;

use ecow::{EcoString, eco_format};
use rust_decimal::MathematicalOps;
use typst_syntax::{Span, Spanned, ast};

use crate::World;
use crate::diag::{At, SourceResult, warning};
use crate::engine::Engine;
use crate::foundations::{Repr, Str, cast, func, repr, scope, ty};

/// 固定小数点decimal型。
///
/// この型は、10進数で表される数値の正確な算術演算に用いるべきです。
/// 典型的なユースケースは通貨の表現です。
///
/// # 例
/// ```example
/// Decimal: #(decimal("0.1") + decimal("0.2")) \
/// Float: #(0.1 + 0.2)
/// ```
///
/// # 構築とキャスト
/// decimalを生成するには、`{decimal(string)}`コンストラクターを用います。
/// 例えば`{decimal("3.141592653")}`のように記述します**（ダブルクォートに注意）**。
/// このコンストラクターは、後述の制限の範囲内で表現可能であれば、
/// 与えられた小数部の桁を全て保持します（そうでない場合はエラーが発生します）。
///
/// `{decimal(int)}`コンストラクターで、任意の[整数]($int)をdecimalに変換することもできます
/// （例：`{decimal(59)}`）。
/// ただし、[浮動小数点数]($float)からdecimalを構築することは、サポートされてはいるものの、
/// **不正確な変換であるため推奨されません**。
/// `{decimal(3.14)}`のように記述した場合（ダブルクォートがなく、これが意図しない`float`
/// キャストであり、不正確であることを示します）、Typstが意図しない`float`から`decimal`への
/// キャストを検出すると警告が発生します。
/// 代わりに、定数のdecimal値には文字列を使うことが推奨されます（例：`{decimal("3.14")}`）。
///
/// `float`から`decimal`へのキャストの精度は、結果を[`calc.round`]で15桁に丸めることで
/// わずかに改善できますが、この種の変換に対して精度の保証は依然としてありません。
///
/// # 演算
/// 2つのdecimal同士、およびdecimalと整数の組に対して基本的な算術演算がサポートされています。
///
/// 不慮の精度低下を防ぐため、`float`と`decimal`との間の組み込み演算はサポートされていません。
/// 代わりにエラーが発生します。
///
/// 三角関数や2つの実数間の冪乗のような特定の`calc`関数も`float`に対してのみサポートされています
/// （ただし、`decimal`の整数冪はサポートされています）。
/// `{float(decimal)}`コンストラクターを用いると、`decimal`を`float`にキャストし、
/// 精度保証のない演算を許容することで、不正確になる可能性のある演算を選べます。
///
/// # decimalの表示
/// decimalを表示するには、単にその値を文書に挿入します。
/// 特定の桁数のみを表示するには、最初にdecimalを[丸めて]($calc.round)ください。
/// decimalや他の数値のロケール対応書式設定はまだサポートされていませんが、将来的に予定されています。
///
/// [`str`]コンストラクターを用いて、decimalを文字列に変換できます。
/// これにより、表示形式を後処理できます。
/// 例えば、ピリオドをカンマに置換できます（カンマを使う言語向けの適切な組み込み
/// ロケール対応の代わりとして）。
///
/// # 精度と制限
/// `decimal`は、10進数で28〜29桁の有効桁数の制限を持ちます。
/// これは小数点の前後の桁数の合計を含みます。
/// したがって、小数部の桁数が多い数値ほど範囲が小さくなります。
/// 最大および最小の`decimal`の値はそれぞれ`{79228162514264337593543950335}`と
/// `{-79228162514264337593543950335}`です。
/// [`float`]とは異なり、この型は無限大やNaNをサポートしないため、
/// オーバーフローやアンダーフローを起こす演算ではエラーが発生します。
///
/// 加算、乗算、整数への[冪乗]($calc.pow)などの`decimal`同士の典型的な演算は、
/// 固定小数点表現のために高精度で行われます。
/// ただし、乗算や除算は、いくつかのエッジケースで全ての桁を保持しないことに注意してください。
/// これらは正確と見なされますが、上記の制限を超える桁は丸められて失われるため、
/// 表現可能な最大桁数を超える精度の低下が起こり得ます。
/// この挙動は除算時だけでなく、0と1の間の数を乗算する際にも観察できます。
/// どちらの演算も数値の小数部の桁を上記の制限を超えて押し上げ、丸めにつながる可能性があるためです。
/// これら2つの演算が桁数の制限を超えない場合、それらは完全に正確です。
#[ty(scope, cast)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    pub const ZERO: Self = Self(rust_decimal::Decimal::ZERO);
    pub const ONE: Self = Self(rust_decimal::Decimal::ONE);
    pub const MIN: Self = Self(rust_decimal::Decimal::MIN);
    pub const MAX: Self = Self(rust_decimal::Decimal::MAX);

    /// Whether this decimal value is zero.
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Whether this decimal value is negative.
    pub const fn is_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    /// Whether this decimal has fractional part equal to zero (is an integer).
    pub fn is_integer(self) -> bool {
        self.0.is_integer()
    }

    /// Computes the absolute value of this decimal.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Computes the largest integer less than or equal to this decimal.
    ///
    /// A decimal is returned as this may not be within `i64`'s range of
    /// values.
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Computes the smallest integer greater than or equal to this decimal.
    ///
    /// A decimal is returned as this may not be within `i64`'s range of
    /// values.
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns the integer part of this decimal.
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    /// Returns the fractional part of this decimal (with the integer part set
    /// to zero).
    pub fn fract(self) -> Self {
        Self(self.0.fract())
    }

    /// Rounds this decimal up to the specified amount of digits with the
    /// traditional rounding rules, using the "midpoint away from zero"
    /// strategy (6.5 -> 7, -6.5 -> -7).
    ///
    /// If given a negative amount of digits, rounds to integer digits instead
    /// with the same rounding strategy. For example, rounding to -3 digits
    /// will turn 34567.89 into 35000.00 and -34567.89 into -35000.00.
    ///
    /// Note that this can return `None` when using negative digits where the
    /// rounded number would overflow the available range for decimals.
    pub fn round(self, digits: i32) -> Option<Self> {
        // Positive digits can be handled by just rounding with rust_decimal.
        if let Ok(positive_digits) = u32::try_from(digits) {
            return Some(Self(self.0.round_dp_with_strategy(
                positive_digits,
                rust_decimal::RoundingStrategy::MidpointAwayFromZero,
            )));
        }

        // We received negative digits, so we round to integer digits.
        let mut num = self.0;
        let old_scale = num.scale();
        let digits = -digits as u32;

        let (Ok(_), Some(ten_to_digits)) = (
            // Same as dividing by 10^digits.
            num.set_scale(old_scale + digits),
            rust_decimal::Decimal::TEN.checked_powi(digits as i64),
        ) else {
            // Scaling more than any possible amount of integer digits.
            let mut zero = rust_decimal::Decimal::ZERO;
            zero.set_sign_negative(self.is_negative());
            return Some(Self(zero));
        };

        // Round to this integer digit.
        num = num.round_dp_with_strategy(
            0,
            rust_decimal::RoundingStrategy::MidpointAwayFromZero,
        );

        // Multiply by 10^digits again, which can overflow and fail.
        num.checked_mul(ten_to_digits).map(Self)
    }

    /// Attempts to add two decimals.
    ///
    /// Returns `None` on overflow or underflow.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    /// Attempts to subtract a decimal from another.
    ///
    /// Returns `None` on overflow or underflow.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    /// Attempts to multiply two decimals.
    ///
    /// Returns `None` on overflow or underflow.
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        self.0.checked_mul(other.0).map(Self)
    }

    /// Attempts to divide two decimals.
    ///
    /// Returns `None` if `other` is zero, as well as on overflow or underflow.
    pub fn checked_div(self, other: Self) -> Option<Self> {
        self.0.checked_div(other.0).map(Self)
    }

    /// Attempts to obtain the quotient of Euclidean division between two
    /// decimals. Implemented similarly to [`f64::div_euclid`].
    ///
    /// The returned quotient is truncated and adjusted if the remainder was
    /// negative.
    ///
    /// Returns `None` if `other` is zero, as well as on overflow or underflow.
    pub fn checked_div_euclid(self, other: Self) -> Option<Self> {
        let q = self.0.checked_div(other.0)?.trunc();
        if self
            .0
            .checked_rem(other.0)
            .as_ref()
            .is_some_and(rust_decimal::Decimal::is_sign_negative)
        {
            return if other.0.is_sign_positive() {
                q.checked_sub(rust_decimal::Decimal::ONE).map(Self)
            } else {
                q.checked_add(rust_decimal::Decimal::ONE).map(Self)
            };
        }
        Some(Self(q))
    }

    /// Attempts to obtain the remainder of Euclidean division between two
    /// decimals. Implemented similarly to [`f64::rem_euclid`].
    ///
    /// The returned decimal `r` is non-negative within the range
    /// `0.0 <= r < other.abs()`.
    ///
    /// Returns `None` if `other` is zero, as well as on overflow or underflow.
    pub fn checked_rem_euclid(self, other: Self) -> Option<Self> {
        let r = self.0.checked_rem(other.0)?;
        Some(Self(if r.is_sign_negative() { r.checked_add(other.0.abs())? } else { r }))
    }

    /// Attempts to calculate the remainder of the division of two decimals.
    ///
    /// Returns `None` if `other` is zero, as well as on overflow or underflow.
    pub fn checked_rem(self, other: Self) -> Option<Self> {
        self.0.checked_rem(other.0).map(Self)
    }

    /// Attempts to take one decimal to the power of an integer.
    ///
    /// Returns `None` for invalid operands, as well as on overflow or
    /// underflow.
    pub fn checked_powi(self, other: i64) -> Option<Self> {
        self.0.checked_powi(other).map(Self)
    }
}

#[scope]
impl Decimal {
    /// 値を`decimal`に変換します。
    ///
    /// decimalの構築には文字列、または（必要なら）[整数]($int)を用いることが推奨されます。
    /// 文字列は`{"3.14159"}`（負の数の場合は`{"-3.141519"}`）の形式の数を含む必要があります。
    /// 小数部の桁数は完全に保持されます。
    /// 有効桁数の制限（28〜29桁程度）に達したためにそれが不可能な場合、
    /// 与えられたdecimalは表現できないためエラーが発生します。
    ///
    /// このコンストラクターは[浮動小数点数]($float)を`decimal`にキャストする用途にも使えますが、
    /// **このキャストは本質的に不正確である**ため**推奨されません**。
    /// `{decimal(1.234)}`のように記述すると（ダブルクォートがない点に注意）、
    /// 意図せずこのキャストを行ってしまいやすいので、Typstはその場合警告を発します。
    /// この特定のケース（定数decimalの初期化）では、代わりに`{decimal("1.234")}`と記述してください。
    /// また、NaNや無限大の浮動小数点数はdecimalにキャストできず、エラーが発生する点に注意してください。
    ///
    /// ```example
    /// #decimal("1.222222222222222")
    /// ```
    #[func(constructor)]
    pub fn construct(
        engine: &mut Engine,
        /// decimalに変換する値。
        value: Spanned<ToDecimal>,
    ) -> SourceResult<Decimal> {
        match value.v {
            ToDecimal::Str(str) => Self::from_str(&str.replace(repr::MINUS_SIGN, "-"))
                .map_err(|_| eco_format!("invalid decimal: {str}"))
                .at(value.span),
            ToDecimal::Int(int) => Ok(Self::from(int)),
            ToDecimal::Float(float) => {
                warn_on_float_literal(engine, value.span);
                Self::try_from(float)
                    .map_err(|_| {
                        eco_format!(
                            "float is not a valid decimal: {}",
                            repr::format_float(float, None, true, "")
                        )
                    })
                    .at(value.span)
            }
            ToDecimal::Decimal(decimal) => Ok(decimal),
        }
    }
}

/// Emits a warning when a decimal is constructed from a float literal.
fn warn_on_float_literal(engine: &mut Engine, span: Span) -> Option<()> {
    let id = span.id()?;
    let source = engine.world.source(id).ok()?;
    let node = source.find(span)?;
    if node.is::<ast::Float>() {
        engine.sink.warn(warning!(
            span,
            "creating a decimal using imprecise float literal";
            hint: "use a string in the decimal constructor to avoid loss \
                   of precision: `decimal({})`",
            node.text().repr()
        ));
    }
    Some(())
}

impl FromStr for Decimal {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rust_decimal::Decimal::from_str_exact(s).map(Self)
    }
}

impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        Self(rust_decimal::Decimal::from(value))
    }
}

impl TryFrom<f64> for Decimal {
    type Error = ();

    /// Attempts to convert a Decimal to a float.
    ///
    /// This can fail if the float is infinite or NaN, or otherwise cannot be
    /// represented by a decimal number.
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        rust_decimal::Decimal::from_f64_retain(value).map(Self).ok_or(())
    }
}

impl TryFrom<Decimal> for f64 {
    type Error = rust_decimal::Error;

    /// Attempts to convert a Decimal to a float.
    ///
    /// This should in principle be infallible according to the implementation,
    /// but we mirror the decimal implementation's API either way.
    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<Decimal> for i64 {
    type Error = rust_decimal::Error;

    /// Attempts to convert a Decimal to an integer.
    ///
    /// Returns an error if the decimal has a fractional part, or if there
    /// would be overflow or underflow.
    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_sign_negative() {
            f.write_str(repr::MINUS_SIGN)?;
        }
        self.0.abs().fmt(f)
    }
}

impl Repr for Decimal {
    fn repr(&self) -> EcoString {
        eco_format!("decimal({})", eco_format!("{}", self.0).repr())
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Hash for Decimal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // `rust_decimal`'s Hash implementation normalizes decimals before
        // hashing them. This means decimals with different scales but
        // equivalent value not only compare equal but also hash equally. Here,
        // we hash all bytes explicitly to ensure the scale is also considered.
        // This means that 123.314 == 123.31400, but 123.314.hash() !=
        // 123.31400.hash().
        //
        // Note that this implies that equal decimals can have different hashes,
        // which might generate problems with certain data structures, such as
        // HashSet and HashMap.
        self.0.serialize().hash(state);
    }
}

/// decimalにキャスト可能な値。
pub enum ToDecimal {
    /// それ自身に変換されるdecimal。
    Decimal(Decimal),
    /// decimalの表現を含む文字列。
    Str(EcoString),
    /// 等価なdecimalに変換される整数。
    Int(i64),
    /// 等価なdecimalに変換される浮動小数点数。
    Float(f64),
}

cast! {
    ToDecimal,
    v: Decimal => Self::Decimal(v),
    v: i64 => Self::Int(v),
    v: bool => Self::Int(v as i64),
    v: f64 => Self::Float(v),
    v: Str => Self::Str(EcoString::from(v)),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use typst_utils::hash128;

    use super::Decimal;

    #[test]
    fn test_decimals_with_equal_scales_hash_identically() {
        let a = Decimal::from_str("3.14").unwrap();
        let b = Decimal::from_str("3.14").unwrap();
        assert_eq!(a, b);
        assert_eq!(hash128(&a), hash128(&b));
    }

    #[test]
    fn test_decimals_with_different_scales_hash_differently() {
        let a = Decimal::from_str("3.140").unwrap();
        let b = Decimal::from_str("3.14000").unwrap();
        assert_eq!(a, b);
        assert_ne!(hash128(&a), hash128(&b));
    }

    #[track_caller]
    fn test_round(value: &str, digits: i32, expected: &str) {
        assert_eq!(
            Decimal::from_str(value).unwrap().round(digits),
            Some(Decimal::from_str(expected).unwrap()),
        );
    }

    #[test]
    fn test_decimal_positive_round() {
        test_round("312.55553", 0, "313.00000");
        test_round("312.55553", 3, "312.556");
        test_round("312.5555300000", 3, "312.556");
        test_round("-312.55553", 3, "-312.556");
        test_round("312.55553", 28, "312.55553");
        test_round("312.55553", 2341, "312.55553");
        test_round("-312.55553", 2341, "-312.55553");
    }

    #[test]
    fn test_decimal_negative_round() {
        test_round("4596.55553", -1, "4600");
        test_round("4596.555530000000", -1, "4600");
        test_round("-4596.55553", -3, "-5000");
        test_round("4596.55553", -28, "0");
        test_round("-4596.55553", -2341, "0");
        assert_eq!(Decimal::MAX.round(-1), None);
    }
}
