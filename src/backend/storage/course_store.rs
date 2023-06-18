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

    pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, Error>{
        match self.storage_configuration.mode{
            super::StorageConfigMode::Prod => {
                sqlx::query_file_as!(
                    Course,
                    "src/backend/storage/prod_queries/select_courses.sql",
                    ["%", course_name, "%"].concat()
                )
            },
            super::StorageConfigMode::Sample => todo!(),
        }
        .fetch_all(&self.pool)
        .await
    }
}
