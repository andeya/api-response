//! Error type standard designed with reference to [gRPC status codes](https://grpc.io/docs/guides/status-codes/).

use http::StatusCode;

use super::{ErrSegment, ErrType};

pub const CANCELLED: ErrType = ErrSegment::E100.new_err_type("The operation was cancelled.");
pub const UNKNOWN: ErrType =
    ErrSegment::E101.new_err_type("Server internal exception or client-side parsing status error.");
pub const INVALID_ARGUMENT: ErrType = ErrSegment::E102.new_err_type("Invalid request argument.");
pub const DEADLINE_EXCEEDED: ErrType = ErrSegment::E103.new_err_type("No response received before Deadline expires.");
pub const NOT_FOUND: ErrType = ErrSegment::E104.new_err_type("Some requested entity was not found.");
pub const ALREADY_EXISTS: ErrType =
    ErrSegment::E105.new_err_type("The entity that is attempting to be created already exists.");
pub const PERMISSION_DENIED: ErrType = ErrSegment::E106.new_err_type("No permission to execute the request.");
pub const RESOURCE_EXHAUSTED: ErrType =
    ErrSegment::E107.new_err_type("Insufficient memory or message size exceeds the limit.");
pub const FAILED_PRECONDITION: ErrType =
    ErrSegment::E108.new_err_type("Operation rejected, system not in required state.");
pub const ABORTED: ErrType = ErrSegment::E109.new_err_type("Operation aborted due to concurrency issues");
pub const OUT_OF_RANGE: ErrType = ErrSegment::E110.new_err_type("The operation was attempted past the valid range.");
pub const UNIMPLEMENTED: ErrType = ErrSegment::E111.new_err_type("The received request/response is not supported.");
pub const INTERNAL: ErrType = ErrSegment::E112.new_err_type("Internal errors indicate broken invariants.");
pub const UNAVAILABLE: ErrType =
    ErrSegment::E113.new_err_type("The service is currently unavailable or there is a connection error.");
pub const DATA_LOSS: ErrType = ErrSegment::E114.new_err_type("Unrecoverable data loss or corruption.");
pub const UNAUTHENTICATED: ErrType =
    ErrSegment::E115.new_err_type("The request does not have valid authentication credentials for the operation.");

impl From<ErrType> for StatusCode {
    fn from(value: ErrType) -> Self {
        Self::from(value.err_segment())
    }
}
impl From<ErrSegment> for StatusCode {
    fn from(value: ErrSegment) -> Self {
        match value {
            v if v == CANCELLED.err_segment() => unsafe { StatusCode::from_u16(499).unwrap_unchecked() },
            v if v == INVALID_ARGUMENT.err_segment() => StatusCode::BAD_REQUEST,
            v if v == DEADLINE_EXCEEDED.err_segment() => StatusCode::GATEWAY_TIMEOUT,
            v if v == NOT_FOUND.err_segment() => StatusCode::NOT_FOUND,
            v if v == ALREADY_EXISTS.err_segment() => StatusCode::CONFLICT,
            v if v == PERMISSION_DENIED.err_segment() => StatusCode::FORBIDDEN,
            v if v == RESOURCE_EXHAUSTED.err_segment() => StatusCode::INSUFFICIENT_STORAGE,
            v if v == FAILED_PRECONDITION.err_segment() => StatusCode::PRECONDITION_FAILED,
            v if v == ABORTED.err_segment() => StatusCode::CONFLICT,
            v if v == OUT_OF_RANGE.err_segment() => StatusCode::RANGE_NOT_SATISFIABLE,
            v if v == UNIMPLEMENTED.err_segment() => StatusCode::NOT_IMPLEMENTED,
            v if v == INTERNAL.err_segment() => StatusCode::INTERNAL_SERVER_ERROR,
            v if v == UNAVAILABLE.err_segment() => StatusCode::SERVICE_UNAVAILABLE,
            v if v == DATA_LOSS.err_segment() => StatusCode::GONE,
            v if v == UNAUTHENTICATED.err_segment() => StatusCode::UNAUTHORIZED,
            v if v == UNKNOWN.err_segment() => unsafe { StatusCode::from_u16(520).unwrap_unchecked() },
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
impl From<StatusCode> for ErrSegment {
    fn from(value: StatusCode) -> Self {
        ErrType::from(value).err_segment()
    }
}
