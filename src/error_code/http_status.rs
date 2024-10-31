use http::StatusCode;

use super::ErrorCode;

impl From<ErrorCode> for StatusCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::CANCELLED => StatusCode::from_u16(499).unwrap(),
            ErrorCode::UNKNOWN => StatusCode::from_u16(520).unwrap(),
            ErrorCode::INVALID_ARGUMENT => StatusCode::BAD_REQUEST,
            ErrorCode::DEADLINE_EXCEEDED => StatusCode::GATEWAY_TIMEOUT,
            ErrorCode::NOT_FOUND => StatusCode::NOT_FOUND,
            ErrorCode::ALREADY_EXISTS => StatusCode::CONFLICT,
            ErrorCode::PERMISSION_DENIED => StatusCode::FORBIDDEN,
            ErrorCode::RESOURCE_EXHAUSTED => StatusCode::INSUFFICIENT_STORAGE,
            ErrorCode::FAILED_PRECONDITION => StatusCode::PRECONDITION_FAILED,
            ErrorCode::ABORTED => StatusCode::CONFLICT,
            ErrorCode::OUT_OF_RANGE => StatusCode::RANGE_NOT_SATISFIABLE,
            ErrorCode::UNIMPLEMENTED => StatusCode::NOT_IMPLEMENTED,
            ErrorCode::INTERNAL => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::UNAVAILABLE => StatusCode::SERVICE_UNAVAILABLE,
            ErrorCode::DATA_LOSS => StatusCode::GONE,
            ErrorCode::UNAUTHENTICATED => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<StatusCode> for ErrorCode {
    fn from(value: StatusCode) -> Self {
        match value.as_u16() {
            499 => ErrorCode::CANCELLED,
            520 => ErrorCode::UNKNOWN,
            400 => ErrorCode::INVALID_ARGUMENT,
            504 => ErrorCode::DEADLINE_EXCEEDED,
            404 => ErrorCode::NOT_FOUND,
            409 => ErrorCode::ALREADY_EXISTS,
            403 => ErrorCode::PERMISSION_DENIED,
            507 => ErrorCode::RESOURCE_EXHAUSTED,
            412 => ErrorCode::FAILED_PRECONDITION,
            416 => ErrorCode::OUT_OF_RANGE,
            501 => ErrorCode::UNIMPLEMENTED,
            500 => ErrorCode::INTERNAL,
            503 => ErrorCode::UNAVAILABLE,
            410 => ErrorCode::DATA_LOSS,
            401 => ErrorCode::UNAUTHENTICATED,
            _ => ErrorCode::INTERNAL,
        }
    }
}
