use crate::config::Config;
use clap::Parser;

/// implements config init from cli arguments
impl Config {
    /// loads config from cli arguments
    #[must_use]
    pub fn from_cli<I>(args: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Config::parse_from(args)
    }
}
