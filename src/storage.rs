use std::collections::HashSet;
use std::fs;
use std::path::Path;

use chrono::{DateTime, FixedOffset};

use crate::config::LlmConfig;
use crate::fetcher::AtomEntry;

fn title_to_slug(title: &str) -> String {
    title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn parse_last_updated(content: &str) -> Option<DateTime<FixedOffset>> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(ts) = trimmed.strip_prefix("> Last updated:") {
            let ts = ts.trim();
            return DateTime::parse_from_rfc3339(ts).ok();
        }
    }
    None
}

fn extract_links(content: &str) -> HashSet<String> {
    let mut links = HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- **Link:**") {
            links.insert(rest.trim().to_string());
        }
    }
    links
}

fn strip_header(content: &str) -> &str {
    // Header is: # Title\n\n> Last updated: ...\n\n---\n\n
    // Return everything after the first "---" separator
    if let Some(pos) = content.find("\n---\n") {
        &content[pos + 5..]
    } else {
        content
    }
}

async fn format_entry(
    entry: &AtomEntry,
    llm_config: &LlmConfig,
    summarize: bool,
) -> anyhow::Result<String> {
    let mut out = String::new();

    out.push_str(&format!("## {}\n\n", entry.title));
    out.push_str(&format!("- **Link:** {}\n", entry.link));
    out.push_str(&format!("- **Published:** {}\n", entry.published.date_naive()));
    out.push_str(&format!("- **Updated:** {}\n", entry.updated.date_naive()));
    if let Some(ref author) = entry.author {
        out.push_str(&format!("- **Author:** {}\n", author));
    }
    out.push('\n');

    let summary = if summarize {
        let summary_input = match &entry.content_html {
            Some(html) => html.clone(),
            None => entry.title.clone(),
        };

        match crate::llm::summarize(llm_config, &summary_input).await {
            Ok(s) if !s.is_empty() => s,
            Ok(_) => "*Summary unavailable.*".to_string(),
            Err(e) => {
                eprintln!("  ⚠ LLM summary failed for '{}': {}", entry.title, e);
                "*Summary unavailable.*".to_string()
            }
        }
    } else {
        "*Summarization disabled.*".to_string()
    };

    out.push_str("## Summary\n\n");
    out.push_str(&summary);
    out.push_str("\n\n---\n\n");

    Ok(out)
}

pub async fn write_entries(
    feed_name: &str,
    entries: &[AtomEntry],
    feed_updated: &DateTime<FixedOffset>,
    llm_config: &LlmConfig,
    summarize: bool,
) -> anyhow::Result<bool> {
    let slug = title_to_slug(feed_name);
    let output_path = Path::new("output").join(format!("{}.md", slug));

    fs::create_dir_all("output")?;

    let existing_content = if output_path.exists() {
        Some(fs::read_to_string(&output_path)?)
    } else {
        None
    };

    let existing_links = existing_content
        .as_ref()
        .map(|c| extract_links(c))
        .unwrap_or_default();

    let new_entries: Vec<&AtomEntry> = entries
        .iter()
        .filter(|e| !existing_links.contains(&e.link))
        .collect();

    if new_entries.is_empty() && existing_content.is_some() {
        // No new entries — update timestamp if feed_updated is newer
        if let Some(ref content) = existing_content {
            let stored = parse_last_updated(content);
            let needs_update = stored.as_ref().map_or(true, |s| s < feed_updated);
            if needs_update {
                let header = format!(
                    "# {}\n\n> Last updated: {}\n\n---\n\n",
                    feed_name,
                    feed_updated.to_rfc3339()
                );
                let body = strip_header(content);
                fs::write(&output_path, format!("{}{}", header, body))?;
            }
        }
        return Ok(false);
    }

    // Build header
    let header = format!(
        "# {}\n\n> Last updated: {}\n\n---\n\n",
        feed_name,
        feed_updated.to_rfc3339()
    );

    // Build new entry sections
    let mut new_sections = String::new();
    for entry in &new_entries {
        new_sections.push_str(&format_entry(entry, llm_config, summarize).await?);
    }

    // Old body (without header)
    let old_body = existing_content
        .as_ref()
        .map(|c| strip_header(c))
        .unwrap_or("");

    let full_content = format!("{}{}{}", header, new_sections, old_body);
    fs::write(&output_path, full_content)?;

    Ok(true)
}
