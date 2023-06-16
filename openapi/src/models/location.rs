/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Location : Model representing a Location for the Buildings dataset



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    /// Unique identifier for this building
    #[serde(rename = "buildingId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub building_id: Option<Option<String>>,
    /// Alpha-numeric building code
    #[serde(rename = "buildingCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub building_code: Option<Option<String>>,
    /// Unofficial, alpha-numeric building code that represents the parent location
    #[serde(rename = "parentBuildingCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_building_code: Option<Option<String>>,
    /// Display name of the building
    #[serde(rename = "buildingName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub building_name: Option<Option<String>>,
    /// Unofficial, collection of alternate building display names
    #[serde(rename = "alternateBuildingNames", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alternate_building_names: Option<Option<Vec<String>>>,
    /// Latitude co-ordinate of this location
    #[serde(rename = "latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f64>>,
    /// Longitude co-ordinate of this location
    #[serde(rename = "longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f64>>,
}

impl Location {
    /// Model representing a Location for the Buildings dataset
    pub fn new() -> Location {
        Location {
            building_id: None,
            building_code: None,
            parent_building_code: None,
            building_name: None,
            alternate_building_names: None,
            latitude: None,
            longitude: None,
        }
    }
}


