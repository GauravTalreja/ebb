use crate::models::Course;
use sqlx::{Error, PgPool};

use super::StorageConfig;

macro_rules! a {
    ($file_name:literal) => {
        match std::env::var("STORAGE_MODE").expect("Err").as_str(){
            "PROD" => sqlx::query_file!("src/backend/storage/prod_queries/" + $file_name),
            _ => sqlx::query_file!("src/backend/storage/sample_queries/" + $file_name),
        }
    };
}

#[derive(Clone)]
pub struct CourseStore {
    pool: PgPool,
    storage_configuration: StorageConfig,
}

impl CourseStore {
    pub fn new(pool: PgPool, storage_configuration: StorageConfig ) -> Self {
        CourseStore { pool, storage_configuration }
    }

    pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, Error> {
        sqlx::query_as!(
            Course,
            "SELECT id, name, department FROM courses WHERE name ILIKE $1",
            ["%", course_name, "%"].concat()
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn select_course_2(&self, course_name: &str) -> Result<Int, Error>{
        let path = self.storage_configuration.query_path.as_str();
        a!("PROD");
    }
}
