use std::time::Instant;

// TODO: update struct to match ER domain.
pub struct Course {
    id: i32,
    code: String,
    name: String,
    department: String,
    credits: u16,
    instructor: String,
    start_date: Instant,
    end_date: Instant,
}
