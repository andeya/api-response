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

impl<Data, Meta> ApiResponse<Data, Meta> {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
    pub fn get_meta(&self) -> Option<&Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response.meta.as_ref(),
            ApiResponse::Error(error_response) => error_response.meta.as_ref(),
        }
    }
    pub fn into_result(self) -> ApiResult<Data, Meta> {
        self.into()
    }
    pub fn into_data(self) -> Result<Data, ErrorResponse<Meta>> {
        match self {
            ApiResponse::Success(success_response) => Ok(success_response.data),
            ApiResponse::Error(error_response) => Err(error_response),
        }
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

impl<Data, Meta> From<Result<Data, ErrorInfo>> for ApiResponse<Data, Meta> {
    fn from(result: Result<Data, ErrorInfo>) -> Self {
        match result {
            Ok(data) => ApiResponse::Success(SuccessResponse::new(data)),
            Err(error) => ApiResponse::Error(ErrorResponse::new(error)),
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

impl<Data, Meta> From<ErrorInfo> for ApiResponse<Data, Meta> {
    fn from(error: ErrorInfo) -> Self {
        ApiResponse::Error(ErrorResponse::new(error))
    }
}

impl<Data, Meta> From<(Data, Meta)> for ApiResponse<Data, Meta> {
    fn from((data, meta): (Data, Meta)) -> Self {
        ApiResponse::Success(SuccessResponse::new(data).with_meta(meta))
    }
}
