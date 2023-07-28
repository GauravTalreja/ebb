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
pub struct TitleField {
    #[serde(
        rename = "en",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub en: Option<Option<Vec<crate::models::En>>>,
}

impl TitleField {
    pub fn new() -> TitleField {
        TitleField { en: None }
    }
}
