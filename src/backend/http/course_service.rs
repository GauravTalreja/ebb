use axum::{extract::Path, http::StatusCode, Extension, Json};
use models::CourseSummary;
use perseus::web_log;
use stores::prelude::*;

pub async fn list_courses(
    Path(course_code): Path<String>,
    Extension(store): Extension<EbbStore>,
) -> Result<Json<Vec<CourseSummary>>, StatusCode> {
    match store.course_store.select_courses(&course_code).await {
        Ok(courses) => Ok(Json(courses)),
        Err(e) => {
            web_log!("course_store.select_courses: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
