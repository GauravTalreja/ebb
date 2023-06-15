/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkbenchModeration {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<crate::models::Current>>,
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<Box<crate::models::Published>>,
    #[serde(rename = "my_revision", skip_serializing_if = "Option::is_none")]
    pub my_revision: Option<Box<crate::models::MyRevision>>,
}

impl WorkbenchModeration {
    pub fn new() -> WorkbenchModeration {
        WorkbenchModeration {
            current: None,
            published: None,
            my_revision: None,
        }
    }
}

