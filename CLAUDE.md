# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust Ecosystem Tracker — an automated knowledge-base builder for the Rust ecosystem. It fetches data from configurable RSS feeds, parses the content, and writes structured Markdown digests to disk.

## Commands

```bash
cargo build          # compile
cargo run            # fetch feeds from config.toml and write output/daily/*.md
cargo test           # run tests (none yet)
```

## Architecture

```
src/main.rs          → entry point: loads config, iterates feeds, calls fetcher then storage
src/config.rs        → Config / FeedSource structs, loads from config.toml via serde+toml
src/fetcher.rs       → async HTTP fetch (reqwest) + RSS parsing (rss crate) → Vec<FeedItem>
src/storage.rs       → groups FeedItems by pub_date, writes output/daily/YYYY-MM-DD.md
```

Processing pipeline: `config.toml → fetch_feed() → write_daily() → output/daily/`

## Config Format

`config.toml` uses TOML with `[[feeds]]` array:

```toml
[[feeds]]
name = "This Week in Rust"
url = "https://this-week-in-rust.org/rss.xml"
```

## Output

`output/daily/` — one Markdown file per publication date, append mode. Files are gitignored (auto-generated).

## Key Dependencies

- `tokio` — async runtime
- `reqwest` — HTTP client
- `rss` — RSS/Atom feed parsing
- `serde` + `toml` — config deserialization
- `anyhow` — error handling (simple, no custom error types)
- `chrono` — date parsing and formatting

## Commit Convention

Semantic, content-driven: `docs(async): add Tokio scheduler redesign article`, `crates(cache): add rust-cache v0.3`, `digest: add tokio scheduler, sqlx update and RFC 3812`.

## Language

Code and commit messages in English. Knowledge base content (markdown output) in English. Project concept doc (`proj.txt`) is in Russian.
