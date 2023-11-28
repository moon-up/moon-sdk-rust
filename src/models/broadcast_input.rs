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
pub struct BroadcastInput {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "rawTransaction")]
    pub raw_transaction: String,
}

impl BroadcastInput {
    pub fn new(chain_id: String, raw_transaction: String) -> BroadcastInput {
        BroadcastInput {
            chain_id,
            raw_transaction,
        }
    }
}


