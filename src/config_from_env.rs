use crate::config::Config;
/// implements config init
impl Config {
    /// loads config from env vars
    #[allow(dead_code)]
    #[must_use]
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let config = envy::from_env().unwrap_or_else(|e| {
            eprintln!("Config error: {e}");
            std::process::exit(1);
        });

        config
    }
}
