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
pub struct IndividualItem {
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(rename = "photo", skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<crate::models::IndividualItemPhoto>>,
    #[serde(
        rename = "price",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Option<String>>,
    #[serde(
        rename = "calories",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calories: Option<Option<serde_json::Value>>,
}

impl IndividualItem {
    pub fn new() -> IndividualItem {
        IndividualItem {
            name: None,
            photo: None,
            price: None,
            calories: None,
        }
    }
}
