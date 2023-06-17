use chrono::NaiveTime;
use serde::{de, Deserialize, Deserializer, Serialize};

/// Optional fields are not included when listing courses, but are included
/// when reading a course in it's description page.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub title: String,
    pub external_id: String,
    pub subject_code: String,
    pub catalog_number: u16,
    pub description: Option<String>,
    pub level: Option<AcademicLevel>,
    pub offerings: Vec<CourseOffering>,
    pub prerequisites: Option<Prerequisites>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prerequisites {
    pub optional_courses: Vec<String>,
    pub required_courses: Vec<String>,
}

impl Course {
    pub fn code(&self) -> String {
        let mut course_code = String::new();
        course_code.push_str(&self.subject_code);
        course_code.push_str(&self.catalog_number.to_string());
        course_code
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AcademicLevel {
    Undergraduate,
    Graduate,
}

impl From<String> for AcademicLevel {
    fn from(level: String) -> Self {
        match level.to_lowercase().as_str() {
            "graduate" => AcademicLevel::Graduate,
            "undergraduate" | _ => AcademicLevel::Undergraduate,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CourseOffering {
    pub year: u16,
    pub term: Term,
    pub instructor_names: Vec<String>,
    pub max_enrollment: u16,
    pub current_enrollment: u16,
    pub events: Vec<CourseEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CourseEvent {
    pub section: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub meeting_days: String,
    pub instructor_name: String,
    pub room_name: Option<String>,
    pub campus_name: String,
    pub course_offering_id: String,
    pub max_enrollment: u16,
    pub current_enrollment: u16,
}

#[derive(Clone, Debug, Serialize)]
pub enum Term {
    Fall,
    Winter,
    Spring,
}

impl<'de> Deserialize<'de> for Term {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let term = String::deserialize(deserializer)?;
        match term.to_lowercase().as_str() {
            "fall" => Ok(Term::Fall),
            "winter" => Ok(Term::Winter),
            "spring" => Ok(Term::Spring),
            _ => Err(de::Error::unknown_variant(
                &term,
                &["fall", "winter", "spring"],
            )),
        }
    }
}
