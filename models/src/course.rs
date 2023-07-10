use serde::{Deserialize, Serialize};

// struct for course_search page table display
// #[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// pub struct CourseSummaryDisplay {
//     pub catalog_number: String,
//     pub subject_code: String,
//     pub title: String,
//     pub location: String,
//     pub status: String/bool,
// }

// struct for course_detail course intro display
// #[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// pub struct CourseIntro {
//     pub catalog_number: String,
//     pub subject_code: String,
//     pub title: String,
//     pub course_description: String,
//     pub prerequisite_description: String,
// }

// struct for course_detail section sehcdule display
// #[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// pub struct CourseSchedule {
//     pub section: String, // LEC001, LEC002, TUT002 etc
//     pub class_number: String,
//     pub current_enroll: i32,
//     pub max_enroll: i32,
//     pub start_time: String/Time,
//     pub end_time: String/Time,
//     pub date: String, // "MTWTF"
//     pub location: String, // building + room number
//     pub instructor: String,
// }


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
    pub term: String
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct OfferingSummaries(Vec<OfferingSummary>);

#[cfg(feature = "sqlx")]
use sqlx::{Database, Postgres, Type};

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
