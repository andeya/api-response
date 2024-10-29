use std::{self, collections::HashMap, error::Error, fmt, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::ApiResponse;

/// Struct to represent an error response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<Meta> {
    pub error: ApiError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<Meta> ErrorResponse<Meta> {
    #[inline(always)]
    pub fn new(error: ApiError, meta: Option<Meta>) -> Self {
        ErrorResponse { error, meta }
    }
    #[inline(always)]
    pub fn from_error(error: ApiError) -> Self {
        ErrorResponse { error, meta: None }
    }
    #[inline(always)]
    pub fn from_error_msg(code: i32, message: impl Into<String>) -> Self {
        Self::from_error(ApiError::new(code, message))
    }
    #[inline(always)]
    pub fn from_error_source(code: i32, source: impl Error + Send + Sync + 'static, message: Option<String>) -> Self {
        Self::from_error(ApiError::from_source(code, source, message))
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
    #[inline(always)]
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set_detail(key, value);
        self
    }
    #[inline(always)]
    pub fn set_detail(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.error.set_detail(key, value);
        self
    }
    #[inline(always)]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.set_source(source);
        self
    }
    #[inline(always)]
    pub fn set_source(&mut self, source: impl Error + Send + Sync + 'static) -> &mut Self {
        self.error.set_source(source);
        self
    }
    #[inline(always)]
    pub fn is<E: Error + 'static>(&self) -> bool {
        self.error.is::<E>()
    }
    #[inline(always)]
    pub fn downcast_ref<E: Error + 'static>(&self) -> Option<&E> {
        self.error.downcast_ref::<E>()
    }
}
impl<Meta> From<ApiError> for ErrorResponse<Meta> {
    fn from(value: ApiError) -> Self {
        Self::from_error(value)
    }
}
impl<Meta> fmt::Display for ErrorResponse<Meta> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error.message)
    }
}
impl<Meta: fmt::Debug> Error for ErrorResponse<Meta> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

/// Struct to represent error information
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
    #[serde(skip)]
    pub source: Option<Arc<dyn Error + Send + Sync + 'static>>,
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("details", &self.details)
            .finish()
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

impl ApiError {
    #[inline(always)]
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        ApiError {
            code,
            message: message.into(),
            details: None,
            source: None,
        }
    }
    #[inline(always)]
    pub fn from_source(code: i32, source: impl Error + Send + Sync + 'static, message: Option<String>) -> Self {
        ApiError {
            code,
            message: message.unwrap_or_else(|| format!("{source}")),
            details: None,
            source: Some(Arc::new(source)),
        }
    }
    #[inline(always)]
    pub fn with_details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }
    #[inline]
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set_detail(key, value);
        self
    }
    #[inline]
    pub fn set_detail(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        if self.details.is_none() {
            self.details = Some(HashMap::new());
        }
        self.details.as_mut().unwrap().insert(key.into(), value.into());
        self
    }
    #[inline(always)]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.set_source(source);
        self
    }
    #[inline(always)]
    pub fn set_source(&mut self, source: impl Error + Send + Sync + 'static) -> &mut Self {
        self.source = Some(Arc::new(source));
        self
    }
    pub fn is<E: Error + 'static>(&self) -> bool {
        match &self.source {
            Some(source) => source.is::<E>(),
            None => false,
        }
    }
    pub fn downcast_ref<E: Error + 'static>(&self) -> Option<&E> {
        match &self.source {
            Some(source) => source.downcast_ref(),
            None => None,
        }
    }
    pub fn api_error_response<Data, Meta>(self, meta: Option<Meta>) -> ApiResponse<Data, Meta> {
        ApiResponse::Error(ErrorResponse { error: self, meta })
    }
    #[inline(always)]
    pub fn api_error_without_meta<Data, Meta>(self) -> ApiResponse<Data, Meta>
    where
        Self: Sized,
    {
        self.api_error_response(None)
    }
    #[inline(always)]
    pub fn api_error_with_meta<Data, Meta>(self, meta: Meta) -> ApiResponse<Data, Meta>
    where
        Self: Sized,
    {
        self.api_error_response(Some(meta))
    }
}
