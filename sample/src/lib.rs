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

    async fn select_courses_with_filters(
        self: Arc<Self>,
        course_code: &str,
        term: &str,
        level1: bool,
        level2: bool,
        level3: bool,
        level4: bool,
        include_closed: bool,
        morning: bool,
        afternoon: bool,
        evening: bool,
        monday: bool,
        tuesday: bool,
        wednesday: bool,
        thursday: bool,
        friday: bool,
        saturday: bool,
    ) -> Result<Vec<CourseSummary>, Error> {

        let mut valid_codes: [String; 4] = ["".to_string(), "".to_string(), "".to_string(), "".to_string()];
        if !level1 && !level2 && !level3 && !level4 {
            valid_codes[0] = "%".to_string();
        }
        if level1 {
            valid_codes[0] = "1%".to_string();
        };
        if level2 {
            valid_codes[1] = "2%".to_string();
        };
        if level3 {
            valid_codes[2] = "3%".to_string();
        };
        if level4 {
            valid_codes[3] = "4%".to_string();
        };

        let mut check_days = true;
        if !monday && !tuesday && !wednesday && !thursday && !friday && !saturday {
            check_days = false;
        }

        let mut check_time = true;
        if !morning && !afternoon && !evening {
            check_time = false;
        }

        sqlx::query_file_as!(
            CourseSummary,
            "./queries/select_courses_with_filters.sql",
            2023,
            ["%", &course_code.to_uppercase(), "%"].concat(),
            &valid_codes,
            check_days,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            check_time,
            morning,
            afternoon,
            evening,
            include_closed,
            ["%", &term, "%"].concat(),
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn select_course_offerings(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<OfferingDetail>, Error> {
        sqlx::query_file_as!(
            OfferingDetail,
            "./queries/select_class_schedules.sql",
            &course_code.to_uppercase(),
        )
        .fetch_all(&self.pool)
        .await
    }
}
