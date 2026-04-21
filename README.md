# fledge-plugin-pet

A [fledge](https://github.com/CorvidLabs/fledge) plugin that gives you a corvid companion. Feed it with good dev habits and watch it grow.

Built as a shell script — no compilation needed.

## Install

```bash
fledge plugin install corvid-agent/fledge-plugin-pet
```

## Usage

```bash
# Check on your pet
fledge pet

# Feed it with dev activities
fledge pet feed commit     # +10 XP
fledge pet feed test       # +5 XP
fledge pet feed review     # +15 XP

# Rename your corvid
fledge pet rename Raven

# Reset (new egg)
fledge pet reset
```

## Evolution

Your corvid evolves as it levels up:

| Level | Stage | Art |
|-------|-------|-----|
| 1 | Hatchling | Tiny chick |
| 2-4 | Fledgling | Young bird |
| 5-9 | Corvid | Full-grown crow |
| 10+ | Elder Corvid | Majestic elder |

## How It Works

Your pet gains XP when you feed it. Every level requires `level * 50` XP. State is stored in `~/.local/state/fledge-pet/pet.json`.

The mood system reacts to how often you interact — leave it too long and it gets lonely!

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

- bash, python3 (for JSON state management)

## License

MIT
