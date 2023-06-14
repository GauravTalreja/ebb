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
pub struct OpeningHour {
    #[serde(rename = "day", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub day: Option<Option<String>>,
    #[serde(rename = "starthours", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub starthours: Option<Option<String>>,
    #[serde(rename = "endhours", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub endhours: Option<Option<String>>,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
}

impl OpeningHour {
    pub fn new() -> OpeningHour {
        OpeningHour {
            day: None,
            starthours: None,
            endhours: None,
            comment: None,
        }
    }
}


