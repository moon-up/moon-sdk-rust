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
pub struct SupportedAssetResponse {
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "assets")]
    pub assets: Vec<crate::models::SupportedAssetResponseAssetsInner>,
}

impl SupportedAssetResponse {
    pub fn new(country: String, assets: Vec<crate::models::SupportedAssetResponseAssetsInner>) -> SupportedAssetResponse {
        SupportedAssetResponse {
            country,
            assets,
        }
    }
}


