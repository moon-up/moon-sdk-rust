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
pub struct Transaction {
    #[serde(rename = "moon_scan_url", skip_serializing_if = "Option::is_none")]
    pub moon_scan_url: Option<String>,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "signed_transaction")]
    pub signed_transaction: String,
    #[serde(rename = "signed_message", skip_serializing_if = "Option::is_none")]
    pub signed_message: Option<String>,
    #[serde(rename = "raw_transaction", skip_serializing_if = "Option::is_none")]
    pub raw_transaction: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<::std::collections::HashMap<String, crate::models::Tx>>,
    #[serde(rename = "userOps", skip_serializing_if = "Option::is_none")]
    pub user_ops: Option<Vec<crate::models::TransactionRequest>>,
    #[serde(rename = "userop_transaction", skip_serializing_if = "Option::is_none")]
    pub userop_transaction: Option<String>,
}

impl Transaction {
    pub fn new(transaction_hash: String, signed_transaction: String) -> Transaction {
        Transaction {
            moon_scan_url: None,
            transaction_hash,
            signed_transaction,
            signed_message: None,
            raw_transaction: None,
            signature: None,
            transaction: None,
            user_ops: None,
            userop_transaction: None,
        }
    }
}

