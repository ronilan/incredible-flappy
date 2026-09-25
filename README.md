# Incredible Flappy

Flappy Bird for your terminal, with three ways to play: dodge the pipes, graze the flowers, or shoot the invaders.

Inspired by Impossible Flappy (parts lifted verbatim).

It's written in [Rust](https://www.rust-lang.org/) using the [Incredible](https://www.incredible.rs/) TUI framework.

<p align=center><img src="./media/social.png" alt="banner" width="640" style="border: 1px solid #999; border-radius: 5px"/></p>

# Install

## Pre Built Binaries

Pre built binaries are provided for each [release](https://github.com/ronilan/incredible-flappy/releases).

## Linux via Docker

To try the Linux terminal version, build and run:

```
docker build -t incredible_flappy .
docker run --rm -it incredible_flappy
```

Type `incredible_flappy` in the container shell to launch.

## TUI Install / Uninstall

Installs the latest release binary — `/usr/local/bin` (macOS/Linux) or `C:\Program Files\incredible-flappy` (Windows).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ronilan/incredible-flappy/main/install.sh | bash
```

```powershell
irm https://raw.githubusercontent.com/ronilan/incredible-flappy/main/install.ps1 | iex
```

Uninstall the same way with `uninstall.sh` / `uninstall.ps1`. If the release lookup fails, pass the binary name explicitly: `bash uninstall.sh <binary-name>` / `uninstall.ps1 -BinName <name>`.

# Play

Pick a game on the splash screen with Tab, then Enter to start. Clicking a game starts it directly. The ← button (top left) goes back to splash.

- **BASIC** — classic flappy. Thread the pipe gaps, one point per gap.
- **GRAZE** — flowers grow instead of some pipes. Land on a bud for a point, but bumping one kills you. Passing flowers pays nothing.
- **SHOOT** — pipes plus roaming alien pairs. Every boost also fires a bullet: kills score, pipes swallow bullets.

## Controls

- Enter / Space / click — flap (and shoot in SHOOT)
- K — kitty mode (cats instead of birds, all titles and buttons swap to art)
- Esc — pause in flight, Enter resumes

## Game over

The score panel slides up with your score, your all-time best, and a medal (🥇🥈🥉) if the run placed top 3 — 💩 otherwise.

## Files

Settings (kitty mode) and best scores are kept in a `.incredible-flappy` file under the OS app-data directory:

- macOS: `~/Library/Application Support/incredible-flappy/.incredible-flappy`
- Linux: `~/.local/share/incredible-flappy/.incredible-flappy` (or `$XDG_DATA_HOME/incredible-flappy/.incredible-flappy`)
- Windows: `C:\Users\<you>\AppData\Roaming\incredible-flappy\.incredible-flappy`

# Development

See [Development](./markdowns/DEVELOPMENT.md) and [Development Environment Prerequisites](./markdowns/DEVELOPMENT_PREREQUISITES.md)

---

*Fabriqué au Canada : Made in Canada 🇨🇦*
