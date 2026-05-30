use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub feeds: Vec<FeedSource>,
}

#[derive(Debug, Deserialize)]
pub struct FeedSource {
    pub name: String,
    pub url: String,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
