use serde::{Deserialize, Serialize};

/// Struct to represent a successful response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<Data, Meta> {
    pub data: Data,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<Data, Meta> SuccessResponse<Data, Meta> {
    #[inline(always)]
    pub fn new(data: Data) -> Self {
        SuccessResponse { data, meta: None }
    }
    #[inline(always)]
    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }
}
