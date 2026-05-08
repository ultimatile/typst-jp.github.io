use std::any::{Any, TypeId};
use std::sync::Arc;

use comemo::Tracked;
use ecow::{EcoString, EcoVec, eco_format};
use smallvec::SmallVec;

use crate::diag::{HintedStrResult, StrResult, bail};
use crate::foundations::{
    CastInfo, Content, Context, Dict, Element, FromValue, Func, Label, Reflect, Regex,
    Repr, Str, StyleChain, Symbol, Type, Value, cast, func, repr, scope, ty,
};
use crate::introspection::{Introspector, Locatable, Location, Unqueriable};

/// A helper macro to create a field selector used in [`Selector::Elem`]
#[macro_export]
#[doc(hidden)]
macro_rules! __select_where {
    ($ty:ty $(, $field:ident => $value:expr)* $(,)?) => {{
        #[allow(unused_mut)]
        let mut fields = $crate::foundations::SmallVec::new();
        $(
            fields.push((
                <$ty>::$field.index(),
                $crate::foundations::IntoValue::into_value($value),
            ));
        )*
        $crate::foundations::Selector::Elem(
            <$ty as $crate::foundations::NativeElement>::ELEM,
            Some(fields),
        )
    }};
}

#[doc(inline)]
pub use crate::__select_where as select_where;

/// 文書内の要素を選択するためのフィルター。
///
/// セレクターを構築する方法は以下の通りです。
/// - 要素[関数]($function)を使う
/// - [特定のフィールド]($function.where)で要素関数を絞り込む
/// - [文字列]($str)や[正規表現]($regex)を使う
/// - [`{<label>}`]($label)を使う
/// - [`location`]を使う
/// - [`selector`]コンストラクターを呼び出して、上記いずれかの型を
///   セレクター値に変換し、以下のメソッドで絞り込む
///
/// セレクターは要素に[スタイル設定ルールを適用]($styling/#show-rules)するために使われます。
/// セレクターを使って、特定の種類の要素を文書から[クエリ]($query)することもできます。
///
/// さらに、Typstの組み込み関数のいくつかにセレクターを渡して、その動作を構成できます。
/// その一例が[outline]で、目次に挙げる要素を変更するためにセレクターを使えます。
///
/// 複数のセレクターは以下に示すメソッドで組み合わせられます。
/// ただし、現時点では全ての種類のセレクターが全ての箇所でサポートされているわけではありません。
///
/// # 例
/// ```example
/// #context query(
///   heading.where(level: 1)
///     .or(heading.where(level: 2))
/// )
///
/// = This will be found
/// == So will this
/// === But this will not.
/// ```
#[ty(scope, cast)]
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Selector {
    /// Matches a specific type of element.
    ///
    /// If there is a dictionary, only elements with the fields from the
    /// dictionary match.
    Elem(Element, Option<SmallVec<[(u8, Value); 1]>>),
    /// Matches the element at the specified location.
    Location(Location),
    /// Matches elements with a specific label.
    Label(Label),
    /// Matches text elements through a regular expression.
    Regex(Regex),
    /// Matches elements with a specific capability.
    Can(TypeId),
    /// Matches if any of the subselectors match.
    Or(EcoVec<Self>),
    /// Matches if all of the subselectors match.
    And(EcoVec<Self>),
    /// Matches all matches of `selector` before `end`.
    Before { selector: Arc<Self>, end: Arc<Self>, inclusive: bool },
    /// Matches all matches of `selector` after `start`.
    After { selector: Arc<Self>, start: Arc<Self>, inclusive: bool },
}

impl Selector {
    /// Define a simple text selector.
    pub fn text(text: &str) -> StrResult<Self> {
        if text.is_empty() {
            bail!("text selector is empty");
        }
        Ok(Self::Regex(Regex::new(&regex::escape(text)).unwrap()))
    }

    /// Define a regex selector.
    pub fn regex(regex: Regex) -> StrResult<Self> {
        if regex.as_str().is_empty() {
            bail!("regex selector is empty");
        }
        if regex.is_match("") {
            bail!("regex matches empty text");
        }
        Ok(Self::Regex(regex))
    }

    /// Define a simple [`Selector::Can`] selector.
    pub fn can<T: ?Sized + Any>() -> Self {
        Self::Can(TypeId::of::<T>())
    }

