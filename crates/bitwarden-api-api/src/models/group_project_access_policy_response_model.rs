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
pub struct GroupProjectAccessPolicyResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "currentUserInGroup", skip_serializing_if = "Option::is_none")]
    pub current_user_in_group: Option<bool>,
    #[serde(rename = "grantedProjectId", skip_serializing_if = "Option::is_none")]
    pub granted_project_id: Option<uuid::Uuid>,
}

impl GroupProjectAccessPolicyResponseModel {
    pub fn new() -> GroupProjectAccessPolicyResponseModel {
        GroupProjectAccessPolicyResponseModel {
            object: None,
            id: None,
            read: None,
            write: None,
            creation_date: None,
            revision_date: None,
            group_id: None,
            group_name: None,
            current_user_in_group: None,
            granted_project_id: None,
        }
    }
}
