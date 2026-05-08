use std::fmt::{self, Debug, Formatter};

use ecow::EcoString;
use serde::{Serialize, Serializer};

use crate::diag::HintedStrResult;
use crate::foundations::{
    CastInfo, FromValue, IntoValue, Reflect, Repr, Type, Value, cast, ty,
};

/// 他の値が存在しないことを示す値。
///
/// none型は`{none}`という1つの値だけを持ちます。
///
/// 文書に挿入されても表示されません。これは空のコードブロックが生成する値でもあります。
/// 任意の値と[結合]($scripting/#blocks)でき、結合結果は他方の値となります。
///
/// # 例
/// ```example
/// Not visible: #none
/// ```
#[ty(cast, name = "none")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NoneValue;

impl Reflect for NoneValue {
    fn input() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn output() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn castable(value: &Value) -> bool {
        matches!(value, Value::None)
    }
}

impl IntoValue for NoneValue {
    fn into_value(self) -> Value {
        Value::None
    }
}

impl FromValue for NoneValue {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        match value {
            Value::None => Ok(Self),
            _ => Err(Self::error(&value)),
        }
    }
}

impl Debug for NoneValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("None")
    }
}

impl Repr for NoneValue {
    fn repr(&self) -> EcoString {
        "none".into()
    }
}

impl Serialize for NoneValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

cast! {
    (),
    self => Value::None,
    _: NoneValue => (),
}

impl<T: Reflect> Reflect for Option<T> {
    fn input() -> CastInfo {
        T::input() + NoneValue::input()
    }

    fn output() -> CastInfo {
        T::output() + NoneValue::output()
    }

    fn castable(value: &Value) -> bool {
        NoneValue::castable(value) || T::castable(value)
    }
}

impl<T: IntoValue> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => Value::None,
        }
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        match value {
            Value::None => Ok(None),
            v if T::castable(&v) => Ok(Some(T::from_value(v)?)),
            _ => Err(Self::error(&value)),
        }
    }
}
