use openapi::apis::configuration::Configuration;
use openapi::models;
use log::{debug, info};
use stores::prelude::PgPool;
use super::{sample_db_models, course, term, offering, schedule};
use std::fs::read_to_string;

#[allow(dead_code, unused)]
pub async fn synchronize_data(config: &Configuration, pool: &PgPool) {
    let code: String = term::get_term_code_for_current_term(config).await;
    debug!("The term code for current term is: {:?}", code);
    
    let courses_from_api: Vec<models::Course> = 
        term::get_active_courses_for_term(&code, config).await;
    debug!("Total number of courses returned by courses/{code} endpoint: {}", courses_from_api.len());
    
    let insert_query_file_path: &str = "./production/src/synch/queries/sync_courses.sql";
    course::map_courses_to_insert_query(&courses_from_api, insert_query_file_path);
    info!("Wrote query to insert courses.");

    // For sample case, do everything in one large transaction.
    let mut tr = pool.begin().await.unwrap();
    
    // Execute inserts into courses table.
    let query: String = read_to_string("./production/src/synch/queries/sync_courses.sql").unwrap();
    sqlx::query(&query).execute(pool).await.unwrap();
    info!("Executed query to insert courses.");
    
    let courses_from_table: Vec<sample_db_models::Course> = course::get_courses_from_table(pool).await;
    debug!("Table size is now: {}", courses_from_table.len());

    // Get classes data for courses listed in the courses table.
    let course_id_to_class_map: Vec<(i32, Vec<models::Class>)> = 
        offering::get_class_schedules_for_courses(config, &code, &courses_from_table).await;
    info!("Total # of courses with schedues: {} for term {}", course_id_to_class_map.len(), code);

    // Extract course ids (note that this is the course(id) field).
    let mut course_ids : Vec<i32> = Vec::<i32>::new();
    for (id, _) in course_id_to_class_map.iter() { course_ids.push(id.clone()); }

    // Write the insert query for the course_offerings table.
    let offering_query_file_path: &str = "./production/src/synch/queries/sync_offerings.sql";
    offering::map_course_ids_to_offering_query(
        &course_ids, 
        term::get_term_name_for_current_term(config).await, 
        offering_query_file_path);
    info!("Wrote query for offerings table");
    
    // Execute query for offerings table.
    let offering_query: String = read_to_string("./production/src/synch/queries/sync_offerings.sql").unwrap();
    sqlx::query(&offering_query).execute(pool).await.unwrap();
    info!("Executed query for offerings table");

    // Get course offerings from table.
    let offerings_from_table: Vec<sample_db_models::CourseOffering> = 
        offering::get_offerings_from_table(pool).await;
    
    // Write the query for class schedules.
    schedule::map_class_data_to_class_schedule_query(
       &course_id_to_class_map,
       &offerings_from_table,
       "./production/src/synch/queries/sync_schedules.sql"
    );
    info!("Wrote class schedule query");

    // Execute query file.
    let schedule_query: String = read_to_string("./production/src/synch/queries/sync_schedules.sql").unwrap();
    sqlx::query(&schedule_query).execute(pool).await.unwrap();
    info!("Executed query for schedules table");

    tr.commit();
}