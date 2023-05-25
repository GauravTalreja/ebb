use crate::backend::storage::StorageError;
use crate::models::Course;
use sqlx::any;

#[derive(Clone)]
pub struct CourseStore {
    pool: any::AnyPool,
}

impl CourseStore {
    pub fn new(pool: any::AnyPool) -> Self {
        CourseStore { pool }
    }

    pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, StorageError> {
        sqlx::query_as!(
            Course,
            "SELECT id, name, department FROM courses WHERE name LIKE ?",
        )
        .bind(["%", course_name, "%"].concat())
        .fetch_all()
        .await
    }
}
