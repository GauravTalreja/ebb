use sqlx::{PgPool, Postgres};

use crate::backend::storage::StorageError;
use crate::models::Course;

// use super::{StorageConfig, StorageConfigMode};

#[derive(Clone)]
pub struct CourseStore {
    pool: PgPool,
    // storage_configuration: StorageConfig,
}

impl CourseStore {
    pub fn new(pool: PgPool/*, storage_configuration: StorageConfig*/) -> Self {
        CourseStore {
            pool,
            // storage_configuration,
        }
    }

    /// Selects courses for the current year from a pre-built materialized view.
    pub async fn select_courses(&self, course_code: &str) -> Result<Vec<Course>, StorageError> {
        let sql = "
        SELECT
            id,
            catalog_number,
            subject_code,
            title,
            external_id,
            offerings
        FROM
            mv_courses
        WHERE
            subject_code || catalog_number::VARCHAR ILIKE $1
        GROUP BY
            id,
            title,
            external_id,
            subject_code,
            catalog_number,
            offerings
        ORDER BY
            catalog_number
        LIMIT
            100;
        ";

        sqlx::query_as::<Postgres, Course>(sql)
            .bind(["%", &course_code.to_uppercase(), "%"].concat())
            .fetch_all(&self.pool)
            .await
            .map_err(StorageError::QueryFailure)
    }

    pub async fn select_course_by_code(&self, course_code: &str) -> Result<Course, StorageError> {
        let sql = "
        SELECT
            c.id AS id,
            c.catalog_number AS catalog_number,
            c.subject_code AS subject_code,
            c.external_id AS external_id,
            c.academic_level AS academic_level,
            c.title AS title,
            c.description AS description,
            COALESCE(
                NULLIF(json_agg(DISTINCT co.*)::TEXT, '[null]'),
                '[]'
            ) AS offerings,
            COALESCE(
                NULLIF(array_agg(DISTINCT rc.subject_code || rc.catalog_number::TEXT), '{null}'),
                '{}'
            ) AS required_prerequisites,
            COALESCE(
                NULLIF(array_agg(DISTINCT oc.subject_code || oc.catalog_number::TEXT), '{null}'),
                '{}'
            ) AS optional_prerequisites
        FROM
            courses c
            LEFT JOIN course_offerings co ON c.id = co.course_id
            LEFT JOIN prerequisites p ON c.prerequisites_id = p.id
            LEFT JOIN required_prerequisites rp ON p.id = rp.prerequisite_id
            LEFT JOIN courses rc ON rp.course_id = rc.id
            LEFT JOIN optional_prerequisites op ON p.id = op.prerequisite_id
            LEFT JOIN courses oc ON op.course_id = oc.id
        WHERE
            c.subject_code || c.catalog_number::VARCHAR = $1
        GROUP BY
            c.id
        LIMIT 1;
        ";

        sqlx::query_as::<Postgres, Course>(sql)
            .bind(course_code.to_uppercase())
            .fetch_one(&self.pool)
            .await
            .map_err(StorageError::MissingRecords)

        // TODO: enable conditional sql query usage
        //     pub async fn select_courses(&self, course_name: &str) -> Result<Vec<Course>, Error> {
        //         match self.storage_configuration.mode {
        //             StorageConfigMode::Production => {
        //                 sqlx::query_file_as!(
        //                     Course,
        //                     "src/backend/storage/prod_queries/select_courses.sql",
        //                     ["%", course_name, "%"].concat()
        //                 )
        //                 .fetch_all(&self.pool)
        //                 .await
        //             }
        //             StorageConfigMode::Sample => {
        //                 sqlx::query_file_as!(
        //                     Course,
        //                     "src/backend/storage/sample_queries/select_courses.sql",
        //                     ["%", course_name, "%"].concat()
        //                 )
        //                 .fetch_all(&self.pool)
        //                 .await
        //             }
        //         }
    }
}
