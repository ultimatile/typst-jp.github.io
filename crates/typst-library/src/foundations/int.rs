use std::num::{
    NonZeroI64, NonZeroIsize, NonZeroU32, NonZeroU64, NonZeroUsize, ParseIntError,
};

use ecow::{EcoString, eco_format};
use smallvec::SmallVec;

use crate::diag::{StrResult, bail};
use crate::foundations::{
    Bytes, Cast, Decimal, Repr, Str, Value, cast, func, repr, scope, ty,
};

/// 整数。
///
/// この数は負、ゼロ、正のいずれにもなります。
/// Typstは整数の格納に64ビットを使用しているため、整数は`{-9223372036854775808}`より小さくも、
/// `{9223372036854775807}`より大きくもなれません。
/// 整数リテラルは常に正であるため、`{-1}`のような負の整数は、
/// 意味的には正のリテラル`1`に対する否定`-`の適用です。
/// 最大値より大きい正の整数や、最小値以下の負の整数は整数リテラルとして表現できず、
/// 代わりに`{float}`としてパースされます。
/// 最小の整数値は整数の演算によって得ることもできます。
///
/// この数は、ゼロの後ろに`x`、`o`、`b`のいずれかを続けることで、
/// 16進数、8進数、2進数として指定することもできます。
///
/// この型のコンストラクターを用いて、値を整数に変換できます。
///
/// # 例
/// ```example
/// #(1 + 2) \
/// #(2 - 5) \
/// #(3 + 4 < 8)
///
/// #0xff \
/// #0o10 \
/// #0b1001
/// ```
#[ty(scope, cast, name = "int", title = "Integer")]
type i64;

