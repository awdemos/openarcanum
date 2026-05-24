# Contributing to Open Arcanum

Thank you for your interest in contributing! This project is a Rust workspace for tabletop RPG character generation.

## Getting Started

1. **Install Rust** — [rustup.rs](https://rustup.rs/) (1.78+ required)
2. **Clone the repo** and run `cargo build`
3. **Run tests** with `cargo test --workspace`

## Project Structure

- `oa-core` — Domain types, traits, and character models
- `oa-rules` — RPG system rule engines
- `oa-server` — Axum HTTP server and Web UI
- `oa-cli` — Command-line interface (`openarcanum` binary)
- `oa-sdk` — Client SDK for programmatic access

## Guidelines

- Follow `cargo fmt` and `cargo clippy`
- Add tests for new rules engine logic
- Update `README.md` if you change CLI or API behavior
- Keep commits atomic and messages descriptive

## Questions?

Open an issue or start a discussion. All contributions are welcome!
