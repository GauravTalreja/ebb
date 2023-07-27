use openapi::apis::configuration::Configuration;
use openapi::models;
use log::{debug, info};
use stores::prelude::PgPool;
use super::{db_models, course, term, offering, schedule};
use std::time::Instant;

pub async fn synchronize_data(config: &Configuration, pool: &PgPool) -> Result<(), String> {
    let code: Result<String, String> = term::get_term_code_for_current_term(config).await;

    // If code is incorrect, return early. GOOD.
    if let Err(err) = code { return Err(err); }
    let code: String = code.unwrap();
    info!("The term code for current term is: {:?}", code);
    
    // Get courses from the OpenData API. GOOD.
    let get_courses_resp: Result<Vec<models::Course>, String> = 
        term::get_active_courses_for_term(&code, config).await;
    if let Err(err) = get_courses_resp { return Err(err); }
    let courses_from_api: Vec<models::Course> = get_courses_resp.unwrap();
    debug!("Total number of courses returned by courses/{code} endpoint: {}", courses_from_api.len());

    let txn_obj: Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error> = pool.begin().await;
    if let Err(err) = txn_obj { return Err(err.to_string()); }
    let tr: sqlx::Transaction<'_, sqlx::Postgres> = txn_obj.unwrap();

    // Insert courses.
    let start_time: Instant = Instant::now();
    let insert_courses_resp: Result<(), String> = course::insert_courses(pool, &courses_from_api).await;
    if let Err(err) = insert_courses_resp { return Err(err); }
    let end_time: Instant = Instant::now();
    info!("INSERT INTO courses query took: {} secs", (end_time - start_time).as_secs());
    
    let courses_from_table: Vec<db_models::Course> = course::get_courses_from_table(pool).await;
    info!("Table size is now: {}", courses_from_table.len());

    // Get classes data for courses listed in the courses table.
    let course_id_to_class_map: Vec<(i32, Vec<models::Class>)> = 
        offering::get_class_schedules_for_courses(config, &code, &courses_from_table).await;
    info!("Total # of courses with schedues: {} for term {}", course_id_to_class_map.len(), code);

    // Extract course ids (note that this is the course(id) field).
    let mut course_ids : Vec<i32> = Vec::<i32>::new();
    for (id, _) in course_id_to_class_map.iter() { course_ids.push(id.clone()); }

    // Insert offerings.
    let start_time: Instant = Instant::now();
    let term_name: Result<String, String> = term::get_term_name_for_current_term(config).await;
    if let Err(err) = term_name { return Err(err); }
    let term_name: String = term_name.unwrap();
    let insert_offerings_resp: Result<(), String> = 
        offering::insert_offerings(&course_ids, term_name, pool).await;
    if let Err(err) = insert_offerings_resp { return Err(err); }
    let end_time = Instant::now();
    info!("INSERT INTO course_offerings query took: {} secs", (end_time - start_time).as_secs());

    // Get course offerings from table.
    let offerings_from_table: Vec<db_models::CourseOffering> = 
        offering::get_offerings_from_table(pool).await;

    // Insert class schedules.
    let start_time: Instant = Instant::now();
    let insert_schedules_resp: Result<(), String> = 
        schedule::insert_class_schedules(&course_id_to_class_map, &offerings_from_table, pool).await;
    if let Err(err) = insert_schedules_resp { return Err(err); }
    let end_time: Instant = Instant::now();
    info!("INSERT INTO class_schedule query took: {} secs", (end_time - start_time).as_secs());

    // Refresh materialized view.
    let refresh_mv_resp: Result<_, _> = 
        sqlx::query_file!("./queries/refresh_courses_mv.sql").execute(pool).await;
    if let Err(_) = refresh_mv_resp { 
        return Err(String::from("Could not refresh materialized view mv_courses.")) 
    }

    let commit_result: Result<(), sqlx::Error> = tr.commit().await;
    if let Err(_) = commit_result { return Err(String::from("Could not commit full transaction.")); } 
    
    Ok(())
}