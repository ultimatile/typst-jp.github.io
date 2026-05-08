use std::num::ParseFloatError;

use ecow::{EcoString, eco_format};

use crate::diag::{StrResult, bail};
use crate::foundations::{
    Bytes, Decimal, Endianness, Repr, Str, cast, func, repr, scope, ty,
};
use crate::layout::Ratio;

/// 浮動小数点数。
///
/// 実数の精度に限りのある表現です。Typstは浮動小数点数を64ビットで保持します。
/// 浮動小数点数が期待される箇所では、[整数]($int)を渡すこともできます。
///
/// 値はこの型のコンストラクターを使って浮動小数点数に変換できます。
///
/// NaNと正の無限大は、それぞれ`{float.nan}`と`{float.inf}`として利用できます。
///
/// # 例
/// ```example
/// #3.14 \
/// #1e4 \
/// #(10 / 4)
/// ```
#[ty(scope, cast, name = "float")]
type f64;

#[scope]
impl f64 {
    /// 正の無限大。
    const INF: f64 = f64::INFINITY;

    /// [IEEE 754標準](https://en.wikipedia.org/wiki/IEEE_754)で定義される
    /// NaN値。
    const NAN: f64 = f64::NAN;

    /// 値を浮動小数点数に変換します。
    ///
    /// - ブール値は`0.0`または`1.0`に変換されます。
    /// - 整数は、最も近い64ビット浮動小数点数に変換されます。絶対値が
    ///   `{calc.pow(2, 53)}`未満の整数では、この変換は正確です。
    /// - 比率は100%で除算されます。
    /// - 文字列は10進数として、最も近い64ビット浮動小数点数にパースされます。
    ///   指数表記もサポートされます。
    ///
    /// ```example
    /// #float(false) \
    /// #float(true) \
    /// #float(4) \
    /// #float(40%) \
    /// #float("2.7") \
    /// #float("1e5")
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 浮動小数点数に変換する値。
        value: ToFloat,
    ) -> f64 {
        value.0
    }

    /// 浮動小数点数がNaNかどうかを判定します。
    ///
    /// IEEE 754では、複数のビットパターンがNaNを表します。
    /// この関数は、浮動小数点数がそれらのビットパターンのいずれかである場合に
    /// `true`を返します。
    ///
    /// ```example
    /// #float.is-nan(0) \
    /// #float.is-nan(1) \
    /// #float.is-nan(float.nan)
    /// ```
    #[func]
    pub fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    /// 浮動小数点数が無限大かどうかを判定します。
    ///
    /// 浮動小数点数は正の無限大と負の無限大を表現できます。
    /// この関数は、浮動小数点数が無限大である場合に`{true}`を返します。
    ///
    /// ```example
    /// #float.is-infinite(0) \
    /// #float.is-infinite(1) \
    /// #float.is-infinite(float.inf)
    /// ```
    #[func]
    pub fn is_infinite(self) -> bool {
        f64::is_infinite(self)
    }

    /// 浮動小数点数の符号を計算します。
    ///
    /// - 数値が正である場合（`{+0.0}`を含む）、`{1.0}`を返します。
    /// - 数値が負である場合（`{-0.0}`を含む）、`{-1.0}`を返します。
    /// - 数値がNaNの場合、`{float.nan}`を返します。
    ///
    /// ```example
    /// #(5.0).signum() \
    /// #(-5.0).signum() \
    /// #(0.0).signum() \
    /// #float.nan.signum()
    /// ```
    #[func]
    pub fn signum(self) -> f64 {
        f64::signum(self)
    }

    /// バイト列を浮動小数点数として解釈します。
    ///
    /// ```example
    /// #float.from-bytes(bytes((0, 0, 0, 0, 0, 0, 240, 63))) \
    /// #float.from-bytes(bytes((63, 240, 0, 0, 0, 0, 0, 0)), endian: "big")
    /// ```
    #[func]
    pub fn from_bytes(
        /// 浮動小数点数に変換するバイト列。
        ///
        /// 長さは4または8でなければなりません。バイト列は、その長さに応じて
        /// [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)のbinary32（単精度）
        /// またはbinary64（倍精度）形式として解釈されます。
        bytes: Bytes,
        /// 変換のエンディアン。
        #[named]
        #[default(Endianness::Little)]
        endian: Endianness,
    ) -> StrResult<f64> {
        // Convert slice to an array of length 4 or 8.
        if let Ok(buffer) = <[u8; 8]>::try_from(bytes.as_ref()) {
            return Ok(match endian {
                Endianness::Little => f64::from_le_bytes(buffer),
                Endianness::Big => f64::from_be_bytes(buffer),
            });
        };
        if let Ok(buffer) = <[u8; 4]>::try_from(bytes.as_ref()) {
            return Ok(match endian {
                Endianness::Little => f32::from_le_bytes(buffer),
                Endianness::Big => f32::from_be_bytes(buffer),
            } as f64);
        };

        bail!("bytes must have a length of 4 or 8");
    }

    /// 浮動小数点数をバイト列に変換します。
    ///
    /// ```example
    /// #array(1.0.to-bytes(endian: "big")) \
    /// #array(1.0.to-bytes())
    /// ```
    #[func]
    pub fn to_bytes(
        self,
        /// 変換のエンディアン。
        #[named]
        #[default(Endianness::Little)]
        endian: Endianness,
        /// 結果のバイト列のサイズ。
        ///
        /// これは4または8でなければなりません。指定されたサイズに応じて、
        /// 呼び出しは浮動小数点数を[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
        /// のbinary32（単精度）またはbinary64（倍精度）形式で表したものを返します。
        #[named]
        #[default(8)]
        size: u32,
    ) -> StrResult<Bytes> {
        Ok(match size {
            8 => Bytes::new(match endian {
                Endianness::Little => self.to_le_bytes(),
                Endianness::Big => self.to_be_bytes(),
            }),
            4 => Bytes::new(match endian {
                Endianness::Little => (self as f32).to_le_bytes(),
                Endianness::Big => (self as f32).to_be_bytes(),
            }),
            _ => bail!("size must be either 4 or 8"),
        })
    }
}

impl Repr for f64 {
    fn repr(&self) -> EcoString {
        repr::format_float(*self, None, true, "")
    }
}

/// A value that can be cast to a float.
pub struct ToFloat(f64);

cast! {
    ToFloat,
    v: f64 => Self(v),
    v: bool => Self(v as i64 as f64),
    v: i64 => Self(v as f64),
    v: Decimal => Self(f64::try_from(v).map_err(|_| eco_format!("invalid float: {}", v))?),
    v: Ratio => Self(v.get()),
    v: Str => Self(
        parse_float(v.clone().into())
            .map_err(|_| eco_format!("invalid float: {}", v))?
    ),
}

fn parse_float(s: EcoString) -> Result<f64, ParseFloatError> {
    s.replace(repr::MINUS_SIGN, "-").parse()
}

/// A floating-point number that must be positive (strictly larger than zero).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PositiveF64(f64);

impl PositiveF64 {
    /// Wrap a float if it is positive.
    pub fn new(value: f64) -> Option<Self> {
        (value > 0.0).then_some(Self(value))
    }

    /// Get the underlying value.
    pub fn get(self) -> f64 {
        self.0
    }
}

cast! {
    PositiveF64,
    self => self.get().into_value(),
    v: f64 => Self::new(v).ok_or("number must be positive")?,
}
