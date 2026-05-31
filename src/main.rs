mod config;
mod fetcher;
mod llm;
mod state;
mod storage;

use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::load("config.toml")?;

    println!("Fetching: {}", config.feed_url);
    let result = fetcher::fetch_feed(&config.feed_url).await?;
    println!("  → {} entries fetched", result.entries.len());

    let state_path = Path::new("output/.state.json");
    let mut feed_state = state::FeedState::load(state_path)?;

    if !feed_state.has_changed(&result.feed_updated) {
        println!("Feed unchanged. Nothing to do.");
        return Ok(());
    }

    storage::write_entries(&result.entries, &config.llm).await?;

    feed_state.last_updated = Some(result.feed_updated.to_rfc3339());
    feed_state.save(state_path)?;

    println!("Done.");
    Ok(())
}
