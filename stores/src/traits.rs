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
    ) -> Result<Vec<CourseSummary>, Error>;
    async fn select_course_offerings(
        self: Arc<Self>,
        course_code: &str,
    ) -> Result<Vec<OfferingDetail>, Error>;
}
