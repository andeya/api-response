use std::{self, collections::HashMap, error::Error, fmt, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{ApiResponse, MaybeString, utils::OrderedHashMap};

/// Struct to represent an error response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ErrorResponse<Meta> {
    pub error: ApiError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<Meta> ErrorResponse<Meta> {
    #[inline(always)]
    pub const fn new(error: ApiError, meta: Meta) -> Self {
        ErrorResponse {
            error,
            meta: Some(meta),
        }
    }
    #[inline(always)]
    pub const fn from_error(error: ApiError) -> Self {
        ErrorResponse { error, meta: None }
    }
    #[inline(always)]
    pub fn from_error_msg(code: impl Into<u32>, message: impl Into<String>) -> Self {
        Self::from_error(ApiError::new(code, message))
    }
    #[inline(always)]
    pub fn from_error_source(
        code: impl Into<u32>,
        source: impl Error + Send + Sync + 'static,
        set_source_detail: bool,
        message: impl Into<MaybeString>,
    ) -> Self {
        Self::from_error(ApiError::from_source(code, source, set_source_detail, message))
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
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static, set_source_detail: bool) -> Self {
        self.set_source(source, set_source_detail);
        self
    }
    #[inline(always)]
    pub fn set_source(&mut self, source: impl Error + Send + Sync + 'static, set_source_detail: bool) -> &mut Self {
        self.error.set_source(source, set_source_detail);
        self
    }
    #[inline]
    pub const fn code(&self) -> u32 {
        self.error.code()
    }
    #[inline]
    pub const fn message(&self) -> &String {
        self.error.message()
    }
    #[inline]
    pub fn details(&self) -> Option<&HashMap<String, String>> {
        self.error.details()
    }
    #[inline]
    pub fn detail(&self, key: impl AsRef<str>) -> Option<&String> {
        self.error.detail(key)
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
        write!(f, "{}", self.error)
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
    pub(crate) code: u32,
    pub(crate) message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) details: Option<OrderedHashMap<String, String>>,
    #[serde(skip)]
    pub(crate) source: Option<Arc<dyn Error + Send + Sync + 'static>>,
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
        write!(f, "{} ErrCode({})", self.message, self.code)
    }
}

impl Error for ApiError {
    #[allow(clippy::as_conversions)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

impl ApiError {
    #[inline(always)]
    pub fn new(code: impl Into<u32>, message: impl Into<String>) -> Self {
        ApiError {
            code: code.into(),
            message: message.into(),
            details: None,
            source: None,
        }
    }
    #[inline(always)]
    pub fn from_source(
        code: impl Into<u32>,
        source: impl Error + Send + Sync + 'static,
        set_source_detail: bool,
        message: impl Into<MaybeString>,
    ) -> Self {
        let mut e = ApiError {
            code: code.into(),
            message: message
                .into()
                .option_string()
                .map_or_else(|| source.to_string(), Into::into),
            details: None,
            source: Some(Arc::new(source)),
        };
        if set_source_detail {
            e.set_source_detail();
        }
        e
    }
    pub fn with_code(mut self, code: impl Into<u32>) -> Self {
        self.code = code.into();
        self
    }
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
    #[inline(always)]
    pub fn with_details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(OrderedHashMap(details));
        self
    }
    #[inline]
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set_detail(key, value);
        self
    }
    #[inline]
    pub fn set_detail(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.details.get_or_insert_default().insert(key.into(), value.into());
        self
    }
    #[inline(always)]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static, set_source_detail: bool) -> Self {
        self.set_source(source, set_source_detail);
        self
    }
    #[inline(always)]
    pub fn set_source(&mut self, source: impl Error + Send + Sync + 'static, set_source_detail: bool) -> &mut Self {
        self.source = Some(Arc::new(source));
        if set_source_detail {
            self.set_source_detail();
        }
        self
    }
    /// Insert the setted source error into the detail field.
    #[inline(always)]
    fn set_source_detail(&mut self) -> &mut Self {
        if let Some(source) = &self.source {
            self.set_detail("source", source.to_string());
        }
        self
    }
    #[inline]
    pub const fn code(&self) -> u32 {
        self.code
    }
    #[inline]
    pub const fn message(&self) -> &String {
        &self.message
    }
    #[inline]
    pub fn details(&self) -> Option<&HashMap<String, String>> {
        self.details.as_deref()
    }
    #[inline]
    pub fn detail(&self, key: impl AsRef<str>) -> Option<&String> {
        self.details.as_ref()?.get(key.as_ref())
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
    pub const fn api_response<Data, Meta>(self, meta: Option<Meta>) -> ApiResponse<Data, Meta> {
        ApiResponse::Error(ErrorResponse { error: self, meta })
    }
    #[inline(always)]
    pub const fn api_response_without_meta<Data, Meta>(self) -> ApiResponse<Data, Meta>
    where
        Self: Sized,
    {
        self.api_response(None)
    }
    #[inline(always)]
    pub const fn api_response_with_meta<Data, Meta>(self, meta: Meta) -> ApiResponse<Data, Meta>
    where
        Self: Sized,
    {
        self.api_response(Some(meta))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApiError, ErrorResponse,
        error_code::{ErrBrief, ErrDecl, ErrPath, ErrPathParent, ErrPathRoot, ErrType},
    };
    #[test]
    fn display() {
        const ET: ErrType = ErrType::T1100("The operation was cancelled.");

        const EP0: ErrPathRoot = ErrPathRoot::X00("module 0");
        const EP1: ErrPathParent = EP0.Y01("module 01");
        const EP2: ErrPath = EP1.Z20("module 20");

        const ED: ErrDecl = ET.declare(EP2);
        const EB: ErrBrief = ED.extract();

        let api_error: ApiError = ET | &EP2;
        assert_eq!(EB.api_error().code(), api_error.code());
        assert_eq!(
            "The operation was cancelled. ErrCode(1100000120), X00(module 0)/Y01(module 01)/Z20(module 20)",
            ED.to_string()
        );
        let api_error: ApiError = ED.api_error();
        assert_eq!(
            "The operation was cancelled. ErrCode(1100000120)",
            api_error.to_string()
        );
        let err_resp: ErrorResponse<()> = ErrorResponse::from_error(api_error);
        assert_eq!("The operation was cancelled. ErrCode(1100000120)", err_resp.to_string());

        assert_eq!("The operation was cancelled.", ET.text());
        let et2: ErrType = ET | "The request was cancelled.";
        assert_eq!("The request was cancelled.", et2.text());
    }
}
