use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Default meta type
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultMeta {
    pub request_id: String,
    pub links: Option<Links>,
    pub custom: HashMap<String, String>,
}

/// Struct to represent links related to the response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Links {
    pub self_link: String,
    pub next: Option<String>,
    pub prev: Option<String>,
}
