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
pub struct BalanceResponse {
    #[serde(rename = "balance")]
    pub balance: String,
}

impl BalanceResponse {
    pub fn new(balance: String) -> BalanceResponse {
        BalanceResponse {
            balance,
        }
    }
}


