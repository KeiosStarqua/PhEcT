# PhEcT

A tiny, minimalist always-on-top desktop widget that shows live [`hc`](https://herdr-core) terminals at a glance.

Built with Rust + [Slint](https://slint.rs/).

## What it shows

- One row per live terminal from `hc ls`: status dot, name/address, cwd, agent · status
- Status colors: working (amber), done (green), blocked (orange), error (red), idle (gray)
- Auto-refreshes every 2 seconds; click ↻ to refresh now

## Usage

```sh
cargo run
```

Requires `hc` on `PATH`. If `hc` is missing or fails, the widget shows an error state instead of crashing.
