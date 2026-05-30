use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

use chrono::NaiveDate;

use crate::fetcher::FeedItem;

/// Parse an RFC 2822 date string (typical RSS format) into a NaiveDate.
/// Falls back to today's date on failure.
fn parse_date(pub_date: &Option<String>) -> NaiveDate {
    pub_date
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc2822(s).ok())
        .map(|dt| dt.date_naive())
        .unwrap_or_else(|| chrono::Local::now().date_naive())
}

/// Read an existing daily digest file and collect all links already present.
/// Returns an empty set if the file does not exist yet.
fn collect_existing_links(path: &Path) -> HashSet<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("- **Link:** ")
                .map(|url| url.to_string())
        })
        .collect()
}

pub fn write_daily(feed_name: &str, items: &[FeedItem]) -> anyhow::Result<()> {
    // Group items by publication date
    let mut grouped: BTreeMap<NaiveDate, Vec<&FeedItem>> = BTreeMap::new();
    for item in items {
        let date = parse_date(&item.pub_date);
        grouped.entry(date).or_default().push(item);
    }

    let output_dir = Path::new("output/daily");
    fs::create_dir_all(output_dir)?;

    for (date, date_items) in &grouped {
        let filename = output_dir.join(format!("{}.md", date));

        // Collect links already written to this file and filter out duplicates
        let existing_links = collect_existing_links(&filename);
        let new_items: Vec<&FeedItem> = date_items
            .iter()
            .filter(|item| !existing_links.contains(&item.link))
            .copied()
            .collect();

        // Skip entirely if all items are already present
        if new_items.is_empty() {
            continue;
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;

        write_section(&mut file, feed_name, date, &new_items)?;
    }

    Ok(())
}

fn write_section(
    mut file: &fs::File,
    feed_name: &str,
    date: &NaiveDate,
    items: &[&FeedItem],
) -> anyhow::Result<()> {
    // Write header only if file is empty (new file)
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        writeln!(file, "# Daily Rust Digest — {}\n", date)?;
    }

    writeln!(file, "## {}\n", feed_name)?;

    for item in items {
        writeln!(file, "### {}", item.title)?;
        writeln!(file)?;
        writeln!(file, "- **Link:** {}", item.link)?;
        if let Some(ref pub_date) = item.pub_date {
            writeln!(file, "- **Published:** {}", pub_date)?;
        }
        if let Some(ref desc) = item.description {
            writeln!(file, "- **Description:** {}", desc)?;
        }
        writeln!(file)?;
    }

    Ok(())
}
