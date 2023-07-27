use tokio::time;
use std::time::Duration;
use log::{debug, info, error, LevelFilter};
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

pub async fn synchronize() -> ! {   
    let mut iteration: u32 = 0;
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let config: Configuration = configuration();
    let db_pool = 
    sqlx::PgPool::connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL"))
        .await
        .expect("Unable to connect to the database");

    debug!("Initialized logger for synchronization task.");
    info!("PgPool details: {:?}", db_pool);

    loop {
        info!("Beginning data synchronization for DB. Iteration: {}", iteration);
        let result: Result<(), String> = generic::synchronize_data(&config, &db_pool).await;

        // If we ran into an error during synchronize, fail gracefully.
        match result {
            Ok(()) => (),
            Err(err) => error!("ERROR DURING DATA SYNC: {:?}", err)
        }
        info!("Ending data synchronization for DB. Iteration: {}", iteration);
        time::sleep(Duration::from_secs(120)).await;
        iteration += 1;
    }
}