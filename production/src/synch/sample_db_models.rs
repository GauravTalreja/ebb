use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Course {
    pub id: i32,
    pub catalog_number: String,
    pub subject_code: String,
    pub external_id: String,
    pub academic_level: String,
    pub title: String,
    pub description: String,
    pub requirements: Option<String>,
    pub enroll_consent: Option<String>,
    pub drop_consent: Option<String>,
    pub prerequisites_id: Option<i32>
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct CourseOffering {
    pub id: i32,
    pub course_id: i32,
    pub year: i16, // SMALLINTS map to i16 for Rust.
    pub term: String
}