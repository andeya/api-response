use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Default meta type
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DefaultMeta {
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub custom: HashMap<String, String>,
}

/// Struct to represent links related to the response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
            custom: HashMap::new(),
        }
    }
    #[inline(always)]
    pub fn with_links(mut self, links: Links) -> Self {
        self.links = Some(links);
        self
    }
    #[inline(always)]
    pub fn insert_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
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
