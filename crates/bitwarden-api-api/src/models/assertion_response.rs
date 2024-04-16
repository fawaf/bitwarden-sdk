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
pub struct AssertionResponse {
    #[serde(rename = "authenticatorData", skip_serializing_if = "Option::is_none")]
    pub authenticator_data: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
    #[serde(rename = "userHandle", skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
}

impl AssertionResponse {
    pub fn new() -> AssertionResponse {
        AssertionResponse {
            authenticator_data: None,
            signature: None,
            client_data_json: None,
            user_handle: None,
        }
    }
}
