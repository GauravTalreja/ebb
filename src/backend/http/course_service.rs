use axum::{extract::Path, http::StatusCode, Extension, Json};

use crate::backend::storage::{CourseStore, StorageError};
use crate::models::Course;

pub async fn get_course(
    Path(course_code): Path<String>,
    Extension(course_store): Extension<CourseStore>,
) -> Result<Json<Course>, StatusCode> {
    match course_store.select_course_by_code(&course_code).await {
        Ok(course) => Ok(Json(course)),
        Err(err) => match err {
            StorageError::MissingRecords(_) => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn list_courses(
    Path(course_code): Path<String>,
    Extension(course_store): Extension<CourseStore>,
) -> Result<Json<Vec<Course>>, StatusCode> {
    // Current course offerings should only be listed from terms in the current year.
    match course_store.select_courses(&course_code).await {
        Ok(courses) => Ok(Json(courses)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
