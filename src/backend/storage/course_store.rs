use sqlx::Row;

use crate::backend::storage::{MySqlClient, SqlType, StorageError};
use crate::models::Course;

#[derive(Clone)]
pub struct CourseStore {
    mysql_client: MySqlClient,
}

impl CourseStore {
    pub fn new(mysql_client: MySqlClient) -> Self {
        Self { mysql_client }
    }

    async fn create_course_table(&self) -> Result<(), StorageError> {
        let sql = "
            CREATE TABLE IF NOT EXISTS courses (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(10) NOT NULL UNIQUE,
                department VARCHAR(30) NOT NULL
            );
        ";
        self.mysql_client.write_query(sql, &[]).await?;
        Ok(())
    }

    pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, StorageError> {
        let sql = "
            SELECT id, name, department
            FROM courses WHERE name LIKE ?;
        ";
        let fuzzy_search_pattern = ["%", course_name, "%"].concat();
        let result_rows = self
            .mysql_client
            .read_query(sql, &[SqlType::Text(fuzzy_search_pattern)])
            .await?;

        Ok(result_rows
            .iter()
            .map(|row| Course {
                id: row.get("id"),
                name: row.get("name"),
                department: row.get("department"),
            })
            .collect::<Vec<Course>>())
    }

    pub fn insert_courses(&self, _courses: &[Course]) -> Result<(), StorageError> {
        todo!()
    }
}
