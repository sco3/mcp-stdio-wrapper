use crate::config::Config;
/// implements config init
impl Config {
    /// loads config from env vars
    #[allow(dead_code)]
    #[must_use]
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env()
    }
}
