# Rust Ecosystem Tracker

Automated daily knowledge-base builder for the Rust ecosystem. Fetches data from configurable RSS feeds, parses the content, and writes structured Markdown digests to disk.

## Features

- **Config-driven** — feed sources defined in `config.toml`, easy to extend
- **RSS parsing** — fetches and parses standard RSS feeds
- **Daily digests** — groups articles by publication date into `output/daily/YYYY-MM-DD.md`
- **Async pipeline** — built on Tokio

## Quick Start

```bash
# Build and run
cargo run

# Output appears in:
#   output/daily/2026-05-27.md
#   output/daily/2026-05-20.md
#   ...
```

## Configuration

Edit `config.toml` to add or remove feed sources:

```toml
[[feeds]]
name = "This Week in Rust"
url = "https://this-week-in-rust.org/rss.xml"

[[feeds]]
name = "Rust Blog"
url = "https://blog.rust-lang.org/feed.xml"
```

## Project Structure

```
src/
├── main.rs          # Entry point: config → fetch → write
├── config.rs        # Config/FeedSource structs, TOML deserialization
├── fetcher.rs       # HTTP fetch (reqwest) + RSS parsing → Vec<FeedItem>
└── storage.rs       # Group by date, write Markdown files
```

## Output Format

Each daily file contains formatted Markdown with article titles, links, publication dates, and descriptions:

```markdown
# Daily Rust Digest — 2026-05-27

## This Week in Rust

### This Week in Rust 653

- **Link:** https://this-week-in-rust.org/blog/2026/05/27/this-week-in-rust-653/
- **Published:** Wed, 27 May 2026 00:00:00 -0400
- **Description:** ...
```

## Roadmap

- More data sources (crates.io, GitHub releases, Rust RFCs)
- LLM-based summarization and tagging
- Knowledge graph with entity extraction
- Obsidian-style structured notes
- Automatic semantic commit messages

## Dependencies

| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| reqwest | HTTP client |
| rss | RSS feed parsing |
| serde + toml | Config deserialization |
| anyhow | Error handling |
| chrono | Date parsing |

## License

MIT
