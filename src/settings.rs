use std::fs;
use std::path::PathBuf;

use crate::ui::state::State;

#[cfg(not(target_arch = "wasm32"))]
const DATA_DIR: &str = "incredible-flappy";
#[cfg(not(target_arch = "wasm32"))]
const DATA_FILE: &str = ".incredible-flappy";

/// Persisted settings and best scores. Bests are top-3 tuples,
/// highest first.
#[derive(Clone, Debug)]
pub struct Persisted {
    pub kitty: bool,
    pub best_classic: [u32; 3],
    pub best_busy: [u32; 3],
    pub best_invaders: [u32; 3],
}

impl Default for Persisted {
    fn default() -> Self {
        Self {
            kitty: false,
            best_classic: [0; 3],
            best_busy: [0; 3],
            best_invaders: [0; 3],
        }
    }
}

impl Persisted {
    fn parse_content(content: &str) -> Self {
        let parts: Vec<&str> = content.trim().split(',').collect();
        // Backward compatibility: the old format held one best per game.
        if parts.len() == 4 {
            let num = |i: usize| {
                parts
                    .get(i)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0)
            };
            return Self {
                kitty: parts.first().is_some_and(|s| *s == "true"),
                best_classic: [num(1), 0, 0],
                best_busy: [num(2), 0, 0],
                best_invaders: [num(3), 0, 0],
            };
        }
        let top = |at: usize| {
            let mut scores = [0; 3];
            for (i, slot) in scores.iter_mut().enumerate() {
                *slot = parts
                    .get(at + i)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);
            }
            scores.sort_unstable_by(|a, b| b.cmp(a));
            scores
        };
        Self {
            kitty: parts.first().is_some_and(|s| *s == "true"),
            best_classic: top(1),
            best_busy: top(4),
            best_invaders: top(7),
        }
    }

    fn content(&self) -> String {
        let flat = |top: &[u32; 3]| format!("{},{},{}", top[0], top[1], top[2]);
        format!(
            "{},{},{},{}",
            self.kitty,
            flat(&self.best_classic),
            flat(&self.best_busy),
            flat(&self.best_invaders)
        )
    }
}

/// Per-user data file location, following platform conventions
/// (macOS: ~/Library/Application Support, Linux: $XDG_DATA_HOME,
/// Windows: %APPDATA%). `None` when no base data directory can be
/// determined (e.g. HOME is unset).
#[cfg(not(target_arch = "wasm32"))]
fn data_file_path() -> Option<PathBuf> {
    Some(dirs::data_dir()?.join(DATA_DIR).join(DATA_FILE))
}

/// Native file persistence does not exist on the web, so there is
/// never a data file path.
#[cfg(target_arch = "wasm32")]
fn data_file_path() -> Option<PathBuf> {
    None
}

/// Loads persisted settings and bests. Defaults when missing or unreadable.
pub(crate) fn load() -> Persisted {
    match data_file_path().and_then(|path| fs::read_to_string(path).ok()) {
        Some(content) => Persisted::parse_content(&content),
        None => Persisted::default(),
    }
}

/// Applies persisted settings and bests onto a fresh state.
pub(crate) fn apply_to_state(data: &Persisted, state: &mut State) {
    state.kitty = data.kitty;
    state.best_classic = data.best_classic;
    state.best_busy = data.best_busy;
    state.best_invaders = data.best_invaders;
}

/// Writes current settings and bests. Silent no-op when the path
/// cannot be determined or created.
pub(crate) fn persist_now(state: &State) {
    let Some(path) = data_file_path() else { return };
    let data = Persisted {
        kitty: state.kitty,
        best_classic: state.best_classic,
        best_busy: state.best_busy,
        best_invaders: state.best_invaders,
    };
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return;
        }
    }
    let _ = fs::write(&path, data.content());
}