    /// Whether the selector matches for the target.
    pub fn matches(&self, target: &Content, styles: Option<StyleChain>) -> bool {
        match self {
            Self::Elem(element, dict) => {
                target.elem() == *element
                    && dict.iter().flat_map(|dict| dict.iter()).all(|(id, value)| {
                        target.get(*id, styles).as_ref().ok() == Some(value)
                    })
            }
            Self::Label(label) => target.label() == Some(*label),
            Self::Can(cap) => target.func().can_type_id(*cap),
            Self::Or(selectors) => {
                selectors.iter().any(move |sel| sel.matches(target, styles))
            }
            Self::And(selectors) => {
                selectors.iter().all(move |sel| sel.matches(target, styles))
            }
            Self::Location(location) => target.location() == Some(*location),
            // Not supported here.
            Self::Regex(_) | Self::Before { .. } | Self::After { .. } => false,
        }
    }
}

#[scope]
impl Selector {
    /// 値をセレクターに変換します。次の値が受け付けられます。
    /// - `heading`や`figure`のような要素関数。
    /// - [文字列]($str)または[正規表現]($regex)。
    /// - `{<label>}`。
    /// - [`location`]。
    /// - `{heading.where(level: 1)}`のようなより複雑なセレクター。
    #[func(constructor)]
    pub fn construct(
        /// `heading`や`figure`のような要素関数、`{<label>}`、または
        /// `{heading.where(level: 1)}`のようなより複雑なセレクター。
        target: Selector,
    ) -> Selector {
        target
    }

    /// このセレクターまたは他のセレクターのいずれかに一致する全ての要素を選択します。
    #[func]
    pub fn or(
        self,
        /// 一致対象とする他のセレクター。
        #[variadic]
        others: Vec<Selector>,
    ) -> Selector {
        Self::Or(others.into_iter().chain(Some(self)).collect())
    }

    /// このセレクターおよび他の全てのセレクターに一致する要素を選択します。
    #[func]
    pub fn and(
        self,
        /// 一致対象とする他のセレクター。
        #[variadic]
        others: Vec<Selector>,
    ) -> Selector {
        Self::And(others.into_iter().chain(Some(self)).collect())
    }

    /// `end`の最初の一致より前に現れる要素にのみ一致するように修正したセレクターを返します。
    #[func]
    pub fn before(
        self,
        /// 元の選択は`end`の最初の一致で終了します。
        end: LocatableSelector,
        /// `end`自体を一致対象に含めるかどうか。
        /// これは両方のセレクターが同じ型の要素に一致する場合にのみ意味があります。
        /// デフォルトは`{true}`です。
        #[named]
        #[default(true)]
        inclusive: bool,
    ) -> Selector {
        Self::Before {
            selector: Arc::new(self),
            end: Arc::new(end.0),
            inclusive,
        }
    }

    /// `start`の最初の一致より後に現れる要素にのみ一致するように修正したセレクターを返します。
    #[func]
    pub fn after(
        self,
        /// 元の選択は`start`の最初の一致から開始します。
        start: LocatableSelector,
        /// `start`自体を一致対象に含めるかどうか。
        /// これは両方のセレクターが同じ型の要素に一致する場合にのみ意味があります。
        /// デフォルトは`{true}`です。
        #[named]
        #[default(true)]
        inclusive: bool,
    ) -> Selector {
        Self::After {
            selector: Arc::new(self),
            start: Arc::new(start.0),
            inclusive,
        }
    }
}

impl From<Location> for Selector {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl Repr for Selector {
    fn repr(&self) -> EcoString {
        match self {
            Self::Elem(elem, dict) => {
                if let Some(dict) = dict {
                    let dict = dict
                        .iter()
                        .map(|(id, value)| (elem.field_name(*id).unwrap(), value.clone()))
                        .map(|(name, value)| (EcoString::from(name).into(), value))
                        .collect::<Dict>();
                    eco_format!("{}.where{}", elem.name(), dict.repr())
                } else {
                    elem.name().into()
                }
            }
            Self::Label(label) => label.repr(),
            Self::Regex(regex) => regex.repr(),
            Self::Can(cap) => eco_format!("{cap:?}"),
            Self::Or(selectors) | Self::And(selectors) => {
                let function = if matches!(self, Self::Or(_)) { "or" } else { "and" };
                let pieces: Vec<_> = selectors.iter().map(Selector::repr).collect();
                eco_format!("{}{}", function, repr::pretty_array_like(&pieces, false))
            }
            Self::Location(loc) => loc.repr(),
            Self::Before { selector, end: split, inclusive }
            | Self::After { selector, start: split, inclusive } => {
                let method =
                    if matches!(self, Self::Before { .. }) { "before" } else { "after" };
                let inclusive_arg = if !*inclusive { ", inclusive: false" } else { "" };
                eco_format!(
                    "{}.{}({}{})",
                    selector.repr(),
                    method,
                    split.repr(),
                    inclusive_arg
                )
            }
        }
    }
}

cast! {
    type Selector,
    text: EcoString => Self::text(&text)?,
    func: Func => func
        .element()
        .ok_or("only element functions can be used as selectors")?
        .select(),
    label: Label => Self::Label(label),
    regex: Regex => Self::regex(regex)?,
    location: Location => Self::Location(location),
}

/// A selector that can be used with `query`.
///
/// Hopefully, this is made obsolete by a more powerful query mechanism in the
/// future.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LocatableSelector(pub Selector);

impl LocatableSelector {
    /// Resolve this selector into a location that is guaranteed to be unique.
    pub fn resolve_unique(
        &self,
        introspector: Tracked<Introspector>,
        context: Tracked<Context>,
    ) -> HintedStrResult<Location> {
        match &self.0 {
            Selector::Location(loc) => Ok(*loc),
            other => {
                context.introspect()?;
                Ok(introspector.query_unique(other).map(|c| c.location().unwrap())?)
            }
        }
    }
}

impl Reflect for LocatableSelector {
    fn input() -> CastInfo {
        CastInfo::Union(vec![
            CastInfo::Type(Type::of::<Label>()),
            CastInfo::Type(Type::of::<Func>()),
            CastInfo::Type(Type::of::<Location>()),
            CastInfo::Type(Type::of::<Selector>()),
        ])
    }

