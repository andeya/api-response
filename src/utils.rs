use std::{
    collections::HashMap,
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
    pub fn into_option_string(self) -> Option<String> {
        match self {
            MaybeString::String(v) => Some(v),
            MaybeString::Str(v) => Some(v.to_owned()),
            MaybeString::OptionString(v) => v,
            MaybeString::OptionStr(v) => v.map(|v| v.to_owned()),
            MaybeString::UnitTuple => None,
        }
    }
}
impl From<MaybeString> for Option<String> {
    fn from(value: MaybeString) -> Self {
        value.into_option_string()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utils::OrderedHashMap;

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
}
