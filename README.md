# Incredible Template

A multi-platform template for building TUI applications using the [Incredible](https://www.incredible.rs) framework.

This template is configured to build and package your application for four distinct platforms from a single codebase:

*   **Terminal**: A native binary that runs directly in your terminal. The version built depends on the platform on which you are running.
*   **Web**: Compiled to WASM and hosted as a static site with setup ready for GitHub Pages.
*   **macOS**: A double-clickable native GUI application.
*   **Windows**: A double-clickable native GUI application.

## Getting Started
> Make sure you meet [development environment prerequisites](markdowns/DEVELOPMENT_PREREQUISITES.md).
0) Make sure git can authenticate with the private [incredible-alpha](markdowns/DEVELOPMENT_PREREQUISITES.md#github-authentication) repository (`gh auth login`).
1) Build the tools.
2) Run config.
3) Develop the app.
4) Package & Publish from development environment.
5) Publish with GitHub Actions.

## Build Tools

From the root of the repo, build all three tools (config, package, run):
```bash
cargo build-tools
```

Then copy the freshly built binaries to the project root:

**macOS:**
```bash
cp target/release/config target/release/package target/release/run .
chmod +x config package run
```

**Windows (PowerShell):**
```powershell
copy target\release\config.exe .
copy target\release\package.exe .
copy target\release\run.exe .
```

The tools are then executed from the project root (`./config`, `./package`, `./run` on macOS; `config.exe`, `package.exe`, `run.exe` on Windows).

## Config App

From the root of the repo:
```bash
./config
```

Update fields as needed for the app being developed. Note: config can be run at any time to change values.

For full documentation of all CLI flags, validation rules, and the list of files the config tool updates, see [`tools/config/README.md`](tools/config/README.md).

Note: to change **Icon**: Replace `web/favicon.svg` with your own SVG logo. The macOS and Windows bundle scripts will automatically regenerate the application icon during the next build.

## Develop App

Code for the app is at `src/`.

It contains files designated for application development: `app.rs` and `platform.rs` and a set of preconfigured gated entry points that are conditionally compiled based on the target platform. Only one is used per build. Generally there is no need to modify these.

App Code:
```
src/
├── ui/
│   └── app.rs    # Cross-platform application code.
└── platform.rs   # Platform specific code required by app.
```

```
src/
├── main.rs      # Terminal entry point - runs as a native TUI binary
├── lib.rs       # WASM entry point - exports `main()` for web builds
├── macos.rs     # macOS native entry point - runs the app as a native GUI
├── windows.rs   # Windows native entry point - runs the app as a native GUI
├── runtime.rs   # Shared runtime - initializes platform and runs the app
```

From the root of the repo use: `./run` (or `./run.exe` on Windows) to launch an interactive selection menu or pass platform flags:

```bash
./run                   # Opens interactive selection UI
./run terminal          # Build/run for terminal target
./run wasm              # Build/run for web target (build + static serve)
./run macos             # Build/run for macOS native target
./run windows           # Build/run for Windows native target
```

For full documentation of the run tool, see [`tools/run/README.md`](tools/run/README.md).

## Package & Publish

From the root of the repo:

```bash
./package
```

Opens the interactive package tool UI. Select your target (All, Terminal, Web, macOS Native) and options (Clean, Bundle, Publish, Preview site), then press Enter or click to run.

For headless usage (no UI):

```bash
./package --all            # Packages all targets
./package --terminal       # Packages terminal CLI target
./package --wasm           # Builds WASM + site (production)
./package --macos          # Packages macOS native binary
./package --windows        # Packages Windows native binary
./package --clean --all    # Cleans build output then packages all
./package --publish        # Packages and publishes to GitHub release
```

For full documentation of the package tool, see [`tools/package/README.md`](tools/package/README.md).

## GitHub Actions

This template ships with two preconfigured workflows (in `.github/workflows/`) that automate building and distributing your app on GitHub's servers. They overlap with what the **Package tool** does locally, but add cross-platform parallel builds and event-driven automation. See [Doing the same thing (mostly) with the package tool](#doing-the-same-thing-mostly-with-the-package-tool) below.

*   **Create Downloadable Binaries** (`downloadable_binaries.yml`) — builds and publishes downloadable binaries:
    *   The Terminal binary across four platforms: macOS (Apple Silicon), macOS (Intel), Windows, and Linux.
    *   The macOS native and Windows native GUI applications.
    *   The resulting `.zip` artifacts are attached to a GitHub Release when you publish one, or uploaded to a rolling `latest` build tag when run manually.

*   **Deploy to GitHub Pages** (`github_pages.yml`) — builds the WASM/web version and deploys it as a static site to GitHub Pages. It runs automatically on every push to `main` and can also be triggered manually.

> Note: this repo depends on the private **incredible-alpha** crate, so the workflows require a `INCREDIBLE_ALPHA` secret (a GitHub token with read access to that repo) configured under Settings > Secrets and variables > Actions.

### Workflows vs. package tool

Some, but not all, of what the **Create Downloadable Binaries** workflow does can also be done locally with the **Package tool**. `./package` builds and bundles the same targets for the platform you are running on, and `./package --publish` attaches the release assets to a GitHub Release. Note that the package tool builds only the platform it runs on, whereas GitHub Actions build the binaries for all platforms on separate runners in parallel.

## Download & Run with Docker

Build the Docker image:

```bash
docker build -t incredible_app_template .
```

Run the container:

```bash
docker run -it incredible_app_template
```

This downloads the latest release binary from GitHub and runs it inside the container.

---

*Fabriqué au Canada : Made in Canada 🇨🇦*
