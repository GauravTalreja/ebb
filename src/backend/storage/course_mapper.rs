use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::JsonValue;

use crate::models::{AcademicLevel, Course, CourseOffering, Prerequisites};

impl<'r> FromRow<'r, PgRow> for Course {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Course {
            title: row.try_get("title")?,
            description: row.try_get("description").unwrap_or(None),
            external_id: row.try_get("external_id")?,
            subject_code: row.try_get("subject_code")?,
            catalog_number: row.try_get::<i16, &str>("catalog_number")? as u16,
            level: row
                .try_get::<String, &str>("academic_level")
                .map(AcademicLevel::from)
                .ok(),
            offerings: serde_json::from_str(&row.try_get::<String, &str>("offerings")?).unwrap(),
            prerequisites: match (
                row.try_get::<Vec<String>, &str>("optional_prerequisites"),
                row.try_get::<Vec<String>, &str>("required_prerequisites"),
            ) {
                (Ok(optional_courses), Ok(required_courses)) => Some(Prerequisites {
                    optional_courses,
                    required_courses,
                }),
                _ => None,
            },
        })
    }
}

impl From<JsonValue> for CourseOffering {
    fn from(offering_json: JsonValue) -> Self {
        serde_json::from_value(offering_json).unwrap()
    }
}
