//! Error type standard designed with reference to [gRPC status codes](https://grpc.io/docs/guides/status-codes/).

use http::StatusCode;

use super::ErrType;

pub const CANCELLED: ErrType = ErrType::T1000("The operation was cancelled.");
pub const UNKNOWN: ErrType = ErrType::T1001("Server internal exception or client-side parsing status error.");
pub const INVALID_ARGUMENT: ErrType = ErrType::T1002("Invalid request argument.");
pub const DEADLINE_EXCEEDED: ErrType = ErrType::T1003("No response received before Deadline expires.");
pub const NOT_FOUND: ErrType = ErrType::T1004("Some requested entity was not found.");
pub const ALREADY_EXISTS: ErrType = ErrType::T1005("The entity that is attempting to be created already exists.");
pub const PERMISSION_DENIED: ErrType = ErrType::T1006("No permission to execute the request.");
pub const RESOURCE_EXHAUSTED: ErrType = ErrType::T1007("Insufficient memory or message size exceeds the limit.");
pub const FAILED_PRECONDITION: ErrType = ErrType::T1008("Operation rejected, system not in required state.");
pub const ABORTED: ErrType = ErrType::T1009("Operation aborted due to concurrency issues");
pub const OUT_OF_RANGE: ErrType = ErrType::T1010("The operation was attempted past the valid range.");
pub const UNIMPLEMENTED: ErrType = ErrType::T1011("The received request/response is not supported.");
pub const INTERNAL: ErrType = ErrType::T1012("Internal errors indicate broken invariants.");
pub const UNAVAILABLE: ErrType = ErrType::T1013("The service is currently unavailable or there is a connection error.");
pub const DATA_LOSS: ErrType = ErrType::T1014("Unrecoverable data loss or corruption.");
pub const UNAUTHENTICATED: ErrType =
    ErrType::T1015("The request does not have valid authentication credentials for the operation.");

impl From<ErrType> for StatusCode {
    fn from(value: ErrType) -> Self {
        match value.flag() {
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
