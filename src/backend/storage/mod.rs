pub use course_store::*;

mod course_store;

#[derive(Clone, PartialEq, Debug)]
pub enum StorageConfigMode {
    Production,
    Sample,
}
#[derive(Clone)]
pub struct StorageConfig {
    mode: StorageConfigMode,
}

impl StorageConfig {
    pub fn new(storage_mode: String) -> Self {
        StorageConfig {
            mode: {
                match storage_mode.as_str() {
                    "PROD" => StorageConfigMode::Production,
                    "SAMPLE" => StorageConfigMode::Sample,
                    _ => unimplemented!(),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::storage::{StorageConfig, StorageConfigMode};

    #[test]
    fn test_prod_value() {
        assert_eq!(
            StorageConfig::new(String::from("PROD")).mode,
            StorageConfigMode::Production
        );
    }

    #[test]
    fn test_non_prod_value() {
        assert_eq!(
            StorageConfig::new(String::from("SAMPLE")).mode,
            StorageConfigMode::Sample
        );
    }
}
