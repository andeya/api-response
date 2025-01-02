use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
};

use crate::{ApiError, ApiResponse, ErrorResponse, SuccessResponse};

impl<Data, Meta> FromResidual<Result<Infallible, ErrorResponse<Meta>>> for ApiResponse<Data, Meta> {
    fn from_residual(residual: Result<Infallible, ErrorResponse<Meta>>) -> Self {
        ApiResponse::Error(residual.unwrap_err())
    }
}

impl<Data, Meta> FromResidual<Result<Infallible, ApiError>> for ApiResponse<Data, Meta> {
    fn from_residual(residual: Result<Infallible, ApiError>) -> Self {
        ApiResponse::Error(ErrorResponse::from_error(residual.unwrap_err()))
    }
}

impl<T, Meta> FromResidual<ApiResponse<Infallible, Meta>> for Result<T, ErrorResponse<Meta>> {
    fn from_residual(residual: ApiResponse<Infallible, Meta>) -> Self {
        Err(residual.unwrap_err())
    }
}

impl<T, Meta> FromResidual<ApiResponse<Infallible, Meta>> for Result<T, ApiError> {
    fn from_residual(residual: ApiResponse<Infallible, Meta>) -> Self {
        Err(residual.unwrap_err().error)
    }
}

impl<Data, Meta> Try for ApiResponse<Data, Meta> {
    type Output = SuccessResponse<Data, Meta>;

    type Residual = ApiResponse<Infallible, Meta>;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(v) => ControlFlow::Continue(v),
            Self::Error(e) => ControlFlow::Break(ApiResponse::Error(e)),
        }
    }
}

impl<Data, Meta> FromResidual<ApiResponse<Infallible, Meta>> for ApiResponse<Data, Meta> {
    fn from_residual(residual: ApiResponse<Infallible, Meta>) -> Self {
        ApiResponse::Error(residual.unwrap_err())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    fn from_result_residual<Data, Meta>() -> ApiResponse<Data, Meta> {
        Err(ErrorResponse::from_error_msg(1u32, "message"))?;
        Err(ApiError::new(1u32, "message"))?
    }
    fn from_self_residual<Data, Meta>() -> ApiResponse<Data, Meta> {
        let x: SuccessResponse<Data, Meta> = ApiResponse::from_error_msg(1u32, "message")?;
        x.into()
    }
    fn into_result_residual<Data, Meta>() -> ApiResult<Data, Meta> {
        let x: SuccessResponse<Data, Meta> = ApiResponse::from_error_msg(1u32, "message")?;
        x.into()
    }
    fn into_result_residual2<Data>() -> Result<Data, ApiError> {
        let x: SuccessResponse<_, ()> = ApiResponse::<_, ()>::from_error_msg(1u32, "message")?;
        x.into()
    }
    #[test]
    fn test_from_residual() {
        println!("{:?}", from_result_residual::<u8, u8>());
        println!("{:?}", from_self_residual::<u8, u8>());
        println!("{:?}", into_result_residual::<u8, u8>());
        println!("{:?}", into_result_residual2::<u8>());
    }
}
