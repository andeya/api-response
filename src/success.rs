use serde::{Deserialize, Serialize};

/// Struct to represent a successful response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<Data, Meta> {
    pub data: Data,
    pub meta: Meta,
}

impl<Data, Meta> SuccessResponse<Data, Meta> {
    pub fn new(data: Data, meta: Meta) -> Self {
        SuccessResponse { data, meta }
    }
}
