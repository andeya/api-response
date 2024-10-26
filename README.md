# API Response Library

This library provides a consistent structure for API responses, including success and error handling.

[![GitHub last commit](https://img.shields.io/github/last-commit/andeya/api-response)](https://github.com/andeya/api-response/commits/main)
[![Crates.io](https://img.shields.io/crates/v/api-response.svg)](https://crates.io/crates/api-response)
[![Docs](https://docs.rs/api-response/badge.svg)](https://docs.rs/api-response)

## Features

-   Structured and unified API response format.
-   Includes meta for both success and error responses.
-   Supports flexible serialization formats like JSON and Protobuf.
-   Integration with the Salvo framework for HTTP handling (see examples).

## Usage

Run the following Cargo command in your project directory:

```sh
cargo add api-response
```

Or add the following line to your Cargo.toml:

```toml
api-response = "0.6"
```

## Example

```rust
use std::num::ParseIntError;

use api_response::{ApiResponse, DefaultMeta, ErrorInfo};
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
    let error = ErrorInfo::new(400, "Invalid input data").with_details(details).with_source(err);
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
