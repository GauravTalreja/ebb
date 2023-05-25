use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::sync::Arc;

use sqlx::database::HasArguments;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlQueryResult, MySqlRow};
use sqlx::query::Query;
use sqlx::{MySql, Pool, query};

use crate::backend::storage::{SqlType, StorageError};
use crate::config::DatabaseConfig;

#[derive(Clone)]
pub struct MySqlClient {
    conn_pool: Arc<Pool<MySql>>,
}

impl MySqlClient {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, StorageError> {
        let conn_settings = MySqlConnectOptions::new()
            .host(&config.hostname)
            .port(config.port.get())
            .database(&config.database)
            .username(&config.username)
            .password(&config.password);

        let conn_pool = MySqlPoolOptions::new()
            .max_connections(config.pool_size.get() as u32)
            .connect_with(conn_settings)
            .await
            .map_err(StorageError::ConnectFailure)?;

        Ok(Self {
            conn_pool: Arc::new(conn_pool),
        })
    }

    pub async fn read_query(
        &self,
        sql_query: &str,
        sql_args: &[SqlType],
    ) -> Result<Vec<MySqlRow>, StorageError> {
        Self::prepare_query(sql_query, sql_args)
            .fetch_all(self.conn_pool.deref())
            .await
            .map_err(StorageError::QueryFailure)
    }

    pub async fn write_query(
        &self,
        sql_query: &str,
        sql_args: &[SqlType],
    ) -> Result<MySqlQueryResult, StorageError> {
        Self::prepare_query(sql_query, sql_args)
            .execute(self.conn_pool.deref())
            .await
            .map_err(StorageError::QueryFailure)
    }

    fn prepare_query<'a>(
        sql_query: &'a str,
        sql_args: &'a [SqlType],
    ) -> Query<'a, MySql, <MySql as HasArguments<'a>>::Arguments> {
        let mut query = sqlx::query(sql_query);
        for arg in sql_args.iter() {
            query = match arg {
                SqlType::Integer(value) => query.bind(value),
                SqlType::Float(value) => query.bind(value),
                SqlType::Text(value) => query.bind(value),
            };
        }

        query
    }
}