#[scope]
impl i64 {
    /// 値を整数に変換します。64ビット符号付き整数の最大値より大きい整数や、
    /// 最小値より小さい整数を生成しようとした場合はエラーを発生させます。
    ///
    /// - ブール値は`0`または`1`に変換されます。
    /// - 浮動小数点数とdecimalはゼロ方向に最も近い64ビット整数に丸められます。
    /// - 文字列は10進数としてパースされます。
    ///
    /// ```example
    /// #int(false) \
    /// #int(true) \
    /// #int(2.7) \
    /// #int(decimal("3.8")) \
    /// #(int("27") + int("4"))
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// 整数に変換する値。
        value: ToInt,
    ) -> i64 {
        value.0
    }

    /// 整数の符号を計算します。
    ///
    /// - 数が正の場合、`{1}`を返します。
    /// - 数が負の場合、`{-1}`を返します。
    /// - 数がゼロの場合、`{0}`を返します。
    ///
    /// ```example
    /// #(5).signum() \
    /// #(-5).signum() \
    /// #(0).signum()
    /// ```
    #[func]
    pub fn signum(self) -> i64 {
        i64::signum(self)
    }

    /// 整数のビットごとのNOTを計算します。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    ///
    /// ```example
    /// #4.bit-not() \
    /// #(-1).bit-not()
    /// ```
    #[func(title = "Bitwise NOT")]
    pub fn bit_not(self) -> i64 {
        !self
    }

    /// 2つの整数のビットごとのANDを計算します。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    ///
    /// ```example
    /// #128.bit-and(192)
    /// ```
    #[func(title = "Bitwise AND")]
    pub fn bit_and(
        self,
        /// ビットごとのANDの右辺オペランド。
        rhs: i64,
    ) -> i64 {
        self & rhs
    }

    /// 2つの整数のビットごとのORを計算します。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    ///
    /// ```example
    /// #64.bit-or(32)
    /// ```
    #[func(title = "Bitwise OR")]
    pub fn bit_or(
        self,
        /// ビットごとのORの右辺オペランド。
        rhs: i64,
    ) -> i64 {
        self | rhs
    }

    /// 2つの整数のビットごとのXORを計算します。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    ///
    /// ```example
    /// #64.bit-xor(96)
    /// ```
    #[func(title = "Bitwise XOR")]
    pub fn bit_xor(
        self,
        /// ビットごとのXORの右辺オペランド。
        rhs: i64,
    ) -> i64 {
        self ^ rhs
    }

    /// オペランドのビットを指定した量だけ左にシフトします。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    /// 結果が大きすぎて64ビット整数に収まらない場合はエラーになります。
    ///
    /// ```example
    /// #33.bit-lshift(2) \
    /// #(-1).bit-lshift(3)
    /// ```
    #[func(title = "Bitwise Left Shift")]
    pub fn bit_lshift(
        self,
        /// シフトするビット数。負であってはなりません。
        shift: u32,
    ) -> StrResult<i64> {
        Ok(self.checked_shl(shift).ok_or("the result is too large")?)
    }

    /// オペランドのビットを指定した量だけ右にシフトします。
    /// 既定では算術シフトを行います（符号ビットを左に拡張するため、負の数は負のままです）。
    /// この挙動は`logical`引数で変更できます。
    ///
    /// この関数では、オペランドは64ビット符号付き整数として扱われます。
    ///
    /// ```example
    /// #64.bit-rshift(2) \
    /// #(-8).bit-rshift(2) \
    /// #(-8).bit-rshift(2, logical: true)
    /// ```
    #[func(title = "Bitwise Right Shift")]
    pub fn bit_rshift(
        self,
        /// シフトするビット数。負であってはなりません。
        ///
        /// 63より大きいシフトも許容され、戻り値は飽和します。
        /// 非負の数では、戻り値は`{0}`に飽和します。
        /// 負の数では、`logical`が`{false}`の場合は`{-1}`、
        /// `{true}`の場合は`{0}`に飽和します。
        /// この挙動は、この演算を複数回適用した場合と一致します。
        /// したがって、シフトは常に成功します。
        shift: u32,
        /// 算術右シフトの代わりに論理（符号なし）右シフトを行うかどうかを切り替えます。
        /// `{true}`の場合、負のオペランドは符号ビットを保持せず、
        /// シフト後に左側に現れるビットは`{0}`になります。
        /// この引数は非負のオペランドには影響しません。
        #[named]
        #[default(false)]
        logical: bool,
    ) -> i64 {
        if logical {
            if shift >= u64::BITS {
                // Excessive logical right shift would be equivalent to setting
                // all bits to zero. Using `.min(63)` is not enough for logical
                // right shift, since `-1 >> 63` returns 1, whereas
                // `calc.bit-rshift(-1, 64)` should return the same as
                // `(-1 >> 63) >> 1`, which is zero.
                0
            } else {
                // Here we reinterpret the signed integer's bits as unsigned to
                // perform logical right shift, and then reinterpret back as signed.
                // This is valid as, according to the Rust reference, casting between
                // two integers of same size (i64 <-> u64) is a no-op (two's complement
                // is used).
                // Reference:
                // https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#numeric-cast
                ((self as u64) >> shift) as i64
            }
        } else {
            // Saturate at -1 (negative) or 0 (otherwise) on excessive arithmetic
            // right shift. Shifting those numbers any further does not change
            // them, so it is consistent.
            let shift = shift.min(i64::BITS - 1);
            self >> shift
        }
    }

    /// バイト列を整数に変換します。
    ///
    /// ```example
    /// #int.from-bytes(bytes((0, 0, 0, 0, 0, 0, 0, 1))) \
    /// #int.from-bytes(bytes((1, 0, 0, 0, 0, 0, 0, 0)), endian: "big")
    /// ```
    #[func]
    pub fn from_bytes(
        /// 整数に変換するバイト列。
        ///
        /// 結果が64ビット符号付き整数に収まるように、長さは最大で8でなければなりません。
        bytes: Bytes,
        /// 変換のエンディアン。
        #[named]
        #[default(Endianness::Little)]
        endian: Endianness,
        /// バイト列を符号付き整数として扱うかどうか。
        /// `{true}`で最上位ビットが立っている場合、結果の数値は負になります。
        #[named]
        #[default(true)]
        signed: bool,
    ) -> StrResult<i64> {
        let len = bytes.len();
        if len == 0 {
            return Ok(0);
        } else if len > 8 {
            bail!("too many bytes to convert to a 64 bit number");
        }

        // `decimal` will hold the part of the buffer that should be filled with
        // the input bytes, `rest` will remain as is or be filled with 0xFF for
        // negative numbers if signed is true.
        //
        // – big-endian: `decimal` will be the rightmost bytes of the buffer.
        // - little-endian: `decimal` will be the leftmost bytes of the buffer.
        let mut buf = [0u8; 8];
        let (rest, decimal) = match endian {
            Endianness::Big => buf.split_at_mut(8 - len),
            Endianness::Little => {
                let (first, second) = buf.split_at_mut(len);
                (second, first)
            }
        };

        decimal.copy_from_slice(bytes.as_ref());

        // Perform sign-extension if necessary.
        if signed {
            let most_significant_byte = match endian {
                Endianness::Big => decimal[0],
                Endianness::Little => decimal[len - 1],
            };

            if most_significant_byte & 0b1000_0000 != 0 {
                rest.fill(0xFF);
            }
        }

        Ok(match endian {
            Endianness::Big => i64::from_be_bytes(buf),
            Endianness::Little => i64::from_le_bytes(buf),
        })
    }

    /// 整数をバイト列に変換します。
    ///
    /// ```example
    /// #array(10000.to-bytes(endian: "big")) \
    /// #array(10000.to-bytes(size: 4))
    /// ```
    #[func]
    pub fn to_bytes(
        self,
        /// 変換のエンディアン。
        #[named]
        #[default(Endianness::Little)]
        endian: Endianness,
        /// 結果のバイト列のサイズ（バイト単位、ゼロ以上でなければなりません）。
        /// 整数が指定サイズに収まらないほど大きい場合、変換はエンディアンに基づいて
        /// 余分なバイトを切り捨てます。同じ結果値を保つため、エンディアンが
        /// ビッグエンディアンの場合は最右のバイトで切り捨てが起こります。
        /// 一方、エンディアンがリトルエンディアンの場合は最左のバイトで切り捨てが起こります。
        ///
        /// 整数が負で、数値を収めるのにサイズが不十分な場合、結果のバイト列を
        /// `int.from-bytes`に渡したときに、最上位ビットが1に設定されない可能性があるため、
        /// 結果の数値が正になることがある点に注意してください。
        #[named]
        #[default(8)]
        size: usize,
    ) -> Bytes {
        let array = match endian {
            Endianness::Big => self.to_be_bytes(),
            Endianness::Little => self.to_le_bytes(),
        };

        let mut buf = SmallVec::<[u8; 8]>::from_elem(0, size);
        match endian {
            Endianness::Big => {
                // Copy the bytes from the array to the buffer, starting from
                // the end of the buffer.
                let buf_start = size.saturating_sub(8);
                let array_start = 8usize.saturating_sub(size);
                buf[buf_start..].copy_from_slice(&array[array_start..])
            }
            Endianness::Little => {
                // Copy the bytes from the array to the buffer, starting from
                // the beginning of the buffer.
                let end = size.min(8);
                buf[..end].copy_from_slice(&array[..end])
            }
        }

        Bytes::new(buf)
    }
}

