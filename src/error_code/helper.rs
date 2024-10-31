use super::{
    CodeSegment,
    ErrorCode::{self, *},
};
use crate::{ApiError, MaybeString};

/// A builder for quickly creating `ApiError`.
pub struct ApiErr(Option<CodeSegment>, Option<CodeSegment>, Option<CodeSegment>);

impl ApiErr {
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}`.
    pub const fn new0() -> Self {
        Self(None, None, None)
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}`.
    pub const fn new1(s1: CodeSegment) -> Self {
        Self(Some(s1), None, None)
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}{CodeSegment}`.
    pub const fn new2(s1: CodeSegment, s2: CodeSegment) -> Self {
        Self(Some(s1), Some(s2), None)
    }
    /// Create an `ApiError` builder with an error code format of
    /// `{ErrorCode}{CodeSegment}{CodeSegment}{CodeSegment}`.
    pub const fn new3(s1: CodeSegment, s2: CodeSegment, s3: CodeSegment) -> Self {
        Self(Some(s1), Some(s2), Some(s3))
    }
    fn new_api_error(&self, error_code: ErrorCode, message: impl Into<MaybeString>) -> ApiError {
        if let Some(c) = self.2 {
            return error_code.api_error3(self.0.unwrap(), self.1.unwrap(), c, message);
        }
        if let Some(b) = self.1 {
            return error_code.api_error2(self.0.unwrap(), b, message);
        }
        if let Some(a) = self.0 {
            return error_code.api_error1(a, message);
        }
        error_code.api_error0(message)
    }
    pub fn cancelled(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(CANCELLED, message)
    }
    pub fn unknown(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNKNOWN, message)
    }
    pub fn invalid_argument(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(INVALID_ARGUMENT, message)
    }
    pub fn deadline_exceeded(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(DEADLINE_EXCEEDED, message)
    }
    pub fn not_found(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(NOT_FOUND, message)
    }
    pub fn already_exists(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(ALREADY_EXISTS, message)
    }
    pub fn permission_denied(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(PERMISSION_DENIED, message)
    }
    pub fn resource_exhausted(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(RESOURCE_EXHAUSTED, message)
    }
    pub fn failed_precondition(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(FAILED_PRECONDITION, message)
    }
    pub fn aborted(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(ABORTED, message)
    }
    pub fn out_of_range(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(OUT_OF_RANGE, message)
    }
    pub fn unimplemented(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNIMPLEMENTED, message)
    }
    pub fn internal(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(INTERNAL, message)
    }
    pub fn unavailable(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNAVAILABLE, message)
    }
    pub fn data_loss(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(DATA_LOSS, message)
    }
    pub fn unauthenticated(&self, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNAUTHENTICATED, message)
    }
}

/// A builder for quickly creating `ApiError` that allows flexible specification of the last segment.
pub struct ApiErrX(Option<CodeSegment>, Option<CodeSegment>);

impl ApiErrX {
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}`.
    pub const fn new1() -> Self {
        Self(None, None)
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}{CodeSegment}`.
    pub const fn new2(s1: CodeSegment) -> Self {
        Self(Some(s1), None)
    }
    /// Create an `ApiError` builder with an error code format of
    /// `{ErrorCode}{CodeSegment}{CodeSegment}{CodeSegment}`.
    pub const fn new3(s1: CodeSegment, s2: CodeSegment) -> Self {
        Self(Some(s1), Some(s2))
    }
    fn new_api_error(&self, error_code: ErrorCode, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        if let Some(b) = self.1 {
            return error_code.api_error3(self.0.unwrap(), b, s, message);
        }
        if let Some(a) = self.0 {
            return error_code.api_error2(a, s, message);
        }
        error_code.api_error1(s, message)
    }
    pub fn cancelled(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(CANCELLED, s, message)
    }
    pub fn unknown(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNKNOWN, s, message)
    }
    pub fn invalid_argument(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(INVALID_ARGUMENT, s, message)
    }
    pub fn deadline_exceeded(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(DEADLINE_EXCEEDED, s, message)
    }
    pub fn not_found(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(NOT_FOUND, s, message)
    }
    pub fn already_exists(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(ALREADY_EXISTS, s, message)
    }
    pub fn permission_denied(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(PERMISSION_DENIED, s, message)
    }
    pub fn resource_exhausted(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(RESOURCE_EXHAUSTED, s, message)
    }
    pub fn failed_precondition(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(FAILED_PRECONDITION, s, message)
    }
    pub fn aborted(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(ABORTED, s, message)
    }
    pub fn out_of_range(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(OUT_OF_RANGE, s, message)
    }
    pub fn unimplemented(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNIMPLEMENTED, s, message)
    }
    pub fn internal(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(INTERNAL, s, message)
    }
    pub fn unavailable(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNAVAILABLE, s, message)
    }
    pub fn data_loss(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(DATA_LOSS, s, message)
    }
    pub fn unauthenticated(&self, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        self.new_api_error(UNAUTHENTICATED, s, message)
    }
}
