use std::fs;
use std::net::SocketAddr;
use std::num::{NonZeroU16, NonZeroUsize};
use std::path::Path;

use clap::Parser;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::config::ConfigError;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct AppArgs {
    #[clap(short = 'c', long = "config-file", required = true)]
    pub config_file_path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub database: String,
    pub port: NonZeroU16,
    pub pool_size: NonZeroUsize,
}

pub fn read<T>(config_path: &Path) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
{
    let config_toml = fs::read_to_string(config_path).map_err(ConfigError::FileReadFailure)?;
    let config: T = toml::from_str(&config_toml).map_err(ConfigError::InvalidToml)?;
    Ok(config)
}
