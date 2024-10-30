use std::num::ParseIntError;

use api_response::{error_status::*, ApiResponse, ApiSuccessResponse, DefaultMeta};
use salvo::prelude::*;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, ToSchema)]
struct User {
    id: i64,
    name: &'static str,
    email: &'static str,
}

/// get user
#[handler]
async fn get_user() -> Json<ApiResponse<User, DefaultMeta>> {
    let user = User {
        id: 123,
        name: "Andeya Lee",
        email: "andeya.lee@example.com",
    };
    Json(user.api_response_with_meta(DefaultMeta::new("abc-123")))
}

/// get error
#[handler]
async fn get_error() -> Json<ApiResponse<Value, ()>> {
    let err: ParseIntError = "@".parse::<u8>().unwrap_err();
    let api_error = INVALID_ARGUMENT
        .api_error_one_segment(S01, Some("Invalid input data"))
        .with_detail("email", "Invalid email format")
        .with_source(err, true);
    println!("api_error={:?}", api_error.downcast_ref::<ParseIntError>().unwrap());
    Json(api_error.api_response_without_meta())
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get(get_user)
        .push(Router::with_path("error").get(get_error));
    Server::new(TcpListener::new("127.0.0.1:7878").bind().await)
        .serve(router)
        .await;
}
