use ecow::EcoString;

use crate::foundations::{Repr, ty};

/// 2つの状態を持つ型。
///
/// ブール型は`{true}`と`{false}`の2つの値を持ちます。
/// 何かが有効であるか、または有効化されているかを示します。
///
/// # 例
/// ```example
/// #false \
/// #true \
/// #(1 < 2)
/// ```
#[ty(cast, title = "Boolean")]
type bool;

impl Repr for bool {
    fn repr(&self) -> EcoString {
        match self {
            true => "true".into(),
            false => "false".into(),
        }
    }
}
