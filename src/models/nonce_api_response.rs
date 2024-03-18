/*
 * moon-vault-api
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NonceApiResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::NonceResponse>>,
}

impl NonceApiResponse {
    pub fn new(success: bool, message: String) -> NonceApiResponse {
        NonceApiResponse {
            success,
            message,
            data: None,
        }
    }
}


