# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # compile
cargo run            # fetch Atom feed from config.toml and write output/daily/*.md
cargo test           # run tests (none yet)
```

## Architecture

```
src/main.rs          → entry point: loads config, fetches feed, checks state, writes entries
src/config.rs        → Config / LlmConfig structs, loads from config.toml via serde+toml
src/fetcher.rs       → async HTTP fetch (reqwest) + Atom parsing (atom_syndication) → FetchResult
src/state.rs         → FeedState: persists last <updated> in output/.state.json for change detection
src/storage.rs       → writes one MD file per AtomEntry to output/daily/{date}-{slug}.md
src/llm.rs           → OpenAI-compatible API call for summarization
```

Processing pipeline: `config.toml → fetch_feed() → state check → write_entries() → save state`

## Key Dependencies

- `tokio` — async runtime
- `reqwest` — HTTP client
- `atom_syndication` — Atom feed parsing
- `serde` + `toml` — config deserialization
- `anyhow` — error handling (simple, no custom error types)
- `chrono` — date parsing and formatting

## Language

Code and commit messages in English. Documentation in English.
