use std::{collections::HashMap, error::Error, fmt, sync::Arc};

use serde::{Deserialize, Serialize};

/// Struct to represent an error response
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<Meta> {
    pub error: ErrorInfo,
    pub meta: Meta,
}

impl<Meta> ErrorResponse<Meta> {
    pub fn new(error: ErrorInfo, meta: Meta) -> Self {
        ErrorResponse { error, meta }
    }
}

/// Struct to represent error information
#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: i32,
    pub message: String,
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
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        ErrorInfo {
            code,
            message: message.into(),
            details: None,
            source: None,
        }
    }
    pub fn with_details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Arc::new(source));
        self
    }
    pub fn downcast_ref<E: Error + 'static>(&self) -> Option<&E> {
        match &self.source {
            Some(source) => source.downcast_ref(),
            None => None,
        }
    }
}
