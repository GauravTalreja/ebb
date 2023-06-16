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
