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
pub struct TransactionInputSupportedParamsPartnerData {
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Box<crate::models::TransactionInputSupportedParamsPartnerDataRedirectUrl>,
}

impl TransactionInputSupportedParamsPartnerData {
    pub fn new(redirect_url: crate::models::TransactionInputSupportedParamsPartnerDataRedirectUrl) -> TransactionInputSupportedParamsPartnerData {
        TransactionInputSupportedParamsPartnerData {
            redirect_url: Box::new(redirect_url),
        }
    }
}

