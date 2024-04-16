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
pub struct RegisterRequestModel {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "masterPasswordHash")]
    pub master_password_hash: String,
    #[serde(rename = "masterPasswordHint", skip_serializing_if = "Option::is_none")]
    pub master_password_hint: Option<String>,
    #[serde(rename = "captchaResponse", skip_serializing_if = "Option::is_none")]
    pub captcha_response: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Box<crate::models::KeysRequestModel>>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "organizationUserId", skip_serializing_if = "Option::is_none")]
    pub organization_user_id: Option<uuid::Uuid>,
    #[serde(rename = "kdf", skip_serializing_if = "Option::is_none")]
    pub kdf: Option<crate::models::KdfType>,
    #[serde(rename = "kdfIterations", skip_serializing_if = "Option::is_none")]
    pub kdf_iterations: Option<i32>,
    #[serde(rename = "kdfMemory", skip_serializing_if = "Option::is_none")]
    pub kdf_memory: Option<i32>,
    #[serde(rename = "kdfParallelism", skip_serializing_if = "Option::is_none")]
    pub kdf_parallelism: Option<i32>,
    #[serde(rename = "referenceData", skip_serializing_if = "Option::is_none")]
    pub reference_data: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl RegisterRequestModel {
    pub fn new(email: String, master_password_hash: String) -> RegisterRequestModel {
        RegisterRequestModel {
            name: None,
            email,
            master_password_hash,
            master_password_hint: None,
            captcha_response: None,
            key: None,
            keys: None,
            token: None,
            organization_user_id: None,
            kdf: None,
            kdf_iterations: None,
            kdf_memory: None,
            kdf_parallelism: None,
            reference_data: None,
        }
    }
}
