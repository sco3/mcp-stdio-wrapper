use crate::config::Config;

impl Config {
    /// Loads the configuration from environment variables.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Required environment variables are missing
    /// * The values cannot be parsed
    #[allow(dead_code)]
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env()
    }
}
