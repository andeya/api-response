use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Default)]
pub(crate) struct OrderedHashMap<K, V>(pub(crate) HashMap<K, V>);

impl<K, V> Serialize for OrderedHashMap<K, V>
where
    K: Serialize + Ord,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sorted_entries: Vec<_> = self.0.iter().collect();
        sorted_entries.sort_by_key(|&(k, _)| k);

        let mut map = serializer.serialize_map(Some(sorted_entries.len()))?;
        for (k, v) in sorted_entries {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, K, V> Deserialize<'de> for OrderedHashMap<K, V>
where
    K: Deserialize<'de> + Eq + std::hash::Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = HashMap::deserialize(deserializer)?;
        Ok(OrderedHashMap(map))
    }
}

impl<K, V> Deref for OrderedHashMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for OrderedHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum MaybeString {
    String(String),
    Str(&'static str),
    OptionString(Option<String>),
    OptionStr(Option<&'static str>),
    UnitTuple,
}
impl From<()> for MaybeString {
    fn from(_: ()) -> Self {
        MaybeString::UnitTuple
    }
}
impl From<String> for MaybeString {
    fn from(value: String) -> Self {
        MaybeString::String(value)
    }
}
impl From<&'static str> for MaybeString {
    fn from(value: &'static str) -> Self {
        MaybeString::Str(value)
    }
}
impl From<Option<String>> for MaybeString {
    fn from(value: Option<String>) -> Self {
        MaybeString::OptionString(value)
    }
}
impl From<Option<&'static str>> for MaybeString {
    fn from(value: Option<&'static str>) -> Self {
        MaybeString::OptionStr(value)
    }
}
impl MaybeString {
    pub fn option_string(self) -> Option<String> {
        match self {
            MaybeString::String(v) => Some(v),
            MaybeString::Str(v) => Some(v.to_owned()),
            MaybeString::OptionString(v) => v,
            MaybeString::OptionStr(v) => v.map(|v| v.to_owned()),
            MaybeString::UnitTuple => None,
        }
    }
    pub fn expect(self, msg: &str) -> String {
        match self {
            MaybeString::String(v) => v,
            MaybeString::Str(v) => v.to_owned(),
            MaybeString::OptionString(v) => v.expect(msg),
            MaybeString::OptionStr(v) => v.expect(msg).to_owned(),
            MaybeString::UnitTuple => panic!("{msg}"),
        }
    }
    pub fn unwrap_or(self, default: impl Into<String>) -> String {
        match self {
            MaybeString::String(v) => v,
            MaybeString::Str(v) => v.to_owned(),
            MaybeString::OptionString(v) => v.unwrap_or_else(|| default.into()),
            MaybeString::OptionStr(v) => v.map_or_else(|| default.into(), |v| v.to_owned()),
            MaybeString::UnitTuple => default.into(),
        }
    }
    pub fn unwrap_or_else(self, f: impl FnOnce() -> String) -> String {
        match self {
            MaybeString::String(v) => v,
            MaybeString::Str(v) => v.to_owned(),
            MaybeString::OptionString(v) => v.unwrap_or_else(f),
            MaybeString::OptionStr(v) => v.map_or_else(f, |v| v.to_owned()),
            MaybeString::UnitTuple => f(),
        }
    }
    pub fn unwrap_or_default(self) -> String {
        match self {
            MaybeString::String(v) => v,
            MaybeString::Str(v) => v.to_owned(),
            MaybeString::OptionString(v) => v.unwrap_or_default(),
            MaybeString::OptionStr(v) => v.map_or_else(Default::default, |v| v.to_owned()),
            MaybeString::UnitTuple => Default::default(),
        }
    }
}
impl From<MaybeString> for Option<String> {
    fn from(value: MaybeString) -> Self {
        value.option_string()
    }
}

#[derive(Debug)]
pub struct ErrWrapper<E: Display>(pub E);

impl<E: Debug + Display> Display for ErrWrapper<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<E: Debug + Display> Error for ErrWrapper<E> {}

pub trait IntoError: Debug + Display + Sized {
    fn into_error(self) -> ErrWrapper<Self> {
        ErrWrapper(self)
    }
}

impl<E: Debug + Display + Sized> IntoError for E {}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error};

    use super::{IntoError, OrderedHashMap};

    #[test]
    fn ordered_hash_map() {
        const S: &str = r#"{"k1":"v1","k2":"v2"}"#;
        let mut map = HashMap::new();
        map.insert("k2", "v2");
        map.insert("k1", "v1");
        let map1 = OrderedHashMap(map);
        assert_eq!(S, serde_json::to_string(&map1).unwrap());
        let map2: OrderedHashMap<&str, &str> = serde_json::from_str(S).unwrap();
        assert_eq!(S, serde_json::to_string(&map2).unwrap());
    }

    #[test]
    fn error_wrapper() {
        let e: super::ErrWrapper<i32> = 1.into_error();
        let d: &dyn Error = &e;
        assert_eq!("1", d.to_string())
    }
}