    fn output() -> CastInfo {
        CastInfo::Type(Type::of::<Selector>())
    }

    fn castable(value: &Value) -> bool {
        Label::castable(value)
            || Func::castable(value)
            || Location::castable(value)
            || Selector::castable(value)
    }
}

cast! {
    LocatableSelector,
    self => self.0.into_value(),
}

impl FromValue for LocatableSelector {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        fn validate(selector: &Selector) -> StrResult<()> {
            match selector {
                Selector::Elem(elem, _) => {
                    if !elem.can::<dyn Locatable>() || elem.can::<dyn Unqueriable>() {
                        Err(eco_format!("{} is not locatable", elem.name()))?
                    }
                }
                Selector::Location(_) => {}
                Selector::Label(_) => {}
                Selector::Regex(_) => bail!("text is not locatable"),
                Selector::Can(_) => bail!("capability is not locatable"),
                Selector::Or(list) | Selector::And(list) => {
                    for selector in list {
                        validate(selector)?;
                    }
                }
                Selector::Before { selector, end: split, .. }
                | Selector::After { selector, start: split, .. } => {
                    for selector in [selector, split] {
                        validate(selector)?;
                    }
                }
            }
            Ok(())
        }

        if !Self::castable(&value) {
            return Err(Self::error(&value));
        }

        let selector = Selector::from_value(value)?;
        validate(&selector)?;
        Ok(Self(selector))
    }
}

impl From<Location> for LocatableSelector {
    fn from(loc: Location) -> Self {
        Self(Selector::Location(loc))
    }
}

/// A selector that can be used with show rules.
///
/// Hopefully, this is made obsolete by a more powerful showing mechanism in the
/// future.
#[derive(Clone, PartialEq, Hash)]
pub struct ShowableSelector(pub Selector);

impl Reflect for ShowableSelector {
    fn input() -> CastInfo {
        CastInfo::Union(vec![
            CastInfo::Type(Type::of::<Symbol>()),
            CastInfo::Type(Type::of::<Str>()),
            CastInfo::Type(Type::of::<Label>()),
            CastInfo::Type(Type::of::<Func>()),
            CastInfo::Type(Type::of::<Regex>()),
            CastInfo::Type(Type::of::<Selector>()),
        ])
    }

    fn output() -> CastInfo {
        CastInfo::Type(Type::of::<Selector>())
    }

    fn castable(value: &Value) -> bool {
        Symbol::castable(value)
            || Str::castable(value)
            || Label::castable(value)
            || Func::castable(value)
            || Regex::castable(value)
            || Selector::castable(value)
    }
}

cast! {
    ShowableSelector,
    self => self.0.into_value(),
}

impl FromValue for ShowableSelector {
    fn from_value(value: Value) -> HintedStrResult<Self> {
        fn validate(selector: &Selector, nested: bool) -> HintedStrResult<()> {
            match selector {
                Selector::Elem(_, _) => {}
                Selector::Label(_) => {}
                Selector::Regex(_) if !nested => {}
                Selector::Or(list) | Selector::And(list) => {
                    for selector in list {
                        validate(selector, true)?;
                    }
                }
                Selector::Regex(_)
                | Selector::Location(_)
                | Selector::Can(_)
                | Selector::Before { .. }
                | Selector::After { .. } => {
                    bail!("this selector cannot be used with show")
                }
            }
            Ok(())
        }

        if !Self::castable(&value) {
            return Err(Self::error(&value));
        }

        let selector = Selector::from_value(value)?;
        validate(&selector, false)?;
        Ok(Self(selector))
    }
}
