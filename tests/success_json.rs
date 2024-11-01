use api_response::prelude::*;

#[test]
fn success_json() {
    const SUCCESS: &str = r##"{"status":"success","data":"success data","meta":{"requestId":"request_id","links":{"selfLink":"http:://andeya.example.com/b","next":"http:://andeya.example.com/c","prev":"http:://andeya.example.com/a"},"custom":{"key":"value"}}}"##;
    let api_response = ApiResponse::from_success(
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
