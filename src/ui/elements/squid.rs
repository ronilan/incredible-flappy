use incredible::*;
use incredible_elements::Rotator;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const SQUID_WIDTH: usize = 4;
pub const SQUID_HEIGHT: usize = 4;
pub const SQUID_INTERVAL_MS: u128 = 500;

// Classic squid frames, 8x8 pixels ('.' empty, 'X' filled).
// Rendered 2x2 pixels per cell through quadrant blocks.
const FRAME_A: [&str; 8] = [
    "...XX...",
    "..XXXX..",
    ".XXXXXX.",
    "XX.XX.XX",
    "XXXXXXXX",
    "..X..X..",
    ".X.XX.X.",
    "X.X..X.X",
];

const FRAME_B: [&str; 8] = [
    "...XX...",
    "..XXXX..",
    ".XXXXXX.",
    "XX.XX.XX",
    "XXXXXXXX",
    ".X.XX.X.",
    "X......X",
    ".X....X.",
];

/// Maps one 2x2 pixel cell (top-left, top-right, bottom-left, bottom-right)
/// to its quadrant block character.
fn quadrant(tl: bool, tr: bool, bl: bool, br: bool) -> char {
    match (tl, tr, bl, br) {
        (false, false, false, false) => ' ',
        (true, false, false, false) => '▘',
        (false, true, false, false) => '▝',
        (true, true, false, false) => '▀',
        (false, false, true, false) => '▖',
        (false, false, false, true) => '▗',
        (false, false, true, true) => '▄',
        (true, false, false, true) => '▚',
        (false, true, true, false) => '▞',
        (true, false, true, false) => '▌',
        (false, true, false, true) => '▐',
        (true, true, true, false) => '▛',
        (true, true, false, true) => '▜',
        (true, false, true, true) => '▙',
        (false, true, true, true) => '▟',
        (true, true, true, true) => '█',
    }
}

/// Converts an X/. pixel bitmap into quadrant-block rows.
fn bitmap_to_quadrants(rows: &[&str]) -> String {
    let height = rows.len();
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let get = |x: usize, y: usize| rows.get(y).and_then(|r| r.as_bytes().get(x)).is_some_and(|&b| b == b'X');
    let mut out = String::new();
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            out.push(quadrant(get(x, y), get(x + 1, y), get(x, y + 1), get(x + 1, y + 1)));
        }
        out.push('\n');
    }
    out
}

fn squid_frame(rows: &[&str], color: u8) -> Look {
    let look = Look::from(bitmap_to_quadrants(rows).as_str());
    for row in look.blocks().iter() {
        for block in row.iter() {
            block.decor.color.set(Some(Color::Ansi(color)));
        }
    }
    look
}

#[derive(Clone, Debug)]
pub struct SquidOptions {
    pub color: u8,
}

impl Default for SquidOptions {
    fn default() -> Self {
        Self {
            color: theme::SQUID_COLOR,
        }
    }
}

element! {
  pub struct Squid<S> {
      options: SquidOptions = SquidOptions::default(),
  }
}

impl<S: Clone + PartialEq> Squid<S> {
    pub fn new(options: SquidOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let rot = Rotator::<S>::new();
        rot.frames(vec![
            squid_frame(&FRAME_A, el.options.color),
            squid_frame(&FRAME_B, el.options.color),
        ])
        .interval(SQUID_INTERVAL_MS)
        .handle("squid_rotator")
        .x(el.get_x())
        .y(el.get_y());
        el.look(Look::from((SQUID_WIDTH, SQUID_HEIGHT, ' ')))
            .handle("squid");
        el.add(rot);

        el
    }
}

impl<S: Clone + PartialEq> Default for Squid<S> {
    fn default() -> Self {
        Self::new(SquidOptions::default())
    }
}
