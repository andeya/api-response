use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de,
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
};

use crate::{ApiError, ApiResponse, ErrorResponse, SuccessResponse, utils::OrderedHashMap};

impl<Data, Meta> Serialize for ApiResponse<Data, Meta>
where
    Data: Serialize,
    Meta: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct __ApiError<'a>(&'a ApiError);
        impl Serialize for __ApiError<'_> {
            fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
            where
                __S: Serializer,
            {
                #![allow(clippy::arithmetic_side_effects)]
                let mut __serde_state = Serializer::serialize_struct(
                    __serializer,
                    "__ApiError",
                    usize::from(false) + 1 + 1 + if Option::is_none(&self.0.details) { 0 } else { 1 },
                )?;
                SerializeStruct::serialize_field(&mut __serde_state, "message", &self.0.message)?;
                if !Option::is_none(&self.0.details) {
                    SerializeStruct::serialize_field(&mut __serde_state, "details", &self.0.details)?;
                } else {
                    SerializeStruct::skip_field(&mut __serde_state, "details")?;
                }
                SerializeStruct::end(__serde_state)
            }
        }

        match *self {
            ApiResponse::Success(ref field0) => {
                if let Some(meta) = &field0.meta {
                    let mut state = serializer.serialize_struct("ApiResponse", 3)?;
                    state.serialize_field("code", &0u32)?;
                    state.serialize_field("data", &field0.data)?;
                    state.serialize_field("meta", meta)?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("ApiResponse", 2)?;
                    state.serialize_field("code", &0u32)?;
                    state.serialize_field("data", &field0.data)?;
                    state.end()
                }
            }
            ApiResponse::Error(ref field0) => {
                if let Some(meta) = &field0.meta {
                    let mut state = serializer.serialize_struct("ApiResponse", 3)?;
                    state.serialize_field("code", &field0.code())?;
                    state.serialize_field("error", &__ApiError(&field0.error))?;
                    state.serialize_field("meta", meta)?;
                    state.end()
                } else {
                    let mut state = serializer.serialize_struct("ApiResponse", 2)?;
                    state.serialize_field("code", &field0.code())?;
                    state.serialize_field("error", &__ApiError(&field0.error))?;
                    state.end()
                }
            }
        }
    }
}

#[cfg_attr(feature = "salvo", derive(salvo::prelude::ToSchema))]
#[derive(Deserialize)]
pub(crate) struct __ApiError {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<OrderedHashMap<String, String>>,
}

#[allow(clippy::shadow_reuse, clippy::default_trait_access)]
impl<'de, Data, Meta> Deserialize<'de> for ApiResponse<Data, Meta>
where
    Data: Deserialize<'de>,
    Meta: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Code,
            Data,
            Meta,
            Error,
        }

        struct ApiResponseVisitor<Data, Meta> {
            marker: std::marker::PhantomData<fn() -> ApiResponse<Data, Meta>>,
        }

        impl<'de, Data, Meta> Visitor<'de> for ApiResponseVisitor<Data, Meta>
        where
            Data: Deserialize<'de>,
            Meta: Deserialize<'de>,
        {
            type Value = ApiResponse<Data, Meta>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ApiResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut data = None;
                let mut meta = None;
                let mut error = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        Field::Meta => {
                            if meta.is_some() {
                                return Err(de::Error::duplicate_field("meta"));
                            }
                            meta = Some(map.next_value()?);
                        }
                        Field::Error => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                    }
                }

                let code: u32 = code.unwrap_or_default();
                // let code: u32 = code.ok_or_else(|| de::Error::missing_field("code"))?;
                if code == 0 {
                    let data: Data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                    Ok(ApiResponse::Success(SuccessResponse { data, meta }))
                } else {
                    let error: __ApiError = error.ok_or_else(|| de::Error::missing_field("error"))?;
                    Ok(ApiResponse::Error(ErrorResponse {
                        error: ApiError {
                            code,
                            message: error.message,
                            details: error.details,
                            source: Default::default(),
                        },
                        meta,
                    }))
                }
            }
        }

        const FIELDS: &[&str] = &["code", "data", "meta", "error"];

        deserializer.deserialize_struct("ApiResponse", FIELDS, ApiResponseVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
