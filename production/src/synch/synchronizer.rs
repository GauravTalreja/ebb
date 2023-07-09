use tokio::time;
use std::time::Duration;
use log::{debug, info, LevelFilter};
use simple_logger::SimpleLogger; 
use super::generic;
use openapi::apis::configuration::{ApiKey, Configuration};

pub fn configuration() -> Configuration {
    Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: std::env::var("API_KEY").expect("API_KEY not found"),
        }),
        ..Default::default()
    }
}


#[allow(unused_variables)]
pub async fn synchronize() -> ! {   
    let mut iteration: u32 = 0;
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let config: Configuration = configuration();
    let synch_time_hrs: u64 = std::env::var("SYNCH_TIME_HOURS").unwrap_or("1".to_string()).parse().unwrap();
    let db_pool = 
    sqlx::PgPool::connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL"))
        .await
        .expect("Unable to connect to the database");

    debug!("Initialized logger for synchronization task.");
    info!("SYNCH_TIME_HOURS: {}", synch_time_hrs);
    info!("PgPool details: {:?}", db_pool);

    loop {
        info!("Beginning data synchronization for db. Iteration: {}", iteration);
        generic::synchronize_data(&config, &db_pool).await;
        time::sleep(Duration::from_secs(120)).await;
        iteration += 1;
    }
}