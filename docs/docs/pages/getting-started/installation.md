---
title: Installation
description: Install prebuilt binaries or build Temprano Watchtower from source.
---

# Installation

See the Dependencies page for required services and optional components like a reverse proxy or supervisor.

## Precompiled Binaries

Download the appropriate archive for your platform from the [GitHub releases page](https://github.com/arvina-tech/temprano-watchtower/releases), then:

1. Unpack the archive.
2. Place the `temprano-watchtower` binary somewhere on your `PATH` (or keep it alongside your deployment artifacts).

## Build From Source

This project targets Rust 2024 edition and can be built using the latest stable release.

Install straight from the GitHub repo in one step:

```bash
cargo install --git https://github.com/arvina-tech/temprano-watchtower.git --locked
```

Otherwise, clone the repo and build the binary from source. From the repository root:

```bash
cargo build --release
```

The binary will be at `target/release/temprano-watchtower`. You can run it directly:

```bash
./target/release/temprano-watchtower
```

Or install it into your Cargo bin directory:

```bash
cargo install --path .
```
