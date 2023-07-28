use models::*;
use stores::prelude::*;
pub mod synch;
use chrono::{DateTime, Utc};

pub async fn prod_store() -> EbbStore {
    dotenvy::dotenv().expect("Unable to find a prod .env file");

    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("Unable to connect to the database");

    EbbStore::new::<ProdMigrate, ProdCourseSync, ProdCourseStore>(
        pool.clone(),
        ProdCourseStore { pool },
    )
    .await
}

struct ProdMigrate;

#[async_trait]
impl Migrate for ProdMigrate {
    async fn migrate(pool: &PgPool) {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .expect("cannot run production database migrations")
    }
}

struct ProdCourseSync;

#[async_trait]
impl CourseSync for ProdCourseSync {
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

        sqlx::query_file!("./queries/refresh_courses_mv.sql")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query_file!("./queries/update_timestamp.sql")
        .execute(&pool)
        .await
        .unwrap();
    }
}

struct ProdCourseStore {
    pool: PgPool,
}

#[async_trait]
impl CourseStore for ProdCourseStore {
    async fn select_courses(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<CourseSummary>, Error> {
        sqlx::query_file_as!(
            CourseSummary,
            "./queries/select_courses.sql",
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

    async fn get_last_updated_time(
        self: Arc<Self>
    ) -> Result<LastUpdated, Error> {
        let q: Result<TempProdUpdated, Error> = sqlx::query_file_as!(
            TempProdUpdated,
            "./queries/select_last_updated.sql"
        )
        .fetch_one(&self.pool)
        .await;

        if let Err(e) = q {
            return Err(e);
        } else {
            return Ok(LastUpdated { date_time: Some(q.unwrap().date_time) })
        }
    
    }
}

struct TempProdUpdated {
    date_time: DateTime<Utc>
}