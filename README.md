# Incredible Flappy

A Flappy Bird clone TUI game built with the [Incredible](https://www.incredible.rs) Rust framework.

Single codebase targeting terminal, web (WASM), macOS native, and Windows native.

## Status

Stub — gameplay under development.

## Run

```bash
./run terminal   # terminal TUI
./run wasm       # web (build + serve)
./run macos      # macOS native GUI
./run windows    # Windows native GUI
```

Or via Docker:

```bash
docker build -t incredible_flappy .
docker run -it incredible_flappy
```

## Development

See [`markdowns/DEVELOPMENT.md`](markdowns/DEVELOPMENT.md) for developer setup, tools (`config`, `run`, `package`), and publishing.
Prerequisites: [`markdowns/DEVELOPMENT_PREREQUISITES.md`](markdowns/DEVELOPMENT_PREREQUISITES.md).

---

*Fabriqué au Canada : Made in Canada 🇨🇦*
