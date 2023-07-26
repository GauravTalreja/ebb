use axum::{extract::Path, http::StatusCode, Extension, Json};
use perseus::web_log;

use models::{CourseDetail, CourseSummary, OfferingDetail};
use stores::prelude::*;

pub async fn list_courses(
    Path(course_code): Path<String>,
    Extension(store): Extension<EbbStore>,
) -> Result<Json<Vec<CourseSummary>>, StatusCode> {
    store
        .course_store
        .select_courses(&course_code)
        .await
        .map_err(|e| {
            web_log!("course_store.select_courses: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(Json)
}

pub async fn list_courses_with_filters(
        Path((
        course_code,
        term,
        level1,
        level2,
        level3,
        level4,
        include_closed,
        morning,
        afternoon,
        evening,
        monday,
        tuesday,
        wednesday,
        thursday,
        friday,
        saturday,
        )): Path<(String, String, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool)>,
    Extension(store): Extension<EbbStore>,
) -> Result<Json<Vec<CourseSummary>>, StatusCode> {
    web_log!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
    course_code, term, level1, level2, level3, level4, include_closed, morning,
    afternoon, evening, monday, tuesday, wednesday, thursday, friday, saturday);

    store
        .course_store
        .select_courses_with_filters(&course_code, &term, level1, level2, level3, level4, include_closed, morning, afternoon, evening, monday, tuesday, wednesday, thursday, friday, saturday)
        .await
        .map_err(|e| {
            web_log!("course_store.select_courses: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(Json)
}

pub async fn get_course(
    Path(course_code): Path<String>,
    Extension(store): Extension<EbbStore>,
) -> Result<Json<CourseDetail>, StatusCode> {
    store.course_store.select_course(&course_code).await
        .map_err(|e| {
            web_log!("course_store.select_course: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .and_then(|course| course
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND))
}

pub async fn list_course_offerings(
    Path(course_code): Path<String>,
    Extension(store): Extension<EbbStore>,
) -> Result<Json<Vec<OfferingDetail>>, StatusCode> {
    store
        .course_store
        .select_course_offerings(&course_code)
        .await
        .map_err(|e| {
            web_log!("course_store.select_course_offerings: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .map(Json)
}
