# Open Arcanum

A Rust-powered RPG character generation engine with a REST API, CLI, and web interface. Open Arcanum supports multiple tabletop RPG systems and is designed for both human game masters and AI agent integration.

## Features

- **Multi-System Support** — Character generation for D&D 1st, 2nd, and 3rd Edition (extensible to other systems)
- **REST API** — HTTP API for character generation, validation, CRUD operations, and export
- **Web UI** — Browser-based interface for generating and viewing characters
- **CLI Tool** — Full-featured command-line interface for local and remote operations
- **Character Validation** — Rule-aware validation against RPG system constraints
- **Export/Import** — JSON, YAML, Markdown, and OGC format support
- **Agent Integration** — SDK for programmatic access and persona generation for AI roleplay
- **Persistent Storage** — SQLite backend with optional in-memory mode
- **Dockerized** — Ready-to-run container with health checks

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.78+
- (Optional) Docker & Docker Compose

### Run the Server

```bash
# Build and run the server
cargo run --bin openarcanum-server

# Server starts on http://localhost:3000
```

### Use the CLI

```bash
# List available game systems
cargo run --bin openarcanum -- systems

# Generate a D&D 2nd Edition character
cargo run --bin openarcanum -- generate dnd2e --name "Elara" --rolled

# Validate a character sheet
cargo run --bin openarcanum -- validate ./character.json --system dnd2e

# Export to Markdown
cargo run --bin openarcanum -- export ./character.json --format markdown --output ./character.md

# Start interactive chat with a character
cargo run --bin openarcanum -- chat ./character.json
```

### Docker

```bash
docker-compose up -d
```

The server will be available at `http://localhost:3000`.

## API

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/api/v1/systems` | List available game systems |
| `GET` | `/api/v1/systems/:id` | Get system definition |
| `POST` | `/api/v1/systems/:id/generate` | Generate a character |
| `POST` | `/api/v1/systems/:id/validate` | Validate a character sheet |
| `POST` | `/api/v1/characters` | Create a character |
| `GET` | `/api/v1/characters` | List all characters |
| `GET` | `/api/v1/characters/:id` | Get a character |
| `DELETE` | `/api/v1/characters/:id` | Delete a character |
| `POST` | `/api/v1/characters/:id/export` | Export a character |
| `GET` | `/api/v1/schemas` | List JSON schemas |
| `GET` | `/api/v1/schemas/:name` | Get a specific schema |

### Example: Generate a Character

```bash
curl -X POST http://localhost:3000/api/v1/systems/dnd2e/generate \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Elara",
    "options": {
      "use_rolled_stats": true,
      "starting_level": 1
    }
  }'
```

### Example: Using the SDK

```rust
use oa_sdk::OpenArcanumClient;

let client = OpenArcanumClient::new("http://localhost:3000");
let result = client
    .generate_character("dnd2e", None, None)
    .await?;

println!("Generated: {}", result.character.name);
```

## Project Structure

This is a Rust workspace with the following crates:

| Crate | Description |
|-------|-------------|
| `oa-core` | Core domain types, traits, and character model |
| `oa-rules` | RPG system rule engines (D&D 1e/2e/3e) |
| `oa-server` | Axum-based HTTP server and Web UI |
| `oa-cli` | Command-line interface |
| `oa-sdk` | Client SDK for agent/programmatic integration |

## CLI Commands

```
openarcanum <command> [options]

Commands:
  systems         List available game systems
  generate        Generate a new character
  validate        Validate a character sheet
  export          Export character to various formats
  import          Import character from external format
  persona         Create an agent persona from a character
  chat            Interactive chat with a character persona
  schema          Show JSON schema for a type
  serve           Start the local server

Global Options:
  -S, --server    Server URL for remote operations
  -k, --api-key   API key for authentication
  -f, --format    Output format (json, yaml, pretty)
```

## Configuration

Environment variables for the server:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | HTTP server port |
| `DATABASE_URL` | *(in-memory)* | SQLite database path |
| `RUST_LOG` | `info` | Log level filter |

## License

Dual-licensed under MIT OR Apache-2.0.
