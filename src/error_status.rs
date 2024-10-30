//！ https://grpc.github.io/grpc/core/md_doc_statuscodes.html

use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::ApiError;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ErrorStatus {
    CANCELLED = 10,
    UNKNOWN = 11,
    INVALID_ARGUMENT = 12,
    DEADLINE_EXCEEDED = 13,
    NOT_FOUND = 14,
    ALREADY_EXISTS = 15,
    PERMISSION_DENIED = 16,
    RESOURCE_EXHAUSTED = 17,
    FAILED_PRECONDITION = 18,
    ABORTED = 19,
    OUT_OF_RANGE = 20,
    UNIMPLEMENTED = 21,
    INTERNAL = 22,
    UNAVAILABLE = 23,
    DATA_LOSS = 24,
    UNAUTHENTICATED = 25,
    #[num_enum(catch_all)]
    Other(i32),
}

impl ErrorStatus {}

impl Display for ErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ErrorStatus::CANCELLED => "The operation was cancelled.",
            ErrorStatus::UNKNOWN => "Server internal exception or client-side parsing status error.",
            ErrorStatus::INVALID_ARGUMENT => "Invalid request argument.",
            ErrorStatus::DEADLINE_EXCEEDED => "No response received before Deadline expires.",
            ErrorStatus::NOT_FOUND => "Some requested entity was not found.",
            ErrorStatus::ALREADY_EXISTS => "The entity that is attempting to be created already exists.",
            ErrorStatus::PERMISSION_DENIED => "No permission to execute the request.",
            ErrorStatus::RESOURCE_EXHAUSTED => "Insufficient memory or message size exceeds the limit.",
            ErrorStatus::FAILED_PRECONDITION => "Operation rejected, system not in required state.",
            ErrorStatus::ABORTED => "Operation aborted due to concurrency issues",
            ErrorStatus::OUT_OF_RANGE => "The operation was attempted past the valid range.",
            ErrorStatus::UNIMPLEMENTED => "The received request/response is not supported.",
            ErrorStatus::INTERNAL => "Internal errors indicate broken invariants.",
            ErrorStatus::UNAVAILABLE => "The service is currently unavailable or there is a connection error.",
            ErrorStatus::DATA_LOSS => "Unrecoverable data loss or corruption.",
            ErrorStatus::UNAUTHENTICATED => {
                "The request does not have valid authentication credentials for the operation."
            }
            ErrorStatus::Other(c) => &format!("Undefined other error code({c})"),
        };
        write!(f, "{s}")
    }
}

impl ErrorStatus {
    pub fn maybe_client_error(self) -> bool {
        match self {
            ErrorStatus::CANCELLED
            | ErrorStatus::UNKNOWN
            | ErrorStatus::DEADLINE_EXCEEDED
            | ErrorStatus::RESOURCE_EXHAUSTED
            | ErrorStatus::UNIMPLEMENTED
            | ErrorStatus::INTERNAL
            | ErrorStatus::UNAVAILABLE
            | ErrorStatus::UNAUTHENTICATED
            | ErrorStatus::Other(_) => true,
            _ => false,
        }
    }
    /// Generate an ApiError.
    pub fn api_error(self, message: Option<String>) -> ApiError {
        ApiError {
            code: self.into(),
            message: message.unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
    /// Generate an ApiError with an added error code suffix.
    /// NOTE:
    ///  Add the absolute value of the suffix to the end of the current code.
    ///  The sign of the suffix determines the sign of the code.
    pub fn expand_api_error(self, code_suffix: impl Into<i32>, message: Option<String>) -> ApiError {
        let code_suffix = code_suffix.into();
        let mut code = Into::<i32>::into(self) * 100 + code_suffix.abs();
        if code_suffix < 0 {
            code = -code;
        }
        ApiError {
            code,
            message: message.unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
}
