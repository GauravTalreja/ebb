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
pub struct HoursChange {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "value2", skip_serializing_if = "Option::is_none")]
    pub value2: Option<String>,
    #[serde(rename = "timezone_db", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timezone_db: Option<Option<String>>,
    #[serde(rename = "date_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_type: Option<Option<String>>,
}

impl HoursChange {
    pub fn new() -> HoursChange {
        HoursChange {
            value: None,
            value2: None,
            timezone_db: None,
            date_type: None,
        }
    }
}

