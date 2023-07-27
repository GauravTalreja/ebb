use openapi::models;

pub fn extract_class_for_course_from_course_id_to_class_map(
    course_id_to_class_map: &Vec<(i32, Vec<models::Class>)>,
    course_id: i32
) -> Result<Vec<models::Class>, String> {

    // Since the course_id is a FK of course_offerings, it must match one of the vector elements.
    for (id, classes) in course_id_to_class_map {
        if *id == course_id {
            return Ok(classes.clone())
        } 
    }
    
    Err(String::from(format!(
        "Cannot find matching classes schedule for course id: {}", 
        course_id
    )))
}