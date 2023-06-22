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
        sqlx::migrate!("./migrations").run(pool).await.unwrap()
    }
}

struct SampleCourseSync;

#[async_trait]
impl CourseSync for SampleCourseSync {
    async fn init_sync(pool: PgPool) {
        sqlx::query_file!("./queries/insert_courses.sql")
            .execute(&pool.clone())
            .await
            .unwrap();

        sqlx::query_file!("./queries/insert_offerings.sql")
            .execute(&pool.clone())
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
}
