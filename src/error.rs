use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// Struct to represent error information
#[derive(Serialize, Deserialize)]
pub struct ErrorInfo<Details> {
    pub code: i32,
    pub message: String,
    pub details: Option<Details>,
    #[serde(skip)]
    pub source: Option<Box<dyn Error>>,
}

impl<Details> ErrorInfo<Details> {
    pub fn new<S: Into<String>>(code: i32, message: S, details: Option<Details>, source: Option<Box<dyn Error>>) -> Self {
        ErrorInfo {
            code,
            message: message.into(),
            details,
            source,
        }
    }
}

impl<Details: fmt::Debug> fmt::Debug for ErrorInfo<Details> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorInfo")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("details", &self.details)
            .finish()
    }
}

impl<Details: fmt::Display> fmt::Display for ErrorInfo<Details> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<Details: Error + 'static> Error for ErrorInfo<Details> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

/// Struct to represent an error response
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<M, Details> {
    pub metadata: M,
    pub error: ErrorInfo<Details>,
}

impl<M, Details> ErrorResponse<M, Details> {
    pub fn new(metadata: M, error: ErrorInfo<Details>) -> Self {
        ErrorResponse { metadata, error }
    }
}
