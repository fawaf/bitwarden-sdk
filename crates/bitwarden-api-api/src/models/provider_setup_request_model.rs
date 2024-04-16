/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderSetupRequestModel {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "businessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "billingEmail")]
    pub billing_email: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "key")]
    pub key: String,
}

impl ProviderSetupRequestModel {
    pub fn new(
        name: String,
        billing_email: String,
        token: String,
        key: String,
    ) -> ProviderSetupRequestModel {
        ProviderSetupRequestModel {
            name,
            business_name: None,
            billing_email,
            token,
            key,
        }
    }
}
