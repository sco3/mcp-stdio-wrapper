use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    10
}
