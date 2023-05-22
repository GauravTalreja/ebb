//! Reads and loads `dosguard` system configurations.

use std::path::Path;
use std::{fs, io};

use toml::de;

pub use reader::*;

mod reader;

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("failed to read ebb.toml file from disk: {0:?}")]
    FileReadFailure(io::Error),
    #[error("failed to parse ebb.toml file: {0:?}")]
    InvalidToml(de::Error),
}
