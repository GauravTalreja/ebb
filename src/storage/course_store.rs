use crate::models::Course;
use crate::storage::{MySqlClient, StorageError};

pub struct CourseStore {
    mysql_client: MySqlClient,
}

impl CourseStore {
    pub async fn new(mysql_client: MySqlClient) -> Result<Self, StorageError> {
        let course_store = Self { mysql_client };
        course_store.create_course_table().await?;
        Ok(course_store)
    }

    async fn create_course_table(&self) -> Result<(), StorageError> {
        let sql = "
            CREATE TABLE IF NOT EXISTS models (
                id INT AUTO_INCREMENT PRIMARY KEY,
                code VARCHAR(10) NOT NULL,
                name VARCHAR(100) NOT NULL,
                department VARCHAR(100) NOT NULL,
                credits INT,
                instructor VARCHAR(100),
                start_date DATE,
                end_date DATE
        );";

        self.mysql_client.run_query(sql, &[]).await?;
        Ok(())
    }

    pub fn find_courses(&self) -> Result<Vec<Course>, StorageError> {
        todo!()
    }

    pub fn add_courses(&self, courses: &[Course]) -> Result<(), StorageError> {
        todo!()
    }
}
