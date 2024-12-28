use std::{fmt::Display, ops::BitOr};

pub use CodeSegment::*;
pub use ErrorCode::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{ApiError, MaybeString};

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum ErrorCode {
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
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ErrorCode::CANCELLED => "The operation was cancelled.",
            ErrorCode::UNKNOWN => "Server internal exception or client-side parsing status error.",
            ErrorCode::INVALID_ARGUMENT => "Invalid request argument.",
            ErrorCode::DEADLINE_EXCEEDED => "No response received before Deadline expires.",
            ErrorCode::NOT_FOUND => "Some requested entity was not found.",
            ErrorCode::ALREADY_EXISTS => "The entity that is attempting to be created already exists.",
            ErrorCode::PERMISSION_DENIED => "No permission to execute the request.",
            ErrorCode::RESOURCE_EXHAUSTED => "Insufficient memory or message size exceeds the limit.",
            ErrorCode::FAILED_PRECONDITION => "Operation rejected, system not in required state.",
            ErrorCode::ABORTED => "Operation aborted due to concurrency issues",
            ErrorCode::OUT_OF_RANGE => "The operation was attempted past the valid range.",
            ErrorCode::UNIMPLEMENTED => "The received request/response is not supported.",
            ErrorCode::INTERNAL => "Internal errors indicate broken invariants.",
            ErrorCode::UNAVAILABLE => "The service is currently unavailable or there is a connection error.",
            ErrorCode::DATA_LOSS => "Unrecoverable data loss or corruption.",
            ErrorCode::UNAUTHENTICATED => {
                "The request does not have valid authentication credentials for the operation."
            }
        };
        write!(f, "{s}")
    }
}

impl ErrorCode {
    pub const fn maybe_client_error(self) -> bool {
        matches!(
            self,
            ErrorCode::CANCELLED
                | ErrorCode::UNKNOWN
                | ErrorCode::DEADLINE_EXCEEDED
                | ErrorCode::RESOURCE_EXHAUSTED
                | ErrorCode::UNIMPLEMENTED
                | ErrorCode::INTERNAL
                | ErrorCode::UNAVAILABLE
                | ErrorCode::UNAUTHENTICATED
        )
    }
    /// Generate an `ApiError` without code segment suffix.
    pub fn api_error0(self, message: impl Into<MaybeString>) -> ApiError {
        ApiError {
            code: self.into(),
            message: message.into().unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
    /// Generate an `ApiError`, and append 2*1 digits at the end of the current
    /// code in the form of a decimal literal.
    pub fn api_error1(self, s1: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        ApiError {
            code: self | s1,
            message: message.into().unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
    /// Generate an `ApiError`, and append 2*2 digits at the end of the current
    /// code in the form of a decimal literal.
    pub fn api_error2(self, s1: CodeSegment, s2: CodeSegment, message: impl Into<MaybeString>) -> ApiError {
        ApiError {
            code: self | s1 | s2,
            message: message.into().unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
    /// Generate an `ApiError`, and append 2*3 digits at the end of the current
    /// code in the form of a decimal literal.
    pub fn api_error3(
        self,
        s1: CodeSegment,
        s2: CodeSegment,
        s3: CodeSegment,
        message: impl Into<MaybeString>,
    ) -> ApiError {
        ApiError {
            code: self | s1 | s2 | s3,
            message: message.into().unwrap_or_else(|| self.to_string()),
            details: None,
            source: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum CodeSegment {
    S01 = 1,
    S02 = 2,
    S03 = 3,
    S04 = 4,
    S05 = 5,
    S06 = 6,
    S07 = 7,
    S08 = 8,
    S09 = 9,
    S10 = 10,
    S11 = 11,
    S12 = 12,
    S13 = 13,
    S14 = 14,
    S15 = 15,
    S16 = 16,
    S17 = 17,
    S18 = 18,
    S19 = 19,
    S20 = 20,
    S21 = 21,
    S22 = 22,
    S23 = 23,
    S24 = 24,
    S25 = 25,
    S26 = 26,
    S27 = 27,
    S28 = 28,
    S29 = 29,
    S30 = 30,
    S31 = 31,
    S32 = 32,
    S33 = 33,
    S34 = 34,
    S35 = 35,
    S36 = 36,
    S37 = 37,
    S38 = 38,
    S39 = 39,
    S40 = 40,
    S41 = 41,
    S42 = 42,
    S43 = 43,
    S44 = 44,
    S45 = 45,
    S46 = 46,
    S47 = 47,
    S48 = 48,
    S49 = 49,
    S50 = 50,
    S51 = 51,
    S52 = 52,
    S53 = 53,
    S54 = 54,
    S55 = 55,
    S56 = 56,
    S57 = 57,
    S58 = 58,
    S59 = 59,
    S60 = 60,
    S61 = 61,
    S62 = 62,
    S63 = 63,
    S64 = 64,
    S65 = 65,
    S66 = 66,
    S67 = 67,
    S68 = 68,
    S69 = 69,
    S70 = 70,
    S71 = 71,
    S72 = 72,
    S73 = 73,
    S74 = 74,
    S75 = 75,
    S76 = 76,
    S77 = 77,
    S78 = 78,
    S79 = 79,
    S80 = 80,
    S81 = 81,
    S82 = 82,
    S83 = 83,
    S84 = 84,
    S85 = 85,
    S86 = 86,
    S87 = 87,
    S88 = 88,
    S89 = 89,
    S90 = 90,
    S91 = 91,
    S92 = 92,
    S93 = 93,
    S94 = 94,
    S95 = 95,
    S96 = 96,
    S97 = 97,
    S98 = 98,
    S99 = 99,
}

impl From<CodeSegment> for i32 {
    #[allow(clippy::as_conversions)]
    fn from(value: CodeSegment) -> Self {
        value as i32
    }
}

const OVERFLOW: &str = "A calculation overflow occurs when generating segmented-error-code.";

/// Append two digits at the end in the form of a decimal literal.
impl BitOr<CodeSegment> for ErrorCode {
    type Output = i32;

    fn bitor(self, rhs: CodeSegment) -> Self::Output {
        i32::from(self)
            .checked_mul(100)
            .expect(OVERFLOW)
            .checked_add(i32::from(rhs))
            .expect(OVERFLOW)
    }
}

/// Append two digits at the end in the form of a decimal literal.
impl BitOr<CodeSegment> for i32 {
    type Output = i32;

    fn bitor(self, rhs: CodeSegment) -> Self::Output {
        self.checked_mul(100)
            .expect(OVERFLOW)
            .checked_add(i32::from(rhs))
            .expect(OVERFLOW)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
#[non_exhaustive]
pub enum CodeSegment2 {
    S01 = 1,
    S02 = 2,
    S03 = 3,
    S04 = 4,
}
api_code_macros::enum_segment!(CodeSegment3, u8, 1, 4, S, 2);
