use crate::models::Course;
use sqlx::{Error, PgPool};

#[derive(Clone)]
pub struct CourseStore {
    pool: PgPool,
}

impl CourseStore {
    pub fn new(pool: PgPool) -> Self {
        CourseStore { pool }
    }

    pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, Error> {
        Ok(Vec::new())
    }
}
