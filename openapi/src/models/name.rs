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
pub struct Name {
    #[serde(
        rename = "predicates",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub predicates: Option<Option<Vec<String>>>,
}

impl Name {
    pub fn new() -> Name {
        Name { predicates: None }
    }
}
