use api_response::{ApiResponse, DefaultMeta, ErrorInfo};
use salvo::prelude::*;
use serde_json::json;

#[handler]
async fn get_user(res: &mut Response) {
    let user = json!({
        "id": 123,
        "name": "John Doe",
        "email": "john.doe@example.com"
    });

    let response: ApiResponse<_, DefaultMeta, serde_json::Value> = Ok(user).into();

    res.render_json(&response);
}

#[handler]
async fn get_error(res: &mut Response) {
    let error = ErrorInfo::new(
        400,
        "Invalid input data",
        Some(json!({
            "email": "Invalid email format"
        })),
        None,
    );

    let response: ApiResponse<(), DefaultMeta, serde_json::Value> = Err(error).into();

    res.render_json(&response);
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get(get_user)
        .push(Router::with_path("error").get(get_error));

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}
