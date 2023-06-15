/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Site : Model representing a Site on the Waterloo CMS



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Site {
    /// Unique, numeric site ID
    #[serde(rename = "siteId", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i32>,
    /// Relative URI of the site from the root uWaterloo CMS domain
    #[serde(rename = "relativeUri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub relative_uri: Option<Option<String>>,
    /// Display name of the Site
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// If available, the department owner code for this Site. Can loosely map to Academic Groups.
    #[serde(rename = "ownerUnitCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_unit_code: Option<Option<String>>,
    /// Short name of the department owner for this Site. See OwnerUnitCode for more.
    #[serde(rename = "ownerUnitNameShort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_unit_name_short: Option<Option<String>>,
    /// Full name of the department owner for this Site. See OwnerUnitCode for more.
    #[serde(rename = "ownerUnitNameFull", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_unit_name_full: Option<Option<String>>,
    /// If available, the faculty owner code for this Site. Can loosely map to Academic Organization.
    #[serde(rename = "ownerGroupCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_group_code: Option<Option<String>>,
    /// The tag describing the association of the owner department, such as academic, research, and more.
    #[serde(rename = "ownerType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<Option<String>>,
}

impl Site {
    /// Model representing a Site on the Waterloo CMS
    pub fn new() -> Site {
        Site {
            site_id: None,
            relative_uri: None,
            name: None,
            owner_unit_code: None,
            owner_unit_name_short: None,
            owner_unit_name_full: None,
            owner_group_code: None,
            owner_type: None,
        }
    }
}

