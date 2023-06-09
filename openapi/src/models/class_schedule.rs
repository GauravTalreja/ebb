/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClassSchedule : Data describing scheduling and availability data for a Class



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClassSchedule {
    /// Course identifier number, not unique
    #[serde(rename = "courseId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub course_id: Option<Option<String>>,
    /// Course offer number identifier for this class
    #[serde(rename = "courseOfferNumber", skip_serializing_if = "Option::is_none")]
    pub course_offer_number: Option<i32>,
    /// Session code number for this class
    #[serde(rename = "sessionCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub session_code: Option<Option<String>>,
    /// Number identifying the section of this class
    #[serde(rename = "classSection", skip_serializing_if = "Option::is_none")]
    pub class_section: Option<i32>,
    /// Waterloo academic term code
    #[serde(rename = "termCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub term_code: Option<Option<String>>,
    /// Identifier for the class meeting this schedule data relates to
    #[serde(rename = "classMeetingNumber", skip_serializing_if = "Option::is_none")]
    pub class_meeting_number: Option<i32>,
    /// The date this schedule begins
    #[serde(rename = "scheduleStartDate", skip_serializing_if = "Option::is_none")]
    pub schedule_start_date: Option<String>,
    /// The date this schedule ends
    #[serde(rename = "scheduleEndDate", skip_serializing_if = "Option::is_none")]
    pub schedule_end_date: Option<String>,
    /// The start time of this class
    #[serde(rename = "classMeetingStartTime", skip_serializing_if = "Option::is_none")]
    pub class_meeting_start_time: Option<String>,
    /// The end time of this class
    #[serde(rename = "classMeetingEndTime", skip_serializing_if = "Option::is_none")]
    pub class_meeting_end_time: Option<String>,
    /// A code representing the days the class schedule takes place during the week
    #[serde(rename = "classMeetingDayPatternCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub class_meeting_day_pattern_code: Option<Option<String>>,
    /// A Y|N per day representation of the class schedule taking place during the week
    #[serde(rename = "classMeetingWeekPatternCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub class_meeting_week_pattern_code: Option<Option<String>>,
    /// Building and room name for the location of this class schedule
    #[serde(rename = "locationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<Option<String>>,
}

impl ClassSchedule {
    /// Data describing scheduling and availability data for a Class
    pub fn new() -> ClassSchedule {
        ClassSchedule {
            course_id: None,
            course_offer_number: None,
            session_code: None,
            class_section: None,
            term_code: None,
            class_meeting_number: None,
            schedule_start_date: None,
            schedule_end_date: None,
            class_meeting_start_time: None,
            class_meeting_end_time: None,
            class_meeting_day_pattern_code: None,
            class_meeting_week_pattern_code: None,
            location_name: None,
        }
    }
}


