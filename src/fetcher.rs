use chrono::{DateTime, FixedOffset};
use std::io::BufReader;

pub struct AtomEntry {
    pub title: String,
    pub link: String,
    pub published: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
    pub content_html: Option<String>,
    pub author: Option<String>,
}

pub struct FetchResult {
    pub feed_updated: DateTime<FixedOffset>,
    pub entries: Vec<AtomEntry>,
}

pub async fn fetch_feed(url: &str) -> anyhow::Result<FetchResult> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;

    let reader = BufReader::new(&body[..]);
    let feed = atom_syndication::Feed::read_from(reader)?;

    let feed_updated = *feed.updated();

    let entries: Vec<AtomEntry> = feed
        .entries()
        .iter()
        .map(|entry| {
            let link = entry
                .links()
                .iter()
                .find(|l| l.rel() == "alternate")
                .map(|l| l.href().to_string())
                .unwrap_or_else(|| entry.id().to_string());

            let published = entry
                .published()
                .copied()
                .unwrap_or_else(|| *entry.updated());

            let updated = *entry.updated();

            let content_html = entry
                .content()
                .and_then(|c| c.value().map(|v| v.to_string()));

            let author = entry
                .authors()
                .first()
                .map(|a| a.name().to_string());

            AtomEntry {
                title: entry.title().to_string(),
                link,
                published,
                updated,
                content_html,
                author,
            }
        })
        .collect();

    Ok(FetchResult {
        feed_updated,
        entries,
    })
}
