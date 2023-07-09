pub mod traits;
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::EbbStore;
    pub use async_trait::async_trait;
    pub use sqlx::{postgres::types, types::Json, Error, PgPool, Postgres};
    pub use std::sync::Arc;
}
use prelude::*;

#[derive(Clone)]
pub struct EbbStore {
    pub pool: PgPool,
    pub course_store: Arc<dyn CourseStore + Send + Sync>,
}

impl EbbStore {
    pub async fn new<M, D, C>(pool: PgPool, course_store: C) -> Self
    where
        M: Migrate,
        D: CourseSync,
        C: CourseStore + Send + Sync + 'static,
    {
        let store = Self {
            pool,
            course_store: Arc::new(course_store),
        };

        M::migrate(&store.pool).await;

        D::init_sync(store.pool.clone()).await;

        store
    }
}
