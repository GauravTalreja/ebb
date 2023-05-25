pub use course_store::*;
pub use mysql::*;

mod course_store;
mod mysql;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("cannot connect to mysql server")]
    ConnectFailure(sqlx::Error),
    #[error("cannot execute sql query against mysql database: {0:?}")]
    QueryFailure(sqlx::Error),
}

pub enum SqlType {
    Integer(i32),
    Float(f64),
    Text(String),
}
