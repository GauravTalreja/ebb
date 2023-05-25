pub use course_store::*;

mod course_store;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("cannot connect to mysql server")]
    ConnectFailure(sqlx::Error),
    #[error("cannot execute sql query against mysql database: {0:?}")]
    QueryFailure(sqlx::Error),
}
