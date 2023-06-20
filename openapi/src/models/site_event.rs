/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SiteEvent : Model representing a WCMS Event item



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SiteEvent {
    /// Unique, numeric site ID
    #[serde(rename = "siteId", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i32>,
    /// Unique Id of this evevnt item
    #[serde(rename = "uniqueKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<Option<String>>,
    /// Title of the event
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Start date of the event
    #[serde(rename = "eventStartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_start_date: Option<Option<String>>,
    /// End date of the event
    #[serde(rename = "eventEndDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_end_date: Option<Option<String>>,
    /// Description of the event recurrence rule
    #[serde(rename = "recurRule", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recur_rule: Option<Option<String>>,
    /// Time zone description of the event dates
    #[serde(rename = "eventTimezone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_timezone: Option<Option<String>>,
    /// Link to the Event
    #[serde(rename = "itemUri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub item_uri: Option<Option<String>>,
    /// Tag(s) representing the event
    #[serde(rename = "eventTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_tags: Option<Option<String>>,
    /// Type of the event
    #[serde(rename = "eventType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Option<String>>,
    /// Supplemental URI for the event
    #[serde(rename = "eventWebsite", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_website: Option<Option<String>>,
    /// Tag(s) representing the intended audience for the event
    #[serde(rename = "audience", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audience: Option<Option<String>>,
    /// Cost of the event
    #[serde(rename = "cost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cost: Option<Option<String>>,
    /// Event host information
    #[serde(rename = "host", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub host: Option<Option<String>>,
    /// Unique Id of the location hosting the event
    #[serde(rename = "locationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<Option<i32>>,
    /// Display name of the location hosting the event
    #[serde(rename = "locationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location_name: Option<Option<String>>,
    /// Date item last updated
    #[serde(rename = "updatedDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<Option<String>>,
    /// Description content for the event, often includes HTML markup
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
}

impl SiteEvent {
    /// Model representing a WCMS Event item
    pub fn new() -> SiteEvent {
        SiteEvent {
            site_id: None,
            unique_key: None,
            title: None,
            event_start_date: None,
            event_end_date: None,
            recur_rule: None,
            event_timezone: None,
            item_uri: None,
            event_tags: None,
            event_type: None,
            event_website: None,
            audience: None,
            cost: None,
            host: None,
            location_id: None,
            location_name: None,
            updated_date: None,
            content: None,
        }
    }
}


