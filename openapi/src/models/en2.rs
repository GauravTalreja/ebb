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
pub struct En2 {
    #[serde(rename = "entity_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<Option<String>>,
    #[serde(rename = "entity_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<Option<String>>,
    #[serde(rename = "revision_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<Option<String>>,
    #[serde(rename = "language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[serde(rename = "source", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source: Option<Option<String>>,
    #[serde(rename = "uid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uid: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    #[serde(rename = "translate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub translate: Option<Option<String>>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "changed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub changed: Option<Option<String>>,
}

impl En2 {
    pub fn new() -> En2 {
        En2 {
            entity_type: None,
            entity_id: None,
            revision_id: None,
            language: None,
            source: None,
            uid: None,
            status: None,
            translate: None,
            created: None,
            changed: None,
        }
    }
}