impl Repr for i64 {
    fn repr(&self) -> EcoString {
        eco_format!("{:?}", self)
    }
}

/// 整数や浮動小数点数をバイト列に変換する際、またはその逆の変換に用いるバイト順を表します。
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Endianness {
    /// ビッグエンディアンのバイト順。最上位のバイトがバイト列の先頭に配置されます。
    Big,
    /// リトルエンディアンのバイト順。最下位のバイトがバイト列の先頭に配置されます。
    Little,
}

/// 整数にキャスト可能な値。
pub struct ToInt(i64);

cast! {
    ToInt,
    v: i64 => Self(v),
    v: bool => Self(v as i64),
    v: f64 => Self(convert_float_to_int(v)?),
    v: Decimal => Self(i64::try_from(v).map_err(|_| eco_format!("number too large"))?),
    v: Str => Self(parse_int(&v).map_err(|_| eco_format!("invalid integer: {}", v))?),
}

pub fn convert_float_to_int(f: f64) -> StrResult<i64> {
    if f <= i64::MIN as f64 - 1.0 || f >= i64::MAX as f64 + 1.0 {
        Err(eco_format!("number too large"))
    } else {
        Ok(f as i64)
    }
}

fn parse_int(mut s: &str) -> Result<i64, ParseIntError> {
    let mut sign = 1;
    if let Some(rest) = s.strip_prefix('-').or_else(|| s.strip_prefix(repr::MINUS_SIGN)) {
        sign = -1;
        s = rest;
    }
    if sign == -1 && s == "9223372036854775808" {
        return Ok(i64::MIN);
    }
    Ok(sign * s.parse::<i64>()?)
}

