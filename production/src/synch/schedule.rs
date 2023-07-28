use super::db_models;
use super::util::extract_class_for_course_from_course_id_to_class_map;
use chrono::NaiveTime;
use log::error;
use openapi::models;
use stores::prelude::PgPool;

pub async fn insert_class_schedules(
    course_id_to_class_map: &Vec<(i32, Vec<models::Class>)>,
    offerings: &Vec<db_models::CourseOffering>,
    pool: &PgPool,
) -> Result<(), String> {
    // Iterate over course offerings.
    for offering_row in offerings.iter() {
        let course_offering_id: i32 = offering_row.id.clone();
        let course_id = offering_row.course_id.clone();
        let classes_resp: Result<Vec<models::Class>, String> =
            extract_class_for_course_from_course_id_to_class_map(course_id_to_class_map, course_id);
        if let Err(err) = classes_resp {
            return Err(err);
        }
        let classes: Vec<models::Class> = classes_resp.unwrap();

        // Iterate over scheduled classes of a course offering.
        for class in classes.iter() {
            let class: models::Class = class.clone();
            let schedule_obj: Option<Vec<models::ClassSchedule>> = class.schedule_data.flatten();

            // If we don't have a schedule, then move on to next entry.
            if schedule_obj.is_none() {
                error!(
                    "A Class Schedule is None for the course with offering id: {}",
                    course_offering_id
                );
                continue;
            }
            let schedule: models::ClassSchedule = schedule_obj.unwrap()[0].clone();
            let instructor_data: Option<Vec<models::ClassInstructor>> =
                class.instructor_data.flatten();

            // These values in the DB CANNOT be null.
            let class_section_obj: Option<i32> = class.class_section;
            let class_number_obj: Option<i32> = class.class_number;
            let component_obj: Option<String> = class.course_component.flatten();
            let max_enrollment_obj: Option<i32> = class.max_enrollment_capacity;
            let curr_enrollment_obj: Option<i32> = class.enrolled_students;
            let start_time_obj: Option<String> = schedule.class_meeting_start_time;
            let end_time_obj: Option<String> = schedule.class_meeting_end_time;
            let days_code_obj: Option<String> = schedule.class_meeting_week_pattern_code.flatten();

            // Check if key attributes are null.
            if class_section_obj.is_none()
                || class_number_obj.is_none()
                || component_obj.is_none()
                || max_enrollment_obj.is_none()
                || curr_enrollment_obj.is_none()
                || start_time_obj.is_none()
                || end_time_obj.is_none()
                || days_code_obj.is_none()
            {
                error!(
                    "A Class Schedule of the course with offering id: {:?} has NULL value \
                for a non-null attribute.",
                    course_offering_id
                );
                continue;
            }

            // The start/end time should be valid times.
            let start_time_str: String = start_time_obj
                .unwrap()
                .split("T")
                .nth(1)
                .unwrap()
                .to_owned();
            let end_time_str: String = end_time_obj.unwrap().split("T").nth(1).unwrap().to_owned();
            let start_time: NaiveTime;
            let end_time: NaiveTime;

            match start_time_str.parse::<NaiveTime>() {
                Ok(time) => start_time = time,
                Err(_) => {
                    error!(
                        "Could not parse start time of class schedule. Start time: {:?}",
                        start_time_str
                    );
                    continue;
                }
            }

            match end_time_str.parse::<NaiveTime>() {
                Ok(time) => end_time = time,
                Err(_) => {
                    error!(
                        "Could not parse end time of class schedule. End time: {:?}",
                        start_time_str
                    );
                    continue;
                }
            }

            // Extract true values.
            let class_section: i32 = class_section_obj.unwrap();
            let class_number: i32 = class_number_obj.unwrap();
            let component: String = component_obj.unwrap();
            let max_enrollment: i32 = max_enrollment_obj.unwrap();
            let curr_enrollment: i32 = curr_enrollment_obj.unwrap();
            let days_code: String = days_code_obj.unwrap();
            let mut monday: bool = false;
            let mut tuesday: bool = false;
            let mut wednesday: bool = false;
            let mut thursday: bool = false;
            let mut friday: bool = false;
            let mut saturday: bool = false;
            let mut sunday: bool = false;

            // Map week pattern code to column values.
            if days_code != "" {
                if days_code.chars().nth(0).unwrap() == 'Y' {
                    monday = true;
                }
                if days_code.chars().nth(1).unwrap() == 'Y' {
                    tuesday = true;
                }
                if days_code.chars().nth(2).unwrap() == 'Y' {
                    wednesday = true;
                }
                if days_code.chars().nth(3).unwrap() == 'Y' {
                    thursday = true;
                }
                if days_code.chars().nth(4).unwrap() == 'Y' {
                    friday = true;
                }
                if days_code.chars().nth(5).unwrap() == 'Y' {
                    saturday = true;
                }
                if days_code.chars().nth(6).unwrap() == 'Y' {
                    sunday = true;
                }
            }

            // Location for a class can be null.
            let location: Option<String> = schedule.location_name.flatten();
            let mut instructor_name: Option<String> = None;

            // We can have more than one instructor for a class. So need to process the vector and
            // convert to a single string with UNIQUE, comma-separated names.
            match instructor_data {
                Some(instructors) => {
                    let mut names: Vec<String> = Vec::<String>::new();
                    for instructor in instructors {
                        let first_name_obj: Option<String> =
                            instructor.instructor_first_name.flatten();
                        let last_name_obj: Option<String> =
                            instructor.instructor_last_name.flatten();

                        if first_name_obj.is_none() || last_name_obj.is_none() {
                            continue;
                        }
                        let first_name: String = first_name_obj.unwrap();
                        let last_name: String = last_name_obj.unwrap();
                        names.push(format!("{} {}", first_name, last_name))
                    }
                    let mut unique_names: Vec<String> = names.into_iter().collect();
                    unique_names.sort();
                    unique_names.dedup();

                    let i_name = unique_names
                        .iter()
                        .map(|n| n.as_str().clone())
                        .collect::<Vec<&str>>()
                        .join(", ");

                    instructor_name = Some(format!("{}", i_name));
                }
                _ => (),
            }

            let insert_query = sqlx::query!(
                "INSERT INTO class_schedule (class_section, class_number, component, start_time, \
                end_time, monday, tuesday, wednesday, thursday, friday, saturday, sunday, instructor_name, \
                location, course_offering_id, max_enrollment, current_enrollment) VALUES \
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) \
                ON CONFLICT (class_number, course_offering_id) \
                DO UPDATE \
                SET max_enrollment = EXCLUDED.max_enrollment, current_enrollment = EXCLUDED.current_enrollment;",
                class_section as i16,
                class_number as i16,
                component,
                start_time,
                end_time,
                monday,
                tuesday,
                wednesday,
                thursday,
                friday,
                saturday,
                sunday,
                instructor_name,
                location,
                course_offering_id,
                max_enrollment as i16,
                curr_enrollment as i16
            );
            let result: Result<_, _> = insert_query.execute(pool).await;
            match result {
                Err(_) => {
                    return Err(format!(
                        "Could not execute insert_class_schedule query with values: \
                    class section({}), class number({}), component({}), \
                    start_time({}), end_time({}), days({}/{}/{}/{}/{}/{}/{}), \
                    instructor_name({:?}), location({:?}), course_offering_id({}) \
                    max_enrollment({}), curr_enrollment({})",
                        class_section,
                        class_number,
                        component,
                        start_time,
                        end_time,
                        monday,
                        tuesday,
                        wednesday,
                        thursday,
                        friday,
                        saturday,
                        sunday,
                        instructor_name,
                        location,
                        course_offering_id,
                        max_enrollment,
                        curr_enrollment
                    ));
                }
                _ => (),
            }
        }
    }

    Ok(())
}
