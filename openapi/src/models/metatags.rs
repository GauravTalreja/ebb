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
pub struct Metatags {
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub en: Option<Box<crate::models::En1>>,
}

impl Metatags {
    pub fn new() -> Metatags {
        Metatags {
            en: None,
        }
    }
}


