pub use course_mapper::*;
pub use course_store::*;

mod course_mapper;
mod course_store;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("cannot execute sql query against database: {0:?}")]
    QueryFailure(sqlx::Error),
    #[error("cannot find rows for sql query against database: {0:?}")]
    MissingRecords(sqlx::Error),
}
