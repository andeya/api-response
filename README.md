# API Response Library

This library provides a consistent structure for API responses, including success and error handling.

[![GitHub last commit](https://img.shields.io/github/last-commit/andeya/api-response)](https://github.com/andeya/api-response/commits/main)
[![Crates.io](https://img.shields.io/crates/v/api-response.svg)](https://crates.io/crates/api-response)
[![Docs](https://docs.rs/api-response/badge.svg)](https://docs.rs/api-response)

## Features

-   Structured and unified API response format.
-   Includes meta for both success and error responses.
-   Support segmented error status codes.
-   Supports flexible serialization formats like JSON and Protobuf.
-   Integration with the Salvo framework for HTTP handling (see examples).

## Usage

Run the following Cargo command in your project directory:

```sh
cargo add api-response
```

Or add the following line to your Cargo.toml:

```toml
api-response = { version = "0.10", features = ["try"] }
```

## Format

-   Success example:

    ```json
    {
        "status": "success",
        "data": "success data",
        "meta": {
            "requestId": "request_id",
            "links": {
                "selfLink": "http:://andeya.example.com/b",
                "next": "http:://andeya.example.com/c",
                "prev": "http:://andeya.example.com/a"
            },
            "custom": {
                "key": "value"
            }
        }
    }
    ```

-   Error example

    ```json
    {
        "status": "error",
        "error": {
            "code": 404,
            "message": "error message",
            "details": {
                "key": "value"
            }
        },
        "meta": {
            "requestId": "request_id",
            "links": {
                "selfLink": "http:://andeya.example.com/b",
                "next": "http:://andeya.example.com/c",
                "prev": "http:://andeya.example.com/a"
            },
            "custom": {
                "key": "value"
            }
        }
    }
    ```

## Example

### Example of data construction.

```rust
use api_response::prelude::*;

#[test]
fn success_json() {
    const SUCCESS: &str = r##"{"status":"success","data":"success data","meta":{"requestId":"request_id","links":{"selfLink":"http:://andeya.example.com/b","next":"http:://andeya.example.com/c","prev":"http:://andeya.example.com/a"},"custom":{"key":"value"}}}"##;
    let api_response = ApiResponse::new_success(
        "success data",
        DefaultMeta::new("request_id")
            .with_links_info(
                "http:://andeya.example.com/b",
                "http:://andeya.example.com/c",
                "http:://andeya.example.com/a",
            )
            .insert_custom("key", "value"),
    );
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap());
    let s = serde_json::to_string(&api_response).unwrap();
    assert_eq!(SUCCESS, s);
}

#[test]
fn error_json() {
    const ERROR: &str = r##"{"status":"error","error":{"code":404,"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","links":{"selfLink":"http:://andeya.example.com/b","next":"http:://andeya.example.com/c","prev":"http:://andeya.example.com/a"},"custom":{"key":"value"}}}"##;
    let api_response = ApiResponse::<(), _>::new_error(
        ApiError::new(404, "error message")
            .with_detail("key", "value")
            .with_source("@".parse::<u8>().unwrap_err(), true),
        DefaultMeta::new("request_id")
            .with_links_info(
                "http:://andeya.example.com/b",
                "http:://andeya.example.com/c",
                "http:://andeya.example.com/a",
            )
            .insert_custom("key", "value"),
    );
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap());
    let e = serde_json::to_string(&api_response).unwrap();
    assert_eq!(ERROR, e);
}
```

### Example of server

```rust
use std::num::ParseIntError;

use api_response::{ApiResponse, DefaultMeta, ApiError};
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
    Json((user, DefaultMeta::new("abc-123")).into())
}

/// get error
#[cfg_attr(feature = "salvo", endpoint)]
#[cfg_attr(not(feature = "salvo"), handler)]
async fn get_error() -> Json<ApiResponse<Value, ()>> {
    let err: ParseIntError = "@".parse::<u8>().unwrap_err();
    let details = [("email".to_string(), "Invalid email format".to_string())].iter().cloned().collect();
    let error = ApiError::new(400, "Invalid input data").with_details(details).with_source(err, true);
    println!("error={:?}", error.downcast_ref::<ParseIntError>().unwrap());
    Json(Err(error).into())
}

#[tokio::main]
async fn main() {
    #[allow(unused_mut)]
    let mut router = Router::new().get(get_user).push(Router::with_path("error").get(get_error));
    #[cfg(feature = "salvo")]
    {
        let doc = OpenApi::new("API-Response", "1").merge_router(&router);
        router = router
            .push(doc.into_router("/api-doc/openapi.json"))
            .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));
    }
    Server::new(TcpListener::new("127.0.0.1:7878").bind().await).serve(router).await;
}
```

-   Get User: http://localhost:7878/
-   Get Error: http://localhost:7878/error
-   If the `salvo` feature is enabled, api-doc can be accessed: http://localhost:7878/swagger-ui
