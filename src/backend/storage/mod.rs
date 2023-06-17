pub use course_store::*;

mod course_store;

#[derive(Clone)]
pub struct StorageConfig {
    query_path: String,
}

impl StorageConfig {
    pub fn new(storage_mode: String) -> Self {
        StorageConfig { query_path: {
            match storage_mode.as_str() {
                "PROD" => "src/backend/storage/prod_queries".to_string(),
                _ => "src/backend/storage/sample_queries".to_string(),
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::storage::StorageConfig;

    #[test]
    fn test_prod_value() {
        assert_eq!(StorageConfig::new(String::from("PROD")).query_path, "/src/backend/storage/prod_queries");
    }

    #[test]
    fn test_non_prod_value() {
        assert_eq!(StorageConfig::new(String::from("SAMPLE")).query_path, "/src/backend/storage/sample_queries");
    }
}