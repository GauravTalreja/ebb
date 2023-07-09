use log::error;
use super::sample_db_models;
use stores::prelude::PgPool;
use openapi::models;
use super::util::extract_value_or_get_null_string;
use std::fs::{OpenOptions, File, create_dir_all};
use std::io::{BufWriter, Write};
use std::path::Path;

pub async fn get_courses_from_table(pool: &PgPool) -> Vec<sample_db_models::Course> {
    let courses_from_table: Result<Vec<sample_db_models::Course>, sqlx::Error> = 
        sqlx::query_as::<_, sample_db_models::Course>("SELECT * FROM courses")
        .fetch_all(pool)
        .await;

    match courses_from_table {
        Ok(courses) => courses,
        Err(err) => {
            error!("{:?}", err);
            Vec::<sample_db_models::Course>::new()
        }
    }
}

/// Maps list of courses to an insert query for the courses table.
pub fn map_courses_to_insert_query(courses: &Vec<models::Course>, file_path: &str) {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create intermediate directories");
    }

    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path).expect("Could not open file");

    let mut file: BufWriter<File> = BufWriter::new(file);

    let start_of_query: &str = "INSERT INTO courses (catalog_number, subject_code, external_id \
        , academic_level, title, description, requirements, enroll_consent, drop_consent, \
        prerequisites_id)\nVALUES\n";
    let fail_msg: &str = "Could not write insert query for courses table.";
    file.write_all(start_of_query.as_bytes()).expect(fail_msg);

    for (idx, course) in courses.iter().enumerate() {
        let mut val: String = map_course_to_query(course);
        // For the last tuple, we don't want a comma at the end.
        if idx == courses.len() - 1 { 
            val.pop(); // remove newline.
            val.pop(); // remove comma.
            val.push('\n'); // insert newline again.
        }
        file.write_all(val.as_bytes()).expect(fail_msg);
    }

    // For now, just set it as do nothing.
    let end_of_query: &str = "ON CONFLICT DO NOTHING;";
    file.write_all(end_of_query.as_bytes()).expect(fail_msg);
}

/// Maps course to tuple: (a1, a2, ..., an) that would come after the VALUES line
/// in an SQL insert stmt.
fn map_course_to_query(course: &models::Course) -> String {
    let course: models::Course = course.clone();

    // Assume these values are not null.
    let catalog_number: String = course.catalog_number.unwrap().unwrap();
    let subject_code: String = course.subject_code.unwrap().unwrap();
    let external_id: String = course.course_id.unwrap().unwrap();
    let academic_level: String = course.associated_academic_career.unwrap().unwrap();
    let title: String = course.title.unwrap().unwrap().replace("'", "''");
    let description: String = course.description.unwrap().unwrap().replace("'", "''");

    // Assume these values CAN be null. So either we get a value, or set as NULL string.
    let requirements: String = extract_value_or_get_null_string(course.requirements_description);
    let enroll_consent: String = extract_value_or_get_null_string(course.enroll_consent_description);
    let drop_consent: String = extract_value_or_get_null_string(course.drop_consent_description);
    let prerequisites_id: String = String::from("NULL");

    format!("('{catalog_number}', '{subject_code}', '{external_id}', '{academic_level}', '{title}', \
        '{description}', {requirements}, {enroll_consent}, {drop_consent}, {prerequisites_id}),\n")
}