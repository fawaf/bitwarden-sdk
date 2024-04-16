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
pub struct StorageRequestModel {
    #[serde(rename = "storageGbAdjustment")]
    pub storage_gb_adjustment: i32,
}

impl StorageRequestModel {
    pub fn new(storage_gb_adjustment: i32) -> StorageRequestModel {
        StorageRequestModel {
            storage_gb_adjustment,
        }
    }
}
