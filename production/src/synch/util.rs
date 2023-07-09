use openapi::models;

/// Helper fn that either extracts string value from Option<Option<String>>
/// type, or returns "NULL" as a value. To be used for inserts.
pub fn extract_value_or_get_null_string(val: Option<Option<String>>) -> String {
    match val.flatten() {
        Some(str_val) => format!("'{}'", str_val.replace("'", "''")),
        None => "NULL".to_string()
    }
}

#[allow(unused)]
pub fn extract_class_for_course_from_course_id_to_class_map(
    course_id_to_class_map: &Vec<(i32, Vec<models::Class>)>,
    course_id: i32
) -> Vec<models::Class> {

    // Since the course_id is a FK of course_offerings, it must match one of the vector elements.
    for (id, classes) in course_id_to_class_map {
        if *id == course_id {
            return classes.clone()
        } 
    }
    
    panic!("Cannot find matching classes schedule for {}", course_id);
}