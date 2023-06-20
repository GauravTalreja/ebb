/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaidHoliday : Data object representing a University of Waterloo paid holiday



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaidHoliday {
    /// Date of the paid holiday event
    #[serde(rename = "holidayDate", skip_serializing_if = "Option::is_none")]
    pub holiday_date: Option<String>,
    /// Name of the paid holiday
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
}

impl PaidHoliday {
    /// Data object representing a University of Waterloo paid holiday
    pub fn new() -> PaidHoliday {
        PaidHoliday {
            holiday_date: None,
            name: None,
        }
    }
}


