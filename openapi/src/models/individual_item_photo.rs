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
pub struct IndividualItemPhoto {
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "filemime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filemime: Option<Option<String>>,
    #[serde(rename = "filesize", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filesize: Option<Option<String>>,
    #[serde(rename = "width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<String>>,
    #[serde(rename = "height", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<String>>,
    #[serde(rename = "alt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alt: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
}

impl IndividualItemPhoto {
    pub fn new() -> IndividualItemPhoto {
        IndividualItemPhoto {
            id: None,
            url: None,
            filemime: None,
            filesize: None,
            width: None,
            height: None,
            alt: None,
            title: None,
        }
    }
}


