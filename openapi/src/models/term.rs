/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 *
 * Generated by: https://openapi-generator.tech
 */

/// Term : An academic Term at Waterloo, a defined period of time that is used by classes and similar

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Term {
    /// Code that describes this Term
    #[serde(
        rename = "termCode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub term_code: Option<Option<String>>,
    /// The Name for this Term, most often the displayed name
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// The short form name for this Term
    #[serde(
        rename = "nameShort",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name_short: Option<Option<String>>,
    /// The date and time from which the Term begins, inclusive
    #[serde(rename = "termBeginDate", skip_serializing_if = "Option::is_none")]
    pub term_begin_date: Option<String>,
    /// The date and time on which the Term ends, inclusive
    #[serde(rename = "termEndDate", skip_serializing_if = "Option::is_none")]
    pub term_end_date: Option<String>,
    /// The date and time at which the term is 60% complete, used for standing, withdrawal, and penalties
    #[serde(
        rename = "sixtyPercentCompleteDate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sixty_percent_complete_date: Option<Option<String>>,
    /// The academic year to which this Term belongs
    #[serde(
        rename = "associatedAcademicYear",
        skip_serializing_if = "Option::is_none"
    )]
    pub associated_academic_year: Option<i32>,
}

impl Term {
    /// An academic Term at Waterloo, a defined period of time that is used by classes and similar
    pub fn new() -> Term {
        Term {
            term_code: None,
            name: None,
            name_short: None,
            term_begin_date: None,
            term_end_date: None,
            sixty_percent_complete_date: None,
            associated_academic_year: None,
        }
    }
}
