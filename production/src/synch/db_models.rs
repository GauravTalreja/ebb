use openapi::models;
use sqlx::FromRow;

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
    pub prerequisites_id: Option<i32>,
}

#[derive(Debug)]
pub struct InsertCourse {
    pub catalog_number: String,
    pub subject_code: String,
    pub external_id: String,
    pub academic_level: String,
    pub title: String,
    pub description: String,
    pub requirements: Option<String>,
    pub enroll_consent: Option<String>,
    pub drop_consent: Option<String>,
    pub prerequisites_id: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct CourseOffering {
    pub id: i32,
    pub course_id: i32,
    pub year: i16, // SMALLINTS map to i16 for Rust.
    pub term: String,
}

pub fn map_api_course_to_db_course(course: &models::Course) -> InsertCourse {
    // VALID DATA ASSUMPTION: These fields are not null.
    let catalog_number: String = course.catalog_number.clone().unwrap().unwrap();
    let subject_code: String = course.subject_code.clone().unwrap().unwrap();
    let external_id: String = course.course_id.clone().unwrap().unwrap();
    let academic_level: String = course.associated_academic_career.clone().unwrap().unwrap();
    let title: String = course.title.clone().unwrap().unwrap().replace("'", "''");
    let description: String = course
        .description
        .clone()
        .unwrap()
        .unwrap()
        .replace("'", "''");

    // Assume these values CAN be null.
    let requirements: Option<String> = course.requirements_description.clone().flatten();
    let enroll_consent: Option<String> = course.enroll_consent_description.clone().flatten();
    let drop_consent: Option<String> = course.drop_consent_description.clone().flatten();

    return InsertCourse {
        catalog_number,
        subject_code,
        external_id,
        academic_level,
        title,
        description,
        requirements,
        enroll_consent,
        drop_consent,
        prerequisites_id: None,
    };
}
