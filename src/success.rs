use serde::{Deserialize, Serialize};

use crate::ApiResponse;

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
    pub fn new(data: Data, meta: Option<Meta>) -> Self {
        SuccessResponse { data, meta }
    }
    #[inline(always)]
    pub fn from_data(data: Data) -> Self {
        SuccessResponse { data, meta: None }
    }
    #[inline(always)]
    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.set_meta(meta);
        self
    }
    #[inline(always)]
    pub fn set_meta(&mut self, meta: Meta) -> &mut Self {
        self.meta = Some(meta);
        self
    }
}

pub trait ApiSuccessResponse: Sized {
    fn api_success_response<Meta>(self, meta: Option<Meta>) -> ApiResponse<Self, Meta> {
        ApiResponse::Success(SuccessResponse { data: self, meta })
    }
    #[inline(always)]
    fn api_success_without_meta<Meta>(self) -> ApiResponse<Self, Meta> {
        self.api_success_response(None)
    }
    #[inline(always)]
    fn api_success_with_meta<Meta>(self, meta: Meta) -> ApiResponse<Self, Meta> {
        self.api_success_response(Some(meta))
    }
}

impl<Data> ApiSuccessResponse for Data {}
