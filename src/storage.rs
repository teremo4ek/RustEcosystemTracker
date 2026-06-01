use std::fs;
use std::io::Write;
use std::path::Path;

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

pub async fn write_entries(entries: &[AtomEntry], llm_config: &LlmConfig, summarize: bool) -> anyhow::Result<()> {
    let output_dir = Path::new("output/daily");
    fs::create_dir_all(output_dir)?;

    for entry in entries {
        let date = entry.published.date_naive();
        let slug = title_to_slug(&entry.title);
        let filename = output_dir.join(format!("{}-{}.md", date, slug));

        // Skip already processed entries
        if filename.exists() {
            continue;
        }

        let mut file = fs::File::create(&filename)?;

        writeln!(file, "# {}\n", entry.title)?;
        writeln!(file, "- **Link:** {}", entry.link)?;
        writeln!(file, "- **Published:** {}", entry.published.date_naive())?;
        writeln!(file, "- **Updated:** {}", entry.updated.date_naive())?;
        if let Some(ref author) = entry.author {
            writeln!(file, "- **Author:** {}", author)?;
        }
        writeln!(file)?;

        let summary = if summarize {
            let summary_input = match &entry.content_html {
                Some(html) => html.clone(),
                None => entry.title.clone(),
            };

            writeln!(file, "## summary_input\n")?;
            writeln!(file, "{}\n", summary_input)?;

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

        writeln!(file, "## Summary\n")?;
        writeln!(file, "{}\n", summary)?;
    }

    Ok(())
}
