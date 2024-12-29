use std::num::ParseIntError;

use api_response::prelude::*;
use error_code::{ModPath, ModSection, ModSegment};
use salvo::prelude::*;
use serde_json::{Value, json};

/// get user
#[cfg_attr(feature = "salvo", endpoint)]
#[cfg_attr(not(feature = "salvo"), handler)]
async fn get_user() -> Json<ApiResponse<Value, DefaultMeta>> {
    let user = json!({
        "id": 123,
        "name": "Andeya Lee",
        "email": "andeya.lee@example.com"
    });
    Json(user.api_response_with_meta(DefaultMeta::new().with_request_id("abc-123")))
}

const MS0: ModSection = ModSegment::M00.new_mod_section("module 0");
const MS1: ModSection = ModSection::new(ModSegment::M01, "module 01");
const MS2: ModSection = ModSection::new(ModSegment::M02, "module 012");
const MP: ModPath = ModPath::new(MS0, MS1, MS2);

/// get error
#[cfg_attr(feature = "salvo", endpoint)]
#[cfg_attr(not(feature = "salvo"), handler)]
async fn get_error() -> Json<ApiResponse<Value, ()>> {
    let err: ParseIntError = "@".parse::<u8>().unwrap_err();
    let details = [("email".to_string(), "Invalid email format".to_string())]
        .iter()
        .cloned()
        .collect();
    let error = ety_grpc::INVALID_ARGUMENT
        .new_api_error(MP)
        .with_details(details)
        .with_source(err, true);
    println!("error={:?}", error.downcast_ref::<ParseIntError>().unwrap());
    Json(Err(error).into())
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
