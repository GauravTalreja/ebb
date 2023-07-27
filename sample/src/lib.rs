use models::*;
use stores::prelude::*;

pub async fn sample_store() -> EbbStore {
    dotenvy::dotenv().expect("Unable to find a sample .env file");
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("Unable to connect to the database");
    EbbStore::new::<SampleMigrate, SampleCourseSync, SampleCourseStore>(
        pool.clone(),
        SampleCourseStore { pool },
    )
    .await
}

struct SampleMigrate;

#[async_trait]
impl Migrate for SampleMigrate {
    async fn migrate(pool: &PgPool) {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .expect("cannot run sample database migrations")
    }
}

struct SampleCourseSync;

#[async_trait]
impl CourseSync for SampleCourseSync {
    async fn init_sync(pool: PgPool) {
        sqlx::query_file!("./queries/insert_courses.sql")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query_file!("./queries/insert_offerings.sql")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query_file!("./queries/insert_schedules.sql")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query_file!("./queries/update_timestamp.sql")
            .execute(&pool)
            .await
            .unwrap();
    }
}

struct SampleCourseStore {
    pool: PgPool,
}

#[async_trait]
impl CourseStore for SampleCourseStore {
    async fn select_courses(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<CourseSummary>, Error> {
        sqlx::query_file_as!(
            CourseSummary,
            "./queries/select_courses.sql",
            2023,
            ["%", &course_code.to_uppercase(), "%"].concat()
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn select_course(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Option<CourseDetail>, Error> {
        sqlx::query_file_as!(
            CourseDetail,
            "./queries/select_course.sql",
            &course_code.to_uppercase()
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn select_course_offerings(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<OfferingDetail>, Error> {
        sqlx::query_file_as!(
            OfferingDetail,
            "./queries/select_class_schedules.sql",
            &course_code.to_uppercase()
        )
        .fetch_all(&self.pool)
        .await
    }
}
