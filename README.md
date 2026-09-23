# Incredible Flappy

Flappy Bird for your terminal (that also works on the web), with three ways to play: dodge the pipes, graze the flowers, or shoot the invaders.

It's written in [Rust](https://www.rust-lang.org/) using the [Incredible](https://www.incredible.rs/) TUI framework.

# Install

## Pre Built Binaries

Pre built binaries are provided for each [release](https://github.com/ronilan/incredible-flappy/releases).

Or via Docker:

```bash
docker build -t incredible_flappy .
docker run -it incredible_flappy
```

# Play

Pick a game on the splash screen with Left/Right (or Tab), then Enter to start. Clicking a game starts it directly. Esc always takes you back.

- **BASIC** — classic flappy. Thread the pipe gaps, one point per gap.
- **GRAZE** — flowers grow instead of some pipes. Land on a bud for a point, but bumping one kills you. Passing flowers pays nothing.
- **SHOOT** — pipes plus roaming alien pairs. Every boost also fires a bullet: kills score, pipes swallow bullets.

## Controls

- Enter / Space / click — flap (and shoot in SHOOT)
- K — kitty mode (cats instead of birds, all titles and buttons swap to art)
- Esc — back to splash

## Game over

The score panel slides up with your score, your all-time best, and a medal (🥇🥈🥉) if the run placed top 3. Best scores persist per game between sessions.

# Development

See [`markdowns/DEVELOPMENT.md`](markdowns/DEVELOPMENT.md) for developer setup, tools (`config`, `run`, `package`), and publishing.
Prerequisites: [`markdowns/DEVELOPMENT_PREREQUISITES.md`](markdowns/DEVELOPMENT_PREREQUISITES.md).

---

*Fabriqué au Canada : Made in Canada 🇨🇦*
