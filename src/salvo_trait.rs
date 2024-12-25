use salvo::{
    Scribe, async_trait,
    oapi::{Components, Content, EndpointOutRegister, Operation, RefOr, Response, ToResponse, ToSchema},
    writing::Json,
};
use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResponse};

/// Struct to represent the overall API response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ApiResponseSchema<Data, Meta> {
    status: ApiStatus,
    data: Data,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
    error: ApiError,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
enum ApiStatus {
    Success,
    Error,
}

impl<Data, Meta> ToResponse for ApiResponse<Data, Meta>
where
    Data: ToSchema + 'static,
    Meta: ToSchema + 'static,
{
    fn to_response(components: &mut Components) -> RefOr<Response> {
        Response::new("Response with json format data")
            .add_content(
                "application/json",
                Content::new(ApiResponseSchema::<Data, Meta>::to_schema(components)),
            )
            .into()
    }
}

impl<Data, Meta> EndpointOutRegister for ApiResponse<Data, Meta>
where
    Data: ToSchema + 'static,
    Meta: ToSchema + 'static,
{
    #[inline]
    fn register(components: &mut Components, operation: &mut Operation) {
        operation.responses.insert("200", Self::to_response(components));
    }
}

#[async_trait]
impl<Data, Meta> Scribe for ApiResponse<Data, Meta>
where
    Data: Serialize + Send,
    Meta: Serialize + Send,
{
    fn render(self, res: &mut salvo::prelude::Response) {
        Json(self).render(res)
    }
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use serde_json::{Value, json};

    use crate::*;
    #[endpoint]
    fn get_user() -> ApiResponse<Value, DefaultMeta> {
        let user = json!({
            "id": 123,
            "name": "Andeya Lee",
            "email": "andeya.lee@example.com"
        });
        user.api_response_with_meta(DefaultMeta::new().with_request_id("abc-123"))
    }
}
