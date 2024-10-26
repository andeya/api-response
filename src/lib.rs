//! # API Response Library
//!
//! This library provides a consistent structure for API responses, including success and error handling, with support for various serialization
//! formats like JSON and Protobuf.
//!
//! ## Modules
//!
//! * `error`: Contains the error handling structures.
//! * `success`: Contains the success response structures.
//! * `meta`: Contains the metadata structures.

pub mod error;
pub mod meta;
pub mod success;

pub use error::{ErrorInfo, ErrorResponse};
pub use meta::{DefaultMeta, Links};
use serde::{Deserialize, Serialize};
pub use success::SuccessResponse;

/// Enum to represent the overall API response
#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ApiResponse<T, M, Details> {
    Success(SuccessResponse<T, M>),
    Error(ErrorResponse<M, Details>),
}

impl<T, M, Details> From<Result<T, ErrorInfo<Details>>> for ApiResponse<T, M, Details>
where
    M: Default + Serialize + for<'de> Deserialize<'de>,
{
    fn from(result: Result<T, ErrorInfo<Details>>) -> Self {
        match result {
            Ok(data) => ApiResponse::Success(SuccessResponse::new(M::default(), data)),
            Err(error) => ApiResponse::Error(ErrorResponse::new(M::default(), error)),
        }
    }
}

impl<T, M, Details> From<ApiResponse<T, M, Details>> for Result<T, ErrorInfo<Details>> {
    fn from(api_response: ApiResponse<T, M, Details>) -> Self {
        match api_response {
            ApiResponse::Success(res) => Ok(res.data),
            ApiResponse::Error(res) => Err(res.error),
        }
    }
}
