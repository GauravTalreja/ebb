use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CourseSummary {
    pub id: i32,
    pub catalog_number: i16,
    pub subject_code: String,
    pub title: String,
    pub external_id: String,
    pub offerings: OfferingSummaries,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OfferingSummary {
    pub year: i16,
    pub term: String,
    pub max_enrollment: i16,
    pub current_enrollment: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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