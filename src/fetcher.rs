use crate::config::FeedSource;

pub struct FeedItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: Option<String>,
}

pub async fn fetch_feed(source: &FeedSource) -> anyhow::Result<Vec<FeedItem>> {
    let response = reqwest::get(&source.url).await?;
    let body = response.bytes().await?;
    let channel = rss::Channel::read_from(&body[..])?;

    let items: Vec<FeedItem> = channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap_or("Untitled").to_string(),
            link: item.link().unwrap_or("").to_string(),
            description: item.description().map(|s| s.to_string()),
            pub_date: item.pub_date().map(|s| s.to_string()),
        })
        .collect();

    Ok(items)
}
