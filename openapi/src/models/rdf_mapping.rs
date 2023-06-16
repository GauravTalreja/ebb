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
pub struct RdfMapping {
    #[serde(rename = "rdftype", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rdftype: Option<Option<Vec<String>>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<crate::models::Title>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<Box<crate::models::Created>>,
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<Box<crate::models::Changed>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<crate::models::Body>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<Box<crate::models::Uid>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<crate::models::Name>>,
    #[serde(rename = "comment_count", skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<Box<crate::models::CommentCount>>,
    #[serde(rename = "last_activity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<Box<crate::models::LastActivity>>,
}

impl RdfMapping {
    pub fn new() -> RdfMapping {
        RdfMapping {
            rdftype: None,
            title: None,
            created: None,
            changed: None,
            body: None,
            uid: None,
            name: None,
            comment_count: None,
            last_activity: None,
        }
    }
}


