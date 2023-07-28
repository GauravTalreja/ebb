use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{Database, Postgres, Type};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LastUpdated {
    pub date_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct CourseSummary {
    pub id: i32,
    pub catalog_number: String,
    pub subject_code: String,
    pub title: String,
    pub external_id: String,
    pub offerings: OfferingSummaries,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct OfferingSummary {
    pub year: i16,
    pub term: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct OfferingSummaries(Vec<OfferingSummary>);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SubjectSummary {
    pub name: String,
    pub course_count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct CourseDetail {
    // TODO: Store CourseSummary as a field
    pub id: i32,
    pub catalog_number: String,
    pub subject_code: String,
    pub title: String,
    pub external_id: String,
    pub description: String,
    pub requirements_description: Option<String>,
    pub academic_level: String,
    pub optional_prerequisites: Vec<String>,
    pub required_prerequisites: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct OfferingDetail {
    // TODO: Store OfferingSummary as a field
    pub course_id: i32,
    pub course_catalog_number: String,
    pub course_subject_code: String,
    pub year: i16,
    pub term: String,
    pub schedules: ScheduleDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct ScheduleDetails(pub Vec<ClassSchedule>);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ClassSchedule {
    pub class_section: i16,
    pub class_number: i16,
    pub component: Option<String>,
    // TODO: Update to use Postgres time range type: https://www.postgresql.org/docs/current/rangetypes.html
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub instructor_name: Option<String>,
    pub location: Option<String>,
    pub max_enrollment: i16,
    pub current_enrollment: i16,
}

#[cfg(feature = "sqlx")]
impl Type<Postgres> for OfferingSummaries {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, Postgres> for OfferingSummaries {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        // decode it to JSON, and then convert it
        let as_json = serde_json::Value::decode(value)?;
        serde_json::from_value(as_json).map_err(sqlx::error::BoxDynError::from)
    }
}

#[cfg(feature = "sqlx")]
impl Type<Postgres> for ScheduleDetails {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, Postgres> for ScheduleDetails {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        // decode it to JSON, and then convert it
        let as_json = serde_json::Value::decode(value)?;
        serde_json::from_value(as_json).map_err(sqlx::error::BoxDynError::from)
    }
}
