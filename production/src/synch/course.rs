use super::db_models;
use log::error;
use openapi::models;
use stores::prelude::PgPool;

pub async fn get_courses_from_table(pool: &PgPool) -> Vec<db_models::Course> {
    let courses_from_table: Result<Vec<db_models::Course>, sqlx::Error> =
        sqlx::query_as::<_, db_models::Course>("SELECT * FROM courses")
            .fetch_all(pool)
            .await;

    match courses_from_table {
        Ok(courses) => courses,
        Err(err) => {
            error!("{:?}", err);
            Vec::<db_models::Course>::new()
        }
    }
}

pub async fn insert_courses(pool: &PgPool, courses: &Vec<models::Course>) -> Result<(), String> {
    for course in courses.iter() {
        let c: db_models::InsertCourse = db_models::map_api_course_to_db_course(course);
        let insert_query = sqlx::query!(
            "INSERT INTO courses (catalog_number, subject_code, external_id \
            , academic_level, title, description, requirements, enroll_consent, drop_consent, \
            prerequisites_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (catalog_number, subject_code) \
            DO UPDATE SET external_id = EXCLUDED.external_id, academic_level = EXCLUDED.academic_level, title = EXCLUDED.title, 
            description = EXCLUDED.description, requirements = EXCLUDED.requirements, enroll_consent = EXCLUDED.enroll_consent, 
            drop_consent = EXCLUDED.drop_consent;",
            c.catalog_number,
            c.subject_code,
            c.external_id,
            c.academic_level,
            c.title,
            c.description,
            c.requirements,
            c.enroll_consent,
            c.drop_consent,
            c.prerequisites_id
        );
        let result: Result<_, _> = insert_query.execute(pool).await;
        match result {
            Err(_) => error!("Could not insert {}{}", c.subject_code, c.catalog_number),
            _ => (),
        }
    }
    Ok(())
}
