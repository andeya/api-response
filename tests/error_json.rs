use api_response::*;

#[test]
fn error_json() {
    const ERROR: &str = r##"{"status":"error","error":{"code":404,"message":"error message","details":{"key":"value"}},"meta":{"requestId":"request_id","links":{"selfLink":"http:://andeya.example.com/b","next":"http:://andeya.example.com/c","prev":"http:://andeya.example.com/a"},"custom":{"key":"value"}}}"##;
    let api_response = ApiResponse::<(), _>::from_error(
        ErrorInfo::new(404, "error message")
            .with_detail("key", "value")
            .with_source("@".parse::<u8>().unwrap_err()),
        DefaultMeta::new("request_id")
            .with_links_info(
                "http:://andeya.example.com/b",
                Some("http:://andeya.example.com/c"),
                Some("http:://andeya.example.com/a"),
            )
            .insert_custom("key", "value"),
    );
    println!("{}", serde_json::to_string_pretty(&api_response).unwrap());
    let e = serde_json::to_string(&api_response).unwrap();
    assert_eq!(ERROR, e);
}
