/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Subject : An academic Subject at Waterloo describes an area that a Course can be in



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subject {
    /// Code that identifies this Subject
    #[serde(rename = "code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub code: Option<Option<String>>,
    /// The Name for this Subject, most often the displayed name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// The short description of this subject, often same as Code
    #[serde(rename = "descriptionAbbreviated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description_abbreviated: Option<Option<String>>,
    /// Description of the Subject
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Code for the Academic Organization that is associated to this Subject
    #[serde(rename = "associatedAcademicOrgCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub associated_academic_org_code: Option<Option<String>>,
}

impl Subject {
    /// An academic Subject at Waterloo describes an area that a Course can be in
    pub fn new() -> Subject {
        Subject {
            code: None,
            name: None,
            description_abbreviated: None,
            description: None,
            associated_academic_org_code: None,
        }
    }
}


