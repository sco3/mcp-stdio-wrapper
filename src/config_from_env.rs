use crate::config::Config;
use dotenvy::dotenv;
use std::env;
/// implements config init
impl Config {
    /// loads config from env vars
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        envy::from_env().expect("Failed to load config.")
    }
}
