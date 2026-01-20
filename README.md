# tempo-watchtower

Tempo Watchtower accepts signed Tempo transactions, stores them durably, and broadcasts them until they are mined or expire.

## Setup

- Ensure Postgres and Redis are running (see `.env` for defaults).
- Adjust `config.toml` for RPC endpoints. Environment variables in the file are expanded at startup.
- Optional: set `CONFIG_PATH` to point at a different config file.

## Running

```bash
cargo run
```

On startup the service runs database migrations automatically.
