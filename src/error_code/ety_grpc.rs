//! Error type standard designed with reference to [gRPC status codes](https://grpc.io/docs/guides/status-codes/).

use http::StatusCode;

use super::{ErrFlag, ErrType};

pub const CANCELLED: ErrType = ErrFlag::E100.define("The operation was cancelled.");
pub const UNKNOWN: ErrType = ErrFlag::E101.define("Server internal exception or client-side parsing status error.");
pub const INVALID_ARGUMENT: ErrType = ErrFlag::E102.define("Invalid request argument.");
pub const DEADLINE_EXCEEDED: ErrType = ErrFlag::E103.define("No response received before Deadline expires.");
pub const NOT_FOUND: ErrType = ErrFlag::E104.define("Some requested entity was not found.");
pub const ALREADY_EXISTS: ErrType = ErrFlag::E105.define("The entity that is attempting to be created already exists.");
pub const PERMISSION_DENIED: ErrType = ErrFlag::E106.define("No permission to execute the request.");
pub const RESOURCE_EXHAUSTED: ErrType = ErrFlag::E107.define("Insufficient memory or message size exceeds the limit.");
pub const FAILED_PRECONDITION: ErrType = ErrFlag::E108.define("Operation rejected, system not in required state.");
pub const ABORTED: ErrType = ErrFlag::E109.define("Operation aborted due to concurrency issues");
pub const OUT_OF_RANGE: ErrType = ErrFlag::E110.define("The operation was attempted past the valid range.");
pub const UNIMPLEMENTED: ErrType = ErrFlag::E111.define("The received request/response is not supported.");
pub const INTERNAL: ErrType = ErrFlag::E112.define("Internal errors indicate broken invariants.");
pub const UNAVAILABLE: ErrType =
    ErrFlag::E113.define("The service is currently unavailable or there is a connection error.");
pub const DATA_LOSS: ErrType = ErrFlag::E114.define("Unrecoverable data loss or corruption.");
pub const UNAUTHENTICATED: ErrType =
    ErrFlag::E115.define("The request does not have valid authentication credentials for the operation.");

impl From<ErrType> for StatusCode {
    fn from(value: ErrType) -> Self {
        Self::from(value.flag())
    }
}
impl From<ErrFlag> for StatusCode {
    fn from(value: ErrFlag) -> Self {
        match value {
            v if v == CANCELLED.flag() => unsafe { StatusCode::from_u16(499).unwrap_unchecked() },
            v if v == INVALID_ARGUMENT.flag() => StatusCode::BAD_REQUEST,
            v if v == DEADLINE_EXCEEDED.flag() => StatusCode::GATEWAY_TIMEOUT,
            v if v == NOT_FOUND.flag() => StatusCode::NOT_FOUND,
            v if v == ALREADY_EXISTS.flag() => StatusCode::CONFLICT,
            v if v == PERMISSION_DENIED.flag() => StatusCode::FORBIDDEN,
            v if v == RESOURCE_EXHAUSTED.flag() => StatusCode::INSUFFICIENT_STORAGE,
            v if v == FAILED_PRECONDITION.flag() => StatusCode::PRECONDITION_FAILED,
            v if v == ABORTED.flag() => StatusCode::CONFLICT,
            v if v == OUT_OF_RANGE.flag() => StatusCode::RANGE_NOT_SATISFIABLE,
            v if v == UNIMPLEMENTED.flag() => StatusCode::NOT_IMPLEMENTED,
            v if v == INTERNAL.flag() => StatusCode::INTERNAL_SERVER_ERROR,
            v if v == UNAVAILABLE.flag() => StatusCode::SERVICE_UNAVAILABLE,
            v if v == DATA_LOSS.flag() => StatusCode::GONE,
            v if v == UNAUTHENTICATED.flag() => StatusCode::UNAUTHORIZED,
            v if v == UNKNOWN.flag() => unsafe { StatusCode::from_u16(520).unwrap_unchecked() },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl From<StatusCode> for ErrType {
    fn from(value: StatusCode) -> Self {
        match value.as_u16() {
            499 => CANCELLED,
            520 => UNKNOWN,
            400 => INVALID_ARGUMENT,
            504 => DEADLINE_EXCEEDED,
            404 => NOT_FOUND,
            409 => ALREADY_EXISTS,
            403 => PERMISSION_DENIED,
            507 => RESOURCE_EXHAUSTED,
            412 => FAILED_PRECONDITION,
            416 => OUT_OF_RANGE,
            501 => UNIMPLEMENTED,
            500 => INTERNAL,
            503 => UNAVAILABLE,
            410 => DATA_LOSS,
            401 => UNAUTHENTICATED,
            _ => INTERNAL,
        }
    }
}
impl From<StatusCode> for ErrFlag {
    fn from(value: StatusCode) -> Self {
        ErrType::from(value).flag()
    }
}
