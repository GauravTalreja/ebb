use crate::{backend::storage::CourseStore, models::Course};
use axum::{extract::Path, http::StatusCode, Extension, Json};

pub async fn list_courses(
    Path(course_name): Path<String>,
    Extension(course_store): Extension<CourseStore>,
) -> Result<Json<Vec<Course>>, StatusCode> {
    match course_store.select_courses(&course_name).await {
        Ok(courses) => Ok(Json(courses)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
