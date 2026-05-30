mod config;
mod fetcher;
mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load("config.toml")?;

    for feed in &config.feeds {
        println!("Fetching: {} ({})", feed.name, feed.url);
        let items = fetcher::fetch_feed(feed).await?;
        println!("  → {} items fetched", items.len());
        storage::write_daily(&feed.name, &items)?;
    }

    println!("Done.");
    Ok(())
}
