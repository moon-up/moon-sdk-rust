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
pub struct TransactionInputMetaData {
    #[serde(rename = "quoteId")]
    pub quote_id: String,
}

impl TransactionInputMetaData {
    pub fn new(quote_id: String) -> TransactionInputMetaData {
        TransactionInputMetaData {
            quote_id,
        }
    }
}


