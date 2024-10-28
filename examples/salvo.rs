use std::num::ParseIntError;

use api_response::{ApiResponse, ApiSuccessResponse, DefaultMeta, ErrorInfo};
use salvo::prelude::*;
use serde_json::{json, Value};

/// get user
#[cfg_attr(feature = "salvo", endpoint)]
#[cfg_attr(not(feature = "salvo"), handler)]
async fn get_user() -> Json<ApiResponse<Value, DefaultMeta>> {
    let user = json!({
        "id": 123,
        "name": "Andeya Lee",
        "email": "andeya.lee@example.com"
    });
    Json(user.api_success_with_meta(DefaultMeta::new("abc-123")))
}

/// get error
#[cfg_attr(feature = "salvo", endpoint)]
#[cfg_attr(not(feature = "salvo"), handler)]
async fn get_error() -> Json<ApiResponse<Value, ()>> {
    let err: ParseIntError = "@".parse::<u8>().unwrap_err();
    let error = ErrorInfo::new(400, "Invalid input data")
        .with_detail("email", "Invalid email format")
        .with_source(err);
    println!("error={:?}", error.downcast_ref::<ParseIntError>().unwrap());
    Json(error.api_error_without_meta())
}

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    let mut router = Router::new()
        .get(get_user)
        .push(Router::with_path("error").get(get_error));
    #[cfg(feature = "salvo")]
    {
        let doc = OpenApi::new("API-Response", "1").merge_router(&router);
        router = router
            .push(doc.into_router("/api-doc/openapi.json"))
            .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));
    }
    Server::new(TcpListener::new("127.0.0.1:7878").bind().await)
        .serve(router)
        .await;
}
