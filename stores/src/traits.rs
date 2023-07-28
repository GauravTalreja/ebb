use crate::prelude::*;
use models::*;

#[async_trait]
pub trait Migrate {
    async fn migrate(pool: &PgPool);
}

#[async_trait]
pub trait CourseSync {
    async fn init_sync(pool: PgPool);
}

#[async_trait]
pub trait CourseStore {
    async fn select_courses(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<CourseSummary>, Error>;
    async fn select_course(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Option<CourseDetail>, Error>;
    async fn select_course_offerings(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<OfferingDetail>, Error>;

    async fn get_last_updated_time(
        self: Arc<Self>,
    ) -> Result<LastUpdated, Error>;
}
