use api_response::prelude::*;

#[test]
fn error_json() {
    const ERROR: &str = if cfg!(feature = "lite") {
        r##"{"code":404,"error":{"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    } else {
        r##"{"status":"error","error":{"code":404,"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##
    };
    let mut api_response = ApiResponse::<(), _>::new_error(
        ApiError::new(404u32, "error message")
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
