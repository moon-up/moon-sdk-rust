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
pub struct InftApproval {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    #[serde(rename = "tokenContractType")]
    pub token_contract_type: String,
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "operator")]
    pub operator: String,
    #[serde(rename = "approvedAll")]
    pub approved_all: bool,
    #[serde(rename = "tokenId", deserialize_with = "Option::deserialize")]
    pub token_id: Option<String>,
}

impl InftApproval {
    pub fn new(transaction_hash: String, contract: String, log_index: String, token_contract_type: String, token_name: String, token_symbol: String, account: String, operator: String, approved_all: bool, token_id: Option<String>) -> InftApproval {
        InftApproval {
            transaction_hash,
            contract,
            log_index,
            token_contract_type,
            token_name,
            token_symbol,
            account,
            operator,
            approved_all,
            token_id,
        }
    }
}


