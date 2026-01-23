use std::ffi::OsString;
use crate::config::Config;
use clap::Parser;

/// implements config init from cli arguments
impl Config {
    /// loads config from cli arguments
    #[must_use]
    pub fn from_cli<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // clap's parse_from expects Iterator<Item = T> where T: Into<OsString>
        Config::parse_from(args)
    }
}
