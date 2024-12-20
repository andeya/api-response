use api_response::prelude::*;

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
