use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgRow;
use sqlx::types::JsonValue;
use sqlx::{Error, FromRow, PgPool, Postgres, Row};

use crate::models::{AcademicLevel, Course, CourseOffering, Prerequisites};

impl<'r> FromRow<'r, PgRow> for Course {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(Course {
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            subject_code: row.try_get("subject_code")?,
            catalog_number: row.try_get::<i16, &str>("catalog_number")? as u16,
            level: AcademicLevel::from(row.try_get::<String, &str>("academic_level")?),
            offerings: serde_json::from_str(&row.try_get::<String, &str>("offerings")?).unwrap(),
            prerequisites: Prerequisites {
                optional_courses: row.try_get::<Vec<String>, &str>("optional_prerequisites")?,
                required_courses: row.try_get::<Vec<String>, &str>("required_prerequisites")?,
            },
        })
    }
}

impl From<JsonValue> for CourseOffering {
    fn from(offering_json: JsonValue) -> Self {
        serde_json::from_value(offering_json).unwrap()
    }
}
