/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Exam : Represents a scheduled Exam for a Class



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Exam {
    /// The name of the Exam, representative of the Course name
    #[serde(rename = "examDisplayName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exam_display_name: Option<Option<String>>,
    /// Sections of the Class this Exam schedule is applicable to
    #[serde(rename = "sections", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sections: Option<Option<String>>,
    /// Indicates whether this Exam is held online, or provides an alternate description
    #[serde(rename = "isOnlineDescription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_online_description: Option<Option<String>>,
    /// Day name on which this Exam is scheduled to take place
    #[serde(rename = "day", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub day: Option<Option<String>>,
    /// Description of the location of the Exam
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<String>>,
    /// The date the Exam is scheduled for
    #[serde(rename = "examWindowStartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exam_window_start_date: Option<Option<String>>,
    /// The time the Exam is scheduled to start
    #[serde(rename = "examWindowStartTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exam_window_start_time: Option<Option<String>>,
    /// The scheduled duration of the Exam
    #[serde(rename = "examDuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exam_duration: Option<Option<String>>,
    /// Additional notes about this Exam schedule
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Term Code for the Term this Exam is associated with
    #[serde(rename = "termCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub term_code: Option<Option<String>>,
}

impl Exam {
    /// Represents a scheduled Exam for a Class
    pub fn new() -> Exam {
        Exam {
            exam_display_name: None,
            sections: None,
            is_online_description: None,
            day: None,
            location: None,
            exam_window_start_date: None,
            exam_window_start_time: None,
            exam_duration: None,
            notes: None,
            term_code: None,
        }
    }
}


