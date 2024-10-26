use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Struct to represent links related to the response
#[derive(Serialize, Deserialize)]
pub struct Links {
    pub self_link: String,
    pub next: Option<String>,
    pub prev: Option<String>,
}

/// Default metadata type
#[derive(Serialize, Deserialize, Default)]
pub struct DefaultMeta {
    pub request_id: String,
    pub links: Option<Links>,
    #[serde(flatten)]
    pub custom: HashMap<String, String>,
}
