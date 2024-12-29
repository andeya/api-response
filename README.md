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

## Unified API Response Structure

### 1. Well-defined Structure

#### Top-Level Fields

| **Field Name** | **Type & Example**       | **Required**                  | **Meaning**           | **Description**                                               |
| -------------- | ------------------------ | ----------------------------- | --------------------- | ------------------------------------------------------------- |
| status         | `"success"` or `"error"` | Yes                           | Status of the request | Indicates whether the request was successful.                 |
| data           | `Any`                    | Required in success responses | Response data         | The data returned when the request is successful.             |
| error          | `ApiError` object        | Required in error responses   | Error information     | The error information object returned when the request fails. |
| meta           | `DefaultMeta` object     | No                            | Metadata information  | Metadata about the request.                                   |

#### `ApiError` Object Fields

| **Field Name** | **Type & Example**   | **Required** | **Meaning**   | **Description**                                                                                                                      |
| -------------- | -------------------- | ------------ | ------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| code           | `404`                | Yes          | Error code    | A code that identifies the type of error.                                                                                            |
| message        | `"error message"`    | Yes          | Error message | Text description of the error.                                                                                                       |
| details        | `{ "key": "value" }` | No           | Error details | The field is of the `map<string, string>` type and can be used to pass the front-end display configuration, error details and so on. |

#### `DefaultMeta` Object Fields

| **Meta Field** | **Type & Example**                                                                                           | **Required** | **Meaning**                                          | **Description**                                                                                                                                                |
| -------------- | ------------------------------------------------------------------------------------------------------------ | ------------ | ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `requestId`    | `"abc4567890"`                                                                                               | No           | Request tracking information                         | Included in the Response Body to maintain consistent data structure in non-standard protocols such as RPC, MQ, etc.                                            |
| `user`         | `{ "id": "user-123", "roles": ["admin", "editor"] }`                                                         | No           | The permission information of the current user, etc. | Notifies the client about the permissions the current user has for this request.                                                                               |
| `pagination`   | `{ "currentPage": 1, "pageSize": 10, "totalPages": 5, "totalRecords": 50, "nextPage": 2, "prevPage": null }` | No           | Pagination information                               | Helps clients with pagination navigation, displaying, and retrieving more data.                                                                                |
| `rateLimit`    | `{ "limit": 1000, "remaining": 990, "restoreRate": 50, "resetAt": "2021-01-01T00:00:00Z" }`                  | No           | Rate limiting information                            | Includes the limit, remaining calls, restore rate, and reset time, helping clients manage API call frequencies to avoid rate limit issues.                     |
| `cost`         | `{ "actualCost": 10, "requestedQueryCost": 10, "executionTime": "250ms" }`                                   | No           | Cost statistics                                      | Provides the cost statistics of the request operation, helping clients understand API resource consumption.                                                    |
| `apiVersion`   | `"v1.0.1"`                                                                                                   | No           | Current API version information                      | Ensures the API version consistency between client and server, beneficial for compatibility management, suitable for internal use or frequently iterated APIs. |

#### Well-defined JSON Examples

**Success Response Example:**

```json
{
    "status": "success",
    "data": "success data",
    "meta": {
        "requestId": "abc4567890",
        "user": {
            "id": "user-123",
            "roles": ["admin", "editor"]
        },
        "pagination": {
            "currentPage": 1,
            "pageSize": 10,
            "totalPages": 5,
            "totalRecords": 50,
            "nextPage": 2,
            "prevPage": null
        },
        "rateLimit": {
            "limit": 1000,
            "remaining": 990,
            "restoreRate": 50,
            "resetAt": "2021-01-01T00:00:00Z"
        },
        "cost": {
            "actualCost": 10,
            "requestedQueryCost": 10,
            "executionTime": "250ms"
        },
        "apiVersion": "v1.0.1"
    }
}
```

