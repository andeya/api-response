use serde::{Deserialize, Serialize};

/// Struct to represent a successful response
#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T, M> {
    pub metadata: M,
    pub data: T,
}

impl<T, M> SuccessResponse<T, M> {
    pub fn new(metadata: M, data: T) -> Self {
        SuccessResponse { metadata, data }
    }
}
