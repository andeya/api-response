#![cfg(all(feature = "salvo", feature = "try"))]

use std::num::ParseIntError;

use api_response::{error_status::*, ApiError, ApiResponse, ApiSuccessResponse, DefaultMeta};
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct User {
    id: i64,
    name: &'static str,
    email: &'static str,
}

/// get user
#[endpoint]
async fn get_user() -> ApiResponse<User, DefaultMeta> {
    let user = User {
        id: "123"
            .parse()
            .map_err(|e| ApiError::from_source(1, e, true, ()).into())?,
        name: "Andeya Lee",
        email: "andeya.lee@example.com",
    };
    user.api_response_with_meta(DefaultMeta::new("abc-123"))
}

/// get error
#[endpoint]
async fn get_error() -> ApiResponse<(), ()> {
    let err: ParseIntError = "@".parse::<u8>().unwrap_err();
    let api_error = INVALID_ARGUMENT
        .api_error_one_segment(S01, Some("Invalid input data"))
        .with_detail("email", "Invalid email format")
        .with_source(err, true);
    println!("api_error={:?}", api_error.downcast_ref::<ParseIntError>().unwrap());
    api_error.api_response_without_meta()
}

#[tokio::main]
async fn main() {
    let mut router = Router::new()
        .get(get_user)
        .push(Router::with_path("error").get(get_error));
    let doc = OpenApi::new("API-Response", "1").merge_router(&router);
    router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));
    Server::new(TcpListener::new("127.0.0.1:7878").bind().await)
        .serve(router)
        .await;
}
