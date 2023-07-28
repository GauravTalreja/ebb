use super::db_models;
use log::error;
use openapi::apis::class_schedules_api::v3_class_schedules_term_code_subject_catalog_number_get as get_class_schdule;
use openapi::apis::class_schedules_api::V3ClassSchedulesTermCodeSubjectCatalogNumberGetError;
use openapi::apis::configuration::Configuration;
use openapi::apis::Error;
use openapi::models;
use stores::prelude::PgPool;
use tokio::task;

pub async fn get_class_schedules_for_courses(
    config: &Configuration,
    term_code: &str,
    courses: &Vec<db_models::Course>,
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
                Error<V3ClassSchedulesTermCodeSubjectCatalogNumberGetError>,
            > = get_class_schdule(
                &config,
                term_code.as_str(),
                subject.as_str(),
                catalog_number.as_str(),
            )
            .await;
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

pub async fn insert_offerings(
    course_ids: &Vec<i32>,
    term_name: String,
    pool: &PgPool,
) -> Result<(), String> {
    let v: Vec<&str> = term_name.split(' ').collect(); // Ex. term_name = "Spring 2023"
    let term: String = String::from(v[0]);
    let year: i16 = v[1].parse::<i16>().unwrap();

    let mut term_vec: Vec<String> = Vec::new();
    for _ in 0..course_ids.len() {
        term_vec.push(term.clone());
    }
    let mut year_vec: Vec<i16> = Vec::new();
    for _ in 0..course_ids.len() {
        year_vec.push(year.clone());
    }

    // Use UNNEST method for fast bulk inserts. Only works for queries w/ all NON-NULL columns.
    let insert_query = sqlx::query!(
        "INSERT INTO course_offerings (course_id, term, year) \
        SELECT * FROM UNNEST($1::int[], $2::text[], $3::int2[]) \
        ON CONFLICT (course_id, term, year) DO NOTHING;",
        course_ids,
        &term_vec,
        &year_vec
    );

    let result: Result<_, _> = insert_query.execute(pool).await;
    if let Err(_) = result {
        return Err(String::from(
            "Could not insert course offerings into table.",
        ));
    }

    Ok(())
}

pub async fn get_offerings_from_table(pool: &PgPool) -> Vec<db_models::CourseOffering> {
    let offerings_from_table: Result<Vec<db_models::CourseOffering>, sqlx::Error> =
        sqlx::query_as::<_, db_models::CourseOffering>("SELECT * FROM course_offerings")
            .fetch_all(pool)
            .await;

    match offerings_from_table {
        Ok(offerings) => offerings,
        Err(err) => {
            error!("{:?}", err);
            Vec::<db_models::CourseOffering>::new()
        }
    }
}
