# fledge-plugin-pet

A [fledge](https://github.com/CorvidLabs/fledge) plugin that gives you an ASCII corvid companion powered by [corvid-pet](https://github.com/CorvidLabs/corvid-pet).

Your pet reacts to dev activity — feed it commits, test runs, code reviews, and deploys. It has moods, life stages, hunger, energy, and happiness stats.

## Install

```bash
fledge plugin install corvid-agent/fledge-plugin-pet
```

Requires Rust toolchain (`cargo`) — the plugin auto-builds on install via the `build` hook.

## Usage

```bash
# Check on your pet
fledge pet

# Feed with dev activity
fledge pet feed commit
fledge pet feed test
fledge pet feed review
fledge pet feed lint
fledge pet feed deploy

# Play with your pet
fledge pet play

# Rename your corvid
fledge pet rename Raven

# Start fresh
fledge pet reset
```

## How It Works

Uses [corvid-pet](https://crates.io/crates/corvid-pet)'s simulation system — your pet has hunger, energy, happiness, and health stats that decay over time. Dev activity feeds it and keeps it healthy. Neglect it and it gets sad.

## Integration Ideas

Add to your lanes to auto-feed:

```toml
[lanes.ci]
steps = ["lint", "test", "pet-feed"]

[tasks.pet-feed]
cmd = "fledge pet feed test"
```

Or hook it into git:

```bash
# .git/hooks/post-commit
fledge pet feed commit
```

## Requirements

- Rust toolchain (cargo) — builds automatically on `fledge plugin install`

## License

MIT
