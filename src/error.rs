use std::{collections::HashMap, error::Error, fmt, sync::Arc};

use serde::{Deserialize, Serialize};

/// Struct to represent an error response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<Meta> {
    pub error: ErrorInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<Meta> ErrorResponse<Meta> {
    #[inline(always)]
    pub fn new(error: ErrorInfo) -> Self {
        ErrorResponse { error, meta: None }
    }
    #[inline(always)]
    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
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
pub struct ErrorInfo {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
    #[serde(skip)]
    pub source: Option<Arc<dyn Error + Send + Sync + 'static>>,
}

impl fmt::Debug for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorInfo")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("details", &self.details)
            .finish()
    }
}

impl fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorInfo {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

impl ErrorInfo {
    #[inline(always)]
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        ErrorInfo {
            code,
            message: message.into(),
            details: None,
            source: None,
        }
    }
    #[inline(always)]
    pub fn with_details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }
    #[inline]
    pub fn insert_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.details.is_none() {
            self.details = Some(HashMap::new());
        }
        self.details.as_mut().unwrap().insert(key.into(), value.into());
        self
    }
    #[inline(always)]
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
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
}
