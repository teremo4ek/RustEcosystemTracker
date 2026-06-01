mod config;
mod fetcher;
mod llm;
mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::load("config.toml")?;

    println!("Fetching: {}", config.feed_url);
    let result = fetcher::fetch_feed(&config.feed_url).await?;
    println!("  → {} entries fetched", result.entries.len());

    let wrote = storage::write_entries(
        &config.feed_name,
        &result.entries,
        &result.feed_updated,
        &config.llm,
        config.summarize,
    )
    .await?;

    if !wrote {
        println!("Feed unchanged. Nothing to do.");
    } else {
        println!("Done.");
    }

    Ok(())
}
