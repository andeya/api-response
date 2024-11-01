use std::fmt::Display;

use super::{
    CodeSegment,
    ErrorCode::{self, *},
};
use crate::{ApiError, MaybeString};

/// A builder for quickly creating `ApiError`.
#[derive(Debug)]
pub struct ApiErr {
    pub(crate) intro: &'static str,
    pub(crate) s1: Option<CodeSegment>,
    pub(crate) s2: Option<CodeSegment>,
    pub(crate) s3: Option<CodeSegment>,
}

impl ApiErr {
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}`.
    pub const fn new0() -> Self {
        Self {
            intro: "",
            s1: None,
            s2: None,
            s3: None,
        }
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}`.
    pub const fn new1(s1: CodeSegment) -> Self {
        Self {
            intro: "",
            s1: Some(s1),
            s2: None,
            s3: None,
        }
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}{CodeSegment}`.
    pub const fn new2(s1: CodeSegment, s2: CodeSegment) -> Self {
        Self {
            intro: "",
            s1: Some(s1),
            s2: Some(s2),
            s3: None,
        }
    }
    /// Create an `ApiError` builder with an error code format of
    /// `{ErrorCode}{CodeSegment}{CodeSegment}{CodeSegment}`.
    pub const fn new3(s1: CodeSegment, s2: CodeSegment, s3: CodeSegment) -> Self {
        Self {
            intro: "",
            s1: Some(s1),
            s2: Some(s2),
            s3: Some(s3),
        }
    }
    pub const fn intro(mut self, intro: &'static str) -> Self {
        self.intro = intro;
        self
    }
    fn new_api_error(&self, error_code: ErrorCode, message: impl Into<MaybeString>) -> ApiError {
        if let Some(s3) = self.s3 {
            return error_code.api_error3(self.s1.unwrap(), self.s2.unwrap(), s3, message);
        }
        if let Some(s2) = self.s2 {
            return error_code.api_error2(self.s1.unwrap(), s2, message);
        }
        if let Some(s1) = self.s1 {
            return error_code.api_error1(s1, message);
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
pub struct ApiErrX {
    intro: &'static str,
    s1: Option<CodeSegment>,
    s2: Option<CodeSegment>,
}

impl ApiErrX {
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}`.
    pub const fn new1() -> Self {
        Self {
            intro: "",
            s1: None,
            s2: None,
        }
    }
    /// Create an `ApiError` builder with an error code format of `{ErrorCode}{CodeSegment}{CodeSegment}`.
    pub const fn new2(s1: CodeSegment) -> Self {
        Self {
            intro: "",
            s1: Some(s1),
            s2: None,
        }
    }
    /// Create an `ApiError` builder with an error code format of
    /// `{ErrorCode}{CodeSegment}{CodeSegment}{CodeSegment}`.
    pub const fn new3(s1: CodeSegment, s2: CodeSegment) -> Self {
        Self {
            intro: "",
            s1: Some(s1),
            s2: Some(s2),
        }
    }
    pub const fn intro(mut self, intro: &'static str) -> Self {
        self.intro = intro;
        self
    }
    fn new_api_error(&self, error_code: ErrorCode, s: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        if let Some(s2) = self.s2 {
            return error_code.api_error3(self.s1.unwrap(), s2, s, message);
        }
        if let Some(s1) = self.s1 {
            return error_code.api_error2(s1, s, message);
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

impl Display for ApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const A: &str = "**";
        write!(
            f,
            "[??{:0>2}{:0>2}{:0>2}]: {}",
            self.s1.map_or_else(|| A.to_owned(), |v| (v as i32).to_string()),
            self.s2.map_or_else(|| A.to_owned(), |v| (v as i32).to_string()),
            self.s3.map_or_else(|| A.to_owned(), |v| (v as i32).to_string()),
            self.intro
        )
    }
}

impl Display for ApiErrX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const A: &str = "**";
        write!(
            f,
            "[??{:0>2}{:0>2}**]: {}",
            self.s1.map_or_else(|| A.to_owned(), |v| (v as i32).to_string()),
            self.s2.map_or_else(|| A.to_owned(), |v| (v as i32).to_string()),
            self.intro
        )
    }
}
