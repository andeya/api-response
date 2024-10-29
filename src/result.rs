use crate::{ApiError, ApiResponse, ErrorResponse, SuccessResponse};

pub type ApiResult<Data, Meta> = Result<SuccessResponse<Data, Meta>, ErrorResponse<Meta>>;

impl<Data, Meta> From<ApiResult<Data, Meta>> for ApiResponse<Data, Meta>
where
    Meta: Default,
{
    fn from(result: ApiResult<Data, Meta>) -> Self {
        match result {
            Ok(success) => ApiResponse::Success(success),
            Err(error) => ApiResponse::Error(error),
        }
    }
}

impl<Data, Meta> From<ApiResponse<Data, Meta>> for ApiResult<Data, Meta> {
    fn from(api_response: ApiResponse<Data, Meta>) -> Self {
        match api_response {
            ApiResponse::Success(success) => Ok(success),
            ApiResponse::Error(error) => Err(error),
        }
    }
}

impl<Data, Meta> From<Result<Data, ApiError>> for ApiResponse<Data, Meta> {
    fn from(result: Result<Data, ApiError>) -> Self {
        match result {
            Ok(data) => ApiResponse::Success(SuccessResponse::from_data(data)),
            Err(error) => ApiResponse::Error(ErrorResponse::from_error(error)),
        }
    }
}

impl<Data, Meta> From<ApiResponse<Data, Meta>> for Result<Data, ApiError> {
    fn from(api_response: ApiResponse<Data, Meta>) -> Self {
        match api_response {
            ApiResponse::Success(success) => Ok(success.data),
            ApiResponse::Error(error) => Err(error.error),
        }
    }
}

impl<Data, Meta> From<SuccessResponse<Data, Meta>> for ApiResult<Data, Meta> {
    fn from(success_response: SuccessResponse<Data, Meta>) -> Self {
        Ok(success_response)
    }
}

impl<Data, Meta> From<SuccessResponse<Data, Meta>> for Result<Data, ApiError> {
    fn from(success_response: SuccessResponse<Data, Meta>) -> Self {
        Ok(success_response.data)
    }
}
