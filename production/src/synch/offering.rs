use openapi::models;
use openapi::apis::configuration::Configuration;
use openapi::apis::class_schedules_api::v3_class_schedules_term_code_subject_catalog_number_get as get_class_schdule;
use openapi::apis::class_schedules_api::V3ClassSchedulesTermCodeSubjectCatalogNumberGetError;
use openapi::apis::Error;
use super::sample_db_models;
use tokio::task;
use std::fs::{OpenOptions, File, create_dir_all};
use std::path::Path;
use std::io::{BufWriter, Write};
use log::error;
use stores::prelude::PgPool;

pub async fn get_class_schedules_for_courses(
    config: &Configuration,
    term_code: &str,
    courses: &Vec<sample_db_models::Course>
) -> Vec<(i32, Vec<models::Class>)> {
    
    let mut schedule_data = Vec::<(i32, Vec<models::Class>)>::new();

    // Make concurrent API calls for efficiency.
    let task_results = courses.into_iter().map(|course| {
        let config: Configuration = config.clone();
        let term_code: String = term_code.to_string();
        let id: i32 = course.id.clone();
        let subject: String = course.subject_code.clone();
        let catalog_number: String = course.catalog_number.clone();

        task::spawn(async move {

            let result: Result<
                Vec<models::Class>, 
                Error<V3ClassSchedulesTermCodeSubjectCatalogNumberGetError>
            > = get_class_schdule(
                &config, 
                term_code.as_str(), 
                subject.as_str(), 
                catalog_number.as_str()).await;
            (id, result)
        })
    });

    for task in task_results {
        let (id, result) = task.await.unwrap();
        match result {
            Ok(classes) => {
                schedule_data.push((id, classes));
            }
            Err(_) => (),
        }
    }

    return schedule_data;
}

pub fn map_course_ids_to_offering_query(course_ids: &Vec<i32>, term_name: String, file_path: &str) {
    let v: Vec<&str> = term_name.split(' ').collect(); // Ex. term_name = "Spring 2023"
    let term: String = String::from(v[0]);
    let year: i32 = v[1].parse::<i32>().unwrap();

    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create intermediate directories");
    }

    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path).expect(format!("Could not open file {file_path}").as_str());
    let mut file: BufWriter<File> = BufWriter::new(file);

    let start_of_query: &str = "INSERT INTO course_offerings (course_id, year, term)\nVALUES\n";
    let fail_msg: &str = "Could not write insert query for offerings table.";
    file.write_all(start_of_query.as_bytes()).expect(fail_msg);

    for (idx, id) in course_ids.iter().enumerate() {
        let mut val: String = format!("({}, {}, '{}'),\n", id, year, term);
        // Last tuple does not have comma.
        if idx == course_ids.len() - 1 {
            error!("LAST OFFERING VALUE: {}", val);
            val.pop(); // Remove newline.
            val.pop(); // Remove comma.
            val.push('\n'); // Insert newline back.
        }
        file.write_all(val.as_bytes()).expect(fail_msg);
    }

    // Keep it simple for now.
    let end_of_query: &str = "ON CONFLICT DO NOTHING;";
    file.write_all(end_of_query.as_bytes()).expect(fail_msg);
}

pub async fn get_offerings_from_table(pool: &PgPool) -> Vec<sample_db_models::CourseOffering> {
    let offerings_from_table: Result<Vec<sample_db_models::CourseOffering>, sqlx::Error> = 
        sqlx::query_as::<_, sample_db_models::CourseOffering>("SELECT * FROM course_offerings")
        .fetch_all(pool)
        .await;

    match offerings_from_table {
        Ok(offerings) => offerings,
        Err(err) => {
            error!("{:?}", err);
            Vec::<sample_db_models::CourseOffering>::new()
        }
    }
}