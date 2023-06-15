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
pub struct Published {
    #[serde(rename = "hid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hid: Option<Option<String>>,
    #[serde(rename = "vid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vid: Option<Option<String>>,
    #[serde(rename = "nid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nid: Option<Option<String>>,
    #[serde(rename = "from_state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_state: Option<Option<String>>,
    #[serde(rename = "state", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    #[serde(rename = "uid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uid: Option<Option<String>>,
    #[serde(rename = "stamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stamp: Option<Option<String>>,
    #[serde(rename = "published", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub published: Option<Option<String>>,
    #[serde(rename = "is_current", skip_serializing_if = "Option::is_none")]
    pub is_current: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Option<String>>,
}

impl Published {
    pub fn new() -> Published {
        Published {
            hid: None,
            vid: None,
            nid: None,
            from_state: None,
            state: None,
            uid: None,
            stamp: None,
            published: None,
            is_current: None,
            title: None,
            timestamp: None,
        }
    }
}


