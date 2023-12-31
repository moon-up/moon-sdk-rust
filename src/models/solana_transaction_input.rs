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
pub struct SolanaTransactionInput {
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "compress", skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
}

impl SolanaTransactionInput {
    pub fn new() -> SolanaTransactionInput {
        SolanaTransactionInput {
            to: None,
            value: None,
            network: None,
            compress: None,
        }
    }
}


