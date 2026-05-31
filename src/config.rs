use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub feed_url: String,
    #[serde(default = "default_feed_name")]
    pub feed_name: String,
    pub llm: LlmConfig,
}

#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key_env: String,
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_feed_name() -> String {
    "Rust Blog".to_string()
}

fn default_max_tokens() -> u32 {
    500
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
