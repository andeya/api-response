//! # API Response Library
//!
//! This library provides a consistent structure for API responses, including success and error handling, with support
//! for various serialization formats like JSON and Protobuf.
//!
//! ## Modules
//!
//! * `meta`: Contains the meta structures.
//! * `success`: Contains the success response structures.
//! * `error`: Contains the error handling structures.

#![cfg_attr(feature = "try", feature(try_trait_v2))]
#[cfg(feature = "try")]
mod try_trait;

#[cfg(feature = "salvo")]
mod salvo_trait;

mod error;
pub mod error_code;
mod meta;
mod result;
mod success;
mod utils;

use std::{error::Error, fmt::Debug};

pub use prelude::*;
pub mod prelude {
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

    pub use crate::{
        api_err,
        error::{ApiError, ErrorResponse},
        error_code,
        meta::{DefaultMeta, Links},
        result::ApiResult,
        success::{ApiSuccessResponse, SuccessResponse},
        utils::{ErrWrapper, IntoError, MaybeString},
        ApiResponse,
    };
}

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

impl<Data, Meta> From<ApiError> for ApiResponse<Data, Meta> {
    fn from(error: ApiError) -> Self {
        ApiResponse::Error(ErrorResponse::from_error(error))
    }
}

impl<Data, Meta> ApiResponse<Data, Meta> {
    #[inline(always)]
    pub fn new_success(data: Data, meta: Meta) -> Self {
        Self::Success(SuccessResponse::new(data, meta))
    }
    #[inline(always)]
    pub fn from_success(data: Data) -> Self {
        Self::Success(SuccessResponse::from_data(data))
    }
    #[inline(always)]
    pub fn from_success_data(data: Data) -> Self {
        Self::Success(SuccessResponse::from_data(data))
    }
    #[inline(always)]
    pub fn new_error(error: ApiError, meta: Meta) -> Self {
        Self::Error(ErrorResponse::new(error, meta))
    }
    #[inline(always)]
    pub fn from_error(error: ApiError) -> Self {
        Self::Error(ErrorResponse::from_error(error))
    }
    #[inline(always)]
    pub fn from_error_msg(code: impl Into<i32>, message: impl Into<String>) -> Self {
        Self::Error(ErrorResponse::from_error_msg(code, message))
    }
    #[inline(always)]
    pub fn from_error_source(
        code: impl Into<i32>,
        source: impl Error + Send + Sync + 'static,
        set_source_detail: bool,
        message: Option<String>,
    ) -> Self {
        Self::Error(ErrorResponse::from_error_source(
            code,
            source,
            set_source_detail,
            message,
        ))
    }
    #[inline(always)]
    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.set_meta(meta);
        self
    }
    #[inline(always)]
    pub fn set_meta(&mut self, meta: Meta) -> &mut Self {
        match self {
            ApiResponse::Success(success_response) => {
                success_response.set_meta(meta);
            }
            ApiResponse::Error(error_response) => {
                error_response.set_meta(meta);
            }
        }
        self
    }
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
    pub fn expect(self, expect_msg: &str) -> SuccessResponse<Data, Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response,
            ApiResponse::Error(_) => {
                panic!("{expect_msg}")
            }
        }
    }
    pub fn unwrap(self) -> SuccessResponse<Data, Meta> {
        match self {
            ApiResponse::Success(success_response) => success_response,
            ApiResponse::Error(_) => {
                panic!("called `ApiResponse::unwrap()` on an `Error` value")
            }
        }
    }
    pub fn unwrap_err(self) -> ErrorResponse<Meta> {
        match self {
            ApiResponse::Success(_) => {
                panic!("called `ApiResponse::unwrap_err()` on an `Error` value")
            }
            ApiResponse::Error(error_response) => error_response,
        }
    }
    pub fn into_result(self) -> ApiResult<Data, Meta> {
        self.into()
    }
    pub fn into_result_data(self) -> Result<Data, ErrorResponse<Meta>> {
        match self {
            ApiResponse::Success(success_response) => Ok(success_response.data),
            ApiResponse::Error(error_response) => Err(error_response),
        }
    }
    pub fn into_result_without_meta(self) -> Result<Data, ApiError> {
        match self {
            ApiResponse::Success(success_response) => Ok(success_response.data),
            ApiResponse::Error(error_response) => Err(error_response.error),
        }
    }
}
