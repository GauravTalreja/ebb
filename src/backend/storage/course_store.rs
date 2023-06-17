use crate::models::Course;
use sqlx::{Error, PgPool};

use super::StorageConfig;

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
        sqlx::query_file!(
            format!("{path}")
            // "src/backend/storage/prod_queries/test_query.sql"
        )
    }
}
