mod config;
mod fetcher;
mod llm;
mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::load("config.toml")?;

    for feed in &config.feeds {
        println!("Fetching: {} ({})", feed.name, feed.url);
        let items = fetcher::fetch_feed(feed).await?;
        println!("  → {} items fetched", items.len());
        storage::write_daily(&feed.name, &items, &config.llm).await?;
    }

    println!("Done.");
    Ok(())
}
