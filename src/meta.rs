use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{utils::OrderedHashMap, MaybeString};

/// Default meta type
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DefaultMeta {
    request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    custom: OrderedHashMap<String, String>,
}

/// Struct to represent links related to the response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub self_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
}

impl DefaultMeta {
    #[inline(always)]
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
            links: None,
            custom: Default::default(),
        }
    }
    #[inline(always)]
    pub fn with_links(mut self, links: Links) -> Self {
        self.links = Some(links);
        self
    }
    #[inline(always)]
    pub fn with_links_info(
        mut self,
        self_link: impl Into<String>,
        next: impl Into<MaybeString>,
        prev: impl Into<MaybeString>,
    ) -> Self {
        self.links = Some(Links {
            self_link: self_link.into(),
            next: next.into().option_string(),
            prev: prev.into().option_string(),
        });
        self
    }
    #[inline(always)]
    pub fn insert_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
    #[inline]
    pub fn request_id(&self) -> &String {
        &self.request_id
    }
    pub fn links(&self) -> Option<&Links> {
        self.links.as_ref()
    }
    pub fn custom(&self) -> &HashMap<String, String> {
        &self.custom
    }
    pub fn custom_kv(&self, key: impl AsRef<str>) -> Option<&String> {
        self.custom.get(key.as_ref())
    }
}

impl Links {
    #[inline(always)]
    pub fn new(self_link: impl Into<String>) -> Self {
        Self {
            self_link: self_link.into(),
            next: None,
            prev: None,
        }
    }
    #[inline(always)]
    pub fn with_next(mut self, next: impl Into<String>) -> Self {
        self.next = Some(next.into());
        self
    }
    #[inline(always)]
    pub fn with_prev(mut self, prev: impl Into<String>) -> Self {
        self.prev = Some(prev.into());
        self
    }
}
