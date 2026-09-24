use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const BUILDINGS_STR: &str = "   |^^^|   _     ___          \n __|:::|__| |___|:::|         \n|''|:::|oo|.|* *|:::|         \n|''|:::|oo|.|** |:::|         \n|_'|___|_o|_|[]_|_|_|   ";

pub const BUILDINGS_WIDTH: usize = 80;

#[derive(Clone, Debug)]
pub struct BuildingsOptions {
    pub color: u8,
    pub width: usize,
}

impl Default for BuildingsOptions {
    fn default() -> Self {
        Self {
            color: theme::BUILDINGS_COLOR,
            width: BUILDINGS_WIDTH,
        }
    }
}

/// Repeats the skyline pattern across the given width, padding
/// ragged rows to a uniform period first so storeys stay aligned.
fn tiled_str(width: usize) -> String {
    let rows: Vec<&str> = BUILDINGS_STR.lines().collect();
    let period = rows.iter().map(|r| r.len()).max().unwrap_or(1).max(1);
    rows.iter()
        .map(|r| {
            let padded = format!("{r:<period$}");
            padded.chars().cycle().take(width).collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

element! {
  pub struct Buildings<S> {
      options: BuildingsOptions = BuildingsOptions::default(),
  }
}

impl<S: Clone + PartialEq> Buildings<S> {
    pub fn new(options: BuildingsOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from(tiled_str(el.options.width).as_str()))
            .color(Some(Color::Ansi(el.options.color)))
            .handle("buildings");


        el
    }
}

impl<S: Clone + PartialEq> Default for Buildings<S> {
    fn default() -> Self {
        Self::new(BuildingsOptions::default())
    }
}
