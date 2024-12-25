use std::collections::HashMap;

use chrono::{DateTime, Utc};
use getset2::Getset2;
use serde::{Deserialize, Serialize};

use crate::{MaybeString, utils::OrderedHashMap};

/// Default meta type
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DefaultMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    custom: OrderedHashMap<String, String>,
}

/// The user's permission information and so on.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Default, Getset2, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[getset2(get_ref, set_with)]
#[non_exhaustive]
pub struct UserMeta {
    pub id: String,
    pub roles: Vec<String>,
}

/// Pagination information.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Default, Getset2, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[getset2(get_ref(pub), set_with(pub))]
#[non_exhaustive]
pub struct Pagination {
    pub current_page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub total_records: u32,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
}

/// Rate limiting information.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Default, Getset2, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[getset2(set_with)]
#[non_exhaustive]
pub struct RateLimit {
    pub limit: i32,
    pub remaining: i32,
    pub restore_rate: i32,
    pub reset_at: Option<DateTime<Utc>>,
}

/// Cost and cost statistics.
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Default, Getset2, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[getset2(set_with)]
#[non_exhaustive]
pub struct Cost {
    pub actual_cost: u32,
    pub requested_query_cost: u32,
    pub execution_time: Option<DateTime<Utc>>,
}

impl Default for DefaultMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultMeta {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            request_id: None,
            user: None,
            pagination: None,
            rate_limit: None,
            cost: None,
            api_version: None,
            custom: OrderedHashMap::default(),
        }
    }
    #[inline(always)]
    pub fn with_request_id(mut self, request_id: impl Into<MaybeString>) -> Self {
        self.request_id = request_id.into().option_string();
        self
    }
    #[inline(always)]
    pub fn with_user(mut self, user: Option<UserMeta>) -> Self {
        self.user = user;
        self
    }
    #[inline(always)]
    pub const fn with_pagination(mut self, pagination: Option<Pagination>) -> Self {
        self.pagination = pagination;
        self
    }
    #[inline(always)]
    pub const fn with_rate_limit(mut self, rate_limit: Option<RateLimit>) -> Self {
        self.rate_limit = rate_limit;
        self
    }
    #[inline(always)]
    pub const fn with_cost(mut self, cost: Option<Cost>) -> Self {
        self.cost = cost;
        self
    }
    #[inline(always)]
    pub fn with_api_version(mut self, api_version: impl Into<MaybeString>) -> Self {
        self.api_version = api_version.into().option_string();
        self
    }
    #[inline(always)]
    pub fn insert_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
    #[inline]
    pub const fn request_id(&self) -> Option<&String> {
        self.request_id.as_ref()
    }
    pub const fn user(&self) -> Option<&UserMeta> {
        self.user.as_ref()
    }
    pub const fn pagination(&self) -> Option<&Pagination> {
        self.pagination.as_ref()
    }
    pub const fn rate_limit(&self) -> Option<&RateLimit> {
        self.rate_limit.as_ref()
    }
    pub const fn cost(&self) -> Option<&Cost> {
        self.cost.as_ref()
    }
    pub const fn api_version(&self) -> Option<&String> {
        self.api_version.as_ref()
    }
    pub fn custom(&self) -> &HashMap<String, String> {
        &self.custom
    }
    pub fn custom_kv(&self, key: impl AsRef<str>) -> Option<&String> {
        self.custom.get(key.as_ref())
    }
}
