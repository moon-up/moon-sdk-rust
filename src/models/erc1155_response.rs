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
pub struct Erc1155Response {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<f64>,
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<f64>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "gas", skip_serializing_if = "Option::is_none")]
    pub gas: Option<String>,
    #[serde(rename = "gas_price", skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(rename = "gas_tip_cap", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gas_tip_cap: Option<Option<String>>,
    #[serde(rename = "gas_fee_cap", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gas_fee_cap: Option<Option<String>>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<f64>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<String>>,
    #[serde(rename = "blob_gas", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blob_gas: Option<Option<String>>,
    #[serde(rename = "blob_gas_fee_cap", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blob_gas_fee_cap: Option<Option<String>>,
    #[serde(rename = "blob_hashes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blob_hashes: Option<Option<Vec<String>>>,
    #[serde(rename = "v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "balance_of", skip_serializing_if = "Option::is_none")]
    pub balance_of: Option<String>,
    #[serde(rename = "balance_of_batch", skip_serializing_if = "Option::is_none")]
    pub balance_of_batch: Option<String>,
}

impl Erc1155Response {
    pub fn new() -> Erc1155Response {
        Erc1155Response {
            r#type: None,
            chain_id: None,
            data: None,
            gas: None,
            gas_price: None,
            gas_tip_cap: None,
            gas_fee_cap: None,
            value: None,
            nonce: None,
            from: None,
            to: None,
            blob_gas: None,
            blob_gas_fee_cap: None,
            blob_hashes: None,
            v: None,
            r: None,
            s: None,
            balance_of: None,
            balance_of_batch: None,
        }
    }
}


