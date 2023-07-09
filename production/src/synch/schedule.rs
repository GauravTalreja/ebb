use openapi::models;
use super::sample_db_models;
use std::fs::{OpenOptions, File, create_dir_all};
use std::path::Path;
use std::io::{BufWriter, Write};
use super::util::extract_class_for_course_from_course_id_to_class_map;

#[allow(unused)]
pub fn map_class_data_to_class_schedule_query(
    course_id_to_class_map: &Vec<(i32, Vec<models::Class>)>, 
    offerings: &Vec<sample_db_models::CourseOffering>,
    file_path: &str
) {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create intermediate directories");
    }
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path).expect(format!("Could not open file {file_path}").as_str());
    let mut file: BufWriter<File> = BufWriter::new(file);
    let fail_msg = "Could not write insert query for class_schedule table.";

    let start_of_query: &str = "INSERT INTO class_schedule (class_section, class_number, \
        component, start_time, end_time, monday, tuesday, wednesday, thursday, friday, \
        saturday, sunday, instructor_name, location, course_offering_id, max_enrollment, \
        current_enrollment)\nVALUES\n";
    file.write_all(start_of_query.as_bytes()).expect(fail_msg);

    for (outer_idx, offering_row) in offerings.iter().enumerate() {
        let course_offering_id: i32 = offering_row.id.clone(); 
        let course_id = offering_row.course_id.clone();

        let classes: Vec<models::Class> = extract_class_for_course_from_course_id_to_class_map(
            course_id_to_class_map, course_id);
        
        for (inner_idx, class) in classes.iter().enumerate() {
            let class = class.clone();
            let schedule: models::ClassSchedule = class.schedule_data.unwrap().unwrap()[0].clone();
            let instructor_data: Option<Vec<models::ClassInstructor>> = class.instructor_data.flatten();
            let class_section: i32 = class.class_section.unwrap();
            let class_number: i32 = class.class_number.unwrap();
            let component: String = class.course_component.unwrap().unwrap();
            let max_enrollment: i32 = class.max_enrollment_capacity.unwrap();
            let curr_enrollment: i32 = class.enrolled_students.unwrap();
            let start_time: String = schedule.class_meeting_start_time.unwrap().split("T").nth(1).unwrap().to_owned();
            let end_time: String = schedule.class_meeting_end_time.unwrap().split("T").nth(1).unwrap().to_owned();
            let days_code: String = schedule.class_meeting_week_pattern_code.unwrap().unwrap();
            let mut location: String = String::new(); 
            
            // locations for a class can be null.
            match schedule.location_name.flatten() {
                Some(loc_name) => {
                    location = loc_name.replace("'", "''");
                    location = format!("'{}'", location);
                },
                None => location = String::from("NULL")
            }

            let mut monday: &str = "false";
            let mut tuesday: &str = "false";
            let mut wednesday: &str = "false";
            let mut thursday: &str = "false";
            let mut friday: &str = "false";
            let mut saturday: &str = "false";
            let mut sunday: &str = "false";
            
            if days_code != "" {
                if days_code.chars().nth(0).unwrap() == 'Y' { monday = "true" }
                if days_code.chars().nth(1).unwrap() == 'Y' { tuesday = "true" }
                if days_code.chars().nth(2).unwrap() == 'Y' { wednesday = "true" }
                if days_code.chars().nth(3).unwrap() == 'Y' { thursday = "true" }
                if days_code.chars().nth(4).unwrap() == 'Y' { friday = "true" }
                if days_code.chars().nth(5).unwrap() == 'Y' { saturday = "true" }
                if days_code.chars().nth(6).unwrap() == 'Y' { sunday = "true" }
            }

            let mut instructor_name: String = "NULL".to_string();

            match instructor_data {
                Some(instructors) => {
                    let mut names: Vec<String> = Vec::<String>::new();
                    for instructor in instructors {
                        let first_name: String = instructor.instructor_first_name.unwrap().unwrap();
                        let last_name = instructor.instructor_last_name.unwrap().unwrap();
                        names.push(format!("{} {}", first_name, last_name));
                    }
                    
                    let mut unique_names: Vec<String> = names.into_iter().collect();
                    unique_names.sort();
                    unique_names.dedup();
                    
                    let i_name = unique_names
                        .iter()
                        .map(|n| n.as_str().clone())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    
                    instructor_name = format!("'{}'", i_name.replace("'", "''"));

                },
                None => (),
            }

            let mut curr_query: String = format!("({class_section}, {class_number}, '{component}', \
                '{start_time}', '{end_time}', {monday}, {tuesday}, {wednesday}, \
                {thursday}, {friday}, {saturday}, {sunday}, {instructor_name}, {location}, \
                {course_offering_id}, {max_enrollment}, {curr_enrollment}),\n");
            
            // For the last tuple, eliminate the additional comma.
            if outer_idx == offerings.len() - 1 && inner_idx == classes.len() - 1 {
                curr_query.pop(); // remove newline.
                curr_query.pop(); // remove comma.
                curr_query.push('\n');
            }

            file.write_all(curr_query.as_bytes());        
        }
    }
    let end_of_query: &str = "ON CONFLICT DO NOTHING;";
    file.write_all(end_of_query.as_bytes());
}