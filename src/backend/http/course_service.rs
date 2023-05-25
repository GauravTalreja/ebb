use std::error::Error;

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::json;

use crate::backend::storage::CourseStore;
use crate::models::ListCourseResponse;

pub async fn list_courses(
    Path(course_name): Path<String>,
    Extension(course_store): Extension<CourseStore>,
) -> Result<Json<ListCourseResponse>, StatusCode> {
    let sanitized_name = course_name
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();

    match course_store.select_courses(&sanitized_name).await {
        Ok(courses) => Ok(Json(ListCourseResponse { courses })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