macro_rules! signed_int {
    ($($ty:ty)*) => {
        $(cast! {
            $ty,
            self => {
                #[allow(irrefutable_let_patterns)]
                if let Ok(int) = i64::try_from(self) {
                    Value::Int(int)
                } else {
                    // Some numbers (i128) are too large to be cast as i64
                    // In that case, we accept that there may be a
                    // precision loss, and use a floating point number
                    Value::Float(self as _)
                }
            },
            v: i64 => v.try_into().map_err(|_| "number too large")?,
        })*
    }
}

macro_rules! unsigned_int {
    ($($ty:ty)*) => {
        $(cast! {
            $ty,
            self => {
                #[allow(irrefutable_let_patterns)]
                if let Ok(int) = i64::try_from(self) {
                    Value::Int(int)
                } else {
                    // Some numbers (u64, u128) are too large to be cast as i64
                    // In that case, we accept that there may be a
                    // precision loss, and use a floating point number
                    Value::Float(self as _)
                }
            },
            v: i64 => v.try_into().map_err(|_| {
                if v < 0 {
                    "number must be at least zero"
                } else {
                    "number too large"
                }
            })?,
        })*
    }
}

signed_int! { i8 i16 i32 i128 isize }
unsigned_int! { u8 u16 u32 u64 u128 usize }

cast! {
    NonZeroI64,
    self => Value::Int(self.get() as _),
    v: i64 => v.try_into()
        .map_err(|_| if v == 0 {
            "number must not be zero"
        } else {
            "number too large"
        })?,
}

cast! {
    NonZeroIsize,
    self => Value::Int(self.get() as _),
    v: i64 => v
        .try_into()
        .and_then(|v: isize| v.try_into())
        .map_err(|_| if v == 0 {
            "number must not be zero"
        } else {
            "number too large"
        })?,
}

cast! {
    NonZeroU64,
    self => Value::Int(self.get() as _),
    v: i64 => v
        .try_into()
        .and_then(|v: u64| v.try_into())
        .map_err(|_| if v <= 0 {
            "number must be positive"
        } else {
            "number too large"
        })?,
}

cast! {
    NonZeroUsize,
    self => Value::Int(self.get() as _),
    v: i64 => v
        .try_into()
        .and_then(|v: usize| v.try_into())
        .map_err(|_| if v <= 0 {
            "number must be positive"
        } else {
            "number too large"
        })?,
}

cast! {
    NonZeroU32,
    self => Value::Int(self.get() as _),
    v: i64 => v
        .try_into()
        .and_then(|v: u32| v.try_into())
        .map_err(|_| if v <= 0 {
            "number must be positive"
        } else {
            "number too large"
        })?,
}
