//! # API Response Library
//!
//! This library provides a consistent structure for API responses, including success and error handling, with support for various serialization
//! formats like JSON and Protobuf.
//!
//! ## Modules
//!
//! * `meta`: Contains the meta structures.
//! * `success`: Contains the success response structures.
//! * `error`: Contains the error handling structures.

pub mod error;
pub mod meta;
pub mod success;

pub use error::{ErrorInfo, ErrorResponse};
pub use meta::{DefaultMeta, Links};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use success::SuccessResponse;

/// Enum to represent the overall API response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ApiResponse<Data, Meta> {
    Success(SuccessResponse<Data, Meta>),
    Error(ErrorResponse<Meta>),
}

impl<Data, Meta> From<SuccessResponse<Data, Meta>> for ApiResponse<Data, Meta> {
    fn from(value: SuccessResponse<Data, Meta>) -> Self {
        Self::Success(value)
    }
}

impl<Data, Meta> From<ErrorResponse<Meta>> for ApiResponse<Data, Meta> {
    fn from(value: ErrorResponse<Meta>) -> Self {
        Self::Error(value)
    }
}

pub type ApiResult<Data, Meta> = Result<SuccessResponse<Data, Meta>, ErrorResponse<Meta>>;

impl<Data, Meta> From<ApiResult<Data, Meta>> for ApiResponse<Data, Meta>
where
    Meta: Default,
{
    fn from(result: ApiResult<Data, Meta>) -> Self {
        match result {
            Ok(success) => ApiResponse::Success(success),
            Err(error) => ApiResponse::Error(error),
        }
    }
}

impl<Data, Meta> From<ApiResponse<Data, Meta>> for ApiResult<Data, Meta> {
    fn from(api_response: ApiResponse<Data, Meta>) -> Self {
        match api_response {
            ApiResponse::Success(success) => Ok(success),
            ApiResponse::Error(error) => Err(error),
        }
    }
}

impl<Data, Meta> From<Result<Data, ErrorInfo>> for ApiResponse<Data, Meta>
where
    Meta: Default,
{
    fn from(result: Result<Data, ErrorInfo>) -> Self {
        match result {
            Ok(data) => ApiResponse::Success(SuccessResponse::new(data, Meta::default())),
            Err(error) => ApiResponse::Error(ErrorResponse::new(error, Meta::default())),
        }
    }
}

impl<Data, Meta> From<ApiResponse<Data, Meta>> for Result<Data, ErrorInfo> {
    fn from(api_response: ApiResponse<Data, Meta>) -> Self {
        match api_response {
            ApiResponse::Success(success) => Ok(success.data),
            ApiResponse::Error(error) => Err(error.error),
        }
    }
}

impl<Data, Meta> From<ErrorInfo> for ApiResponse<Data, Meta>
where
    Meta: Default,
{
    fn from(error: ErrorInfo) -> Self {
        ApiResponse::Error(ErrorResponse::new(error, Meta::default()))
    }
}

impl<Data, Meta> From<(Data, Meta)> for ApiResponse<Data, Meta> {
    fn from((data, meta): (Data, Meta)) -> Self {
        ApiResponse::Success(SuccessResponse::new(data, meta))
    }
}
