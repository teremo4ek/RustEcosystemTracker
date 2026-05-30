# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

## Key Dependencies

- `tokio` — async runtime
- `reqwest` — HTTP client
- `rss` — RSS/Atom feed parsing
- `serde` + `toml` — config deserialization
- `anyhow` — error handling (simple, no custom error types)
- `chrono` — date parsing and formatting

## Commit Convention

Semantic, content-driven: `docs(async): add ...`, `crates(cache): add ...`, `digest: add ...`.

## Language

Code and commit messages in English. Project concept doc (`proj.txt`) is in Russian.
