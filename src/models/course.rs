use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub title: String,
    pub description: String,
    pub subject_code: String,
    pub catalog_number: u16,
    pub level: AcademicLevel,
    pub offerings: Vec<CourseOffering>,
    pub prerequisites: Prerequisites,
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
    pub term: String,
    pub instructor_names: Vec<String>,
    pub max_enrollment: u16,
    pub current_enrollment: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Term {
    Fall,
    Winter,
    Spring,
}
