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