**Error Response Example:**

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
        "requestId": "abc4567890",
        "user": {
            "id": "user-123",
            "roles": ["admin", "editor"]
        },
        "pagination": {
            "currentPage": 1,
            "pageSize": 10,
            "totalPages": 5,
            "totalRecords": 50,
            "nextPage": 2,
            "prevPage": null
        },
        "rateLimit": {
            "limit": 1000,
            "remaining": 990,
            "restoreRate": 50,
            "resetAt": "2021-01-01T00:00:00Z"
        },
        "cost": {
            "actualCost": 10,
            "requestedQueryCost": 10,
            "executionTime": "250ms"
        },
        "apiVersion": "v1.0.1"
    }
}
```

### 2. Lightly-defined Structure

Enable the `lite` feature to use the lightly-defined structure.

```toml
api-response = { version = ">=0.13.0", features = ["lite"] }
```

The difference between the lightly-defined structure and the well-defined structure is that the `status` field is replaced by the `error.code` field, and when the "code" equals 0, it indicates a successful response.

#### Lightly-defined JSON Examples

**Success Response Example:**

```json
{
    "code": 0,
    "data": "success data",
    "meta": {
        "requestId": "abc4567890",
        "user": {
            "id": "user-123",
            "roles": ["admin", "editor"]
        },
        "pagination": {
            "currentPage": 1,
            "pageSize": 10,
            "totalPages": 5,
            "totalRecords": 50,
            "nextPage": 2,
            "prevPage": null
        },
        "rateLimit": {
            "limit": 1000,
            "remaining": 990,
            "restoreRate": 50,
            "resetAt": "2021-01-01T00:00:00Z"
        },
        "cost": {
            "actualCost": 10,
            "requestedQueryCost": 10,
            "executionTime": "250ms"
        },
        "apiVersion": "v1.0.1"
    }
}
```

**Error Response Example:**

```json
{
    "code": 404,
    "error": {
        "message": "error message",
        "details": {
            "key": "value"
        }
    },
    "meta": {
        "requestId": "abc4567890",
        "user": {
            "id": "user-123",
            "roles": ["admin", "editor"]
        },
        "pagination": {
            "currentPage": 1,
            "pageSize": 10,
            "totalPages": 5,
            "totalRecords": 50,
            "nextPage": 2,
            "prevPage": null
        },
        "rateLimit": {
            "limit": 1000,
            "remaining": 990,
            "restoreRate": 50,
            "resetAt": "2021-01-01T00:00:00Z"
        },
        "cost": {
            "actualCost": 10,
            "requestedQueryCost": 10,
            "executionTime": "250ms"
        },
        "apiVersion": "v1.0.1"
    }
}
```

## Example

### Example of data construction.

```rust
use api_response::prelude::*;

#[test]
fn success_json() {
    const SUCCESS: &str = if cfg!(feature = "lite") {
        r##"{"code":0,"data":"success data","meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    } else {
        r##"{"status":"success","data":"success data","meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    };
    let mut api_response = ApiResponse::new_success(
        "success data",
        DefaultMeta::new()
            .with_request_id("request_id")
            .with_pagination(Some(Pagination::default().with_current_page(1)))
            .insert_custom("key", "value"),
    );
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap());
    let s = serde_json::to_string(&api_response).unwrap();
    assert_eq!(SUCCESS, s);
    api_response = serde_json::from_str(SUCCESS).unwrap();
    let e = serde_json::to_string(&api_response).unwrap();
    assert_eq!(SUCCESS, e);
}

#[test]
fn error_json() {
    const ERROR: &str = if cfg!(feature = "lite") {
        r##"{"code":404,"error":{"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    } else {
        r##"{"status":"error","error":{"code":404,"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    };
    let mut api_response = ApiResponse::<(), _>::new_error(
        ApiError::new(404, "error message")
            .with_detail("key", "value")
            .with_source("@".parse::<u8>().unwrap_err(), true),
        DefaultMeta::new()
            .with_request_id("request_id")
            .with_pagination(Some(Pagination::default().with_current_page(1)))
            .insert_custom("key", "value"),
    );
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap());
    let e = serde_json::to_string(&api_response).unwrap();
    assert_eq!(ERROR, e);
    api_response = serde_json::from_str(ERROR).unwrap();
    let e = serde_json::to_string(&api_response).unwrap();
    assert_eq!(ERROR, e);
}
```

### Example of server

```rust
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
```

-   Get User: http://localhost:7878/
-   Get Error: http://localhost:7878/error
-   If the `salvo` feature is enabled, api-doc can be accessed: http://localhost:7878/swagger-ui
