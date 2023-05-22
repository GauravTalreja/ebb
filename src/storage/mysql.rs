use std::any::Any;
use std::error::Error;
use std::num::NonZeroUsize;
use std::time::Duration;

use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};
use sqlx::{Executor, MySql, Pool};

use crate::config::DatabaseConfig;
use crate::storage::{SqlType, StorageError};

pub struct MySqlClient {
    conn_pool: Pool<MySql>,
}

impl MySqlClient {
    pub async fn new(config: &DatabaseConfig) -> Result<MySqlClient, StorageError> {
        let database_uri = format!(
            "mysql://{}:{}@{}/{}",
            config.username, config.password, config.hostname, config.database
        );
        let conn_pool = MySqlPoolOptions::new()
            .max_connections(config.pool_size.get() as u32)
            .connect(&database_uri)
            .await
            .map_err(StorageError::ConnectFailure)?;

        println!("Connected to mysql server...");
        Ok(Self { conn_pool })
    }

    pub async fn run_query(
        &self,
        sql_query: &str,
        sql_args: &[SqlType],
    ) -> Result<MySqlQueryResult, StorageError> {
        let mut query = sqlx::query(sql_query);
        for arg in sql_args {
            query = match arg {
                SqlType::Integer(value) => query.bind(value),
                SqlType::Float(value) => query.bind(value),
                SqlType::Text(value) => query.bind(value),
            };
        }

        query
            .execute(&self.conn_pool)
            .await
            .map_err(StorageError::QueryFailure)
    }

    async fn disconnect(&self) {
        self.conn_pool.close().await;
    }
}
