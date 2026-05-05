# fledge-plugin-pet

A [fledge](https://github.com/CorvidLabs/fledge) plugin: a corvid companion that tracks your dev habits and evolves as you code.

Your pet is an ASCII crow powered by [corvid-pet](https://github.com/CorvidLabs/corvid-pet). It reacts to dev activity -- feed it commits, test runs, code reviews, lint passes, and deploys. It has moods, life stages, hunger, energy, happiness, and health stats that decay over time. Keep it fed with consistent dev work and watch it grow; neglect it and it gets sad.

## Install

```bash
fledge plugin install corvid-agent/fledge-plugin-pet
```

Requires a Rust toolchain (`cargo`) -- the plugin auto-builds on install via the `build` hook in `plugin.toml`.

## Usage

```bash
# Check on your pet (shows ASCII art + stats)
fledge pet

# Feed with dev activity
fledge pet feed commit    # success event
fledge pet feed test      # progress event
fledge pet feed review    # success event
fledge pet feed lint      # progress event
fledge pet feed deploy    # success event

# Play with your pet
fledge pet play

# Rename your corvid
fledge pet rename Raven

# Hatch a fresh egg
fledge pet reset
```

Use `--name <NAME>` to manage multiple pets (default name is "Pip").

## How It Works

The plugin wraps the [corvid-pet](https://crates.io/crates/corvid-pet) crate's simulation system. Your pet has four core stats (hunger, energy, happiness, health) that decay passively over real time. Dev activity feeds it and keeps it healthy:

- **Success events** (commit, review, deploy) provide a big stat boost.
- **Progress events** (test, lint) provide moderate nourishment.
- **Play** increases happiness directly.

The pet progresses through life stages as it accumulates care. If critical needs go unmet, it will warn you.

## Integration Ideas

Auto-feed from a lane:

```toml
[lanes.ci]
steps = ["lint", "test", "pet-feed"]

[tasks.pet-feed]
cmd = "fledge pet feed test"
```

Or hook into git commits:

```bash
# .git/hooks/post-commit
fledge pet feed commit
```

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Requirements

- Rust toolchain (stable) -- builds automatically on `fledge plugin install`

## License

MIT
