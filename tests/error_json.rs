use api_response::prelude::*;

#[test]
fn error_json() {
    const ERROR: &str = r##"{"status":"error","error":{"code":404,"message":"error message","details":{"key":"value","source":"invalid digit found in string"}},"meta":{"requestId":"request_id","pagination":{"currentPage":1,"pageSize":0,"totalPages":0,"totalRecords":0,"nextPage":null,"prevPage":null},"custom":{"key":"value"}}}"##;
    let api_response = ApiResponse::<(), _>::new_error(
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
}
