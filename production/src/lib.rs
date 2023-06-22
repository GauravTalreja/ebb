use models::*;
use stores::prelude::*;

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
    async fn migrate(_pool: &PgPool) {
        // sqlx::migrate!("./migrations").run(pool).await.unwrap()
        todo!()
    }
}

struct ProdCourseSync;

#[async_trait]
impl CourseSync for ProdCourseSync {
    async fn init_sync(_pool: PgPool) {
        todo!()
    }
}

struct ProdCourseStore {
    pool: PgPool,
}

#[async_trait]
impl CourseStore for ProdCourseStore {
    async fn select_courses(
        self: Arc<Self>,
        _course_code: &str,
    ) -> Result<Vec<CourseSummary>, Error> {
        let _pool = &self.pool;
        todo!()
    }
}
