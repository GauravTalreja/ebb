use serde::{Deserialize, Serialize};

// TODO: update struct to match ER domain.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub department: String,
}