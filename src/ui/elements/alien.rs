use std::cell::Cell;

use incredible::*;
use incredible_elements::Rotator;
use incredible_macros_decl::element;
use rand::Rng;
use rand::rng;

use crate::ui::theme;

pub const ALIEN_HEIGHT: usize = 4;
pub const ALIEN_INTERVAL_MS: u128 = 500;
// Vertical roam: sky row 0 down to bottom row on 20.
pub const ALIEN_MIN_Y: isize = 0;
pub const ALIEN_MAX_Y: isize = 21 - ALIEN_HEIGHT as isize;

// Classic march frames, X/. pixels ('.' empty, 'X' filled).
// Crab and octopus are 12x8 px, squid is 8x8 px.
// Rendered 2x2 pixels per cell through quadrant blocks.
const CRAB_A: [&str; 8] = [
    "..X.....X...",
    "...X...X....",
    "..XXXXXXX...",
    ".XX.XXX.XX..",
    "XXXXXXXXXXX.",
    "X.XXXXXXX.X.",
    "X.X.....X.X.",
    "...XX.XX....",
];

const CRAB_B: [&str; 8] = [
    "..X.....X...",
    "X..X...X..X.",
    "X.XXXXXXX.X.",
    "XXX.XXX.XXX.",
    ".XXXXXXXXX..",
    "..XXXXXXX...",
    "...X...X....",
    "..X.....X...",
];

const SQUID_A: [&str; 8] = [
    "...XX...",
    "..XXXX..",
    ".XXXXXX.",
    "XX.XX.XX",
    "XXXXXXXX",
    "..X..X..",
    ".X.XX.X.",
    "X.X..X.X",
];

const SQUID_B: [&str; 8] = [
    "...XX...",
    "..XXXX..",
    ".XXXXXX.",
    "XX.XX.XX",
    "XXXXXXXX",
    ".X.XX.X.",
    "X......X",
    ".X....X.",
];

const OCTOPUS_A: [&str; 8] = [
    "....XXXX....",
    ".XXXXXXXXXX.",
    "XXXXXXXXXXXX",
    "XXX..XX..XXX",
    "XXXXXXXXXXXX",
    "...XX..XX...",
    "..XX.XX.XX..",
    "XX........XX",
];

const OCTOPUS_B: [&str; 8] = [
    "....XXXX....",
    ".XXXXXXXXXX.",
    "XXXXXXXXXXXX",
    "XXX..XX..XXX",
    "XXXXXXXXXXXX",
    "..XXX..XXX..",
    ".XX..XX..XX.",
    "..X......X..",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlienKind {
    Crab,
    Squid,
    Octopus,
}

impl AlienKind {
    /// Random kind for spawning.
    pub fn random() -> Self {
        match rng().random_range(0..3) {
            0 => AlienKind::Crab,
            1 => AlienKind::Squid,
            _ => AlienKind::Octopus,
        }
    }

    /// Look width in cells: 12px bitmaps render 6 wide, 8px renders 4.
    pub fn width(self) -> usize {
        match self {
            AlienKind::Crab | AlienKind::Octopus => 6,
            AlienKind::Squid => 4,
        }
    }

    fn color(self) -> u8 {
        match self {
            AlienKind::Crab => theme::CRAB_COLOR,
            AlienKind::Squid => theme::SQUID_COLOR,
            AlienKind::Octopus => theme::OCTOPUS_COLOR,
        }
    }

    fn frames(self) -> ([&'static str; 8], [&'static str; 8]) {
        match self {
            AlienKind::Crab => (CRAB_A, CRAB_B),
            AlienKind::Squid => (SQUID_A, SQUID_B),
            AlienKind::Octopus => (OCTOPUS_A, OCTOPUS_B),
        }
    }
}

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
    let get =
        |x: usize, y: usize| rows.get(y).and_then(|r| r.as_bytes().get(x)).is_some_and(|&b| b == b'X');
    let mut out = String::new();
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            out.push(quadrant(get(x, y), get(x + 1, y), get(x, y + 1), get(x + 1, y + 1)));
        }
        out.push('\n');
    }
    out
}

fn alien_frame(rows: &[&str], color: u8) -> Look {
    let look = Look::from(bitmap_to_quadrants(rows).as_str());
    for row in look.blocks().iter() {
        for block in row.iter() {
            block.decor.color.set(Some(Color::Ansi(color)));
        }
    }
    look
}

#[derive(Clone, Debug)]
pub struct AlienOptions {
    pub kind: AlienKind,
    pub roam: bool,
}

impl Default for AlienOptions {
    fn default() -> Self {
        Self {
            kind: AlienKind::Crab,
            roam: true,
        }
    }
}

element! {
  pub struct Alien<S> {
      options: AlienOptions = AlienOptions::default(),
      internal_state: AlienState = AlienState::default(),
  }
}

#[derive(Clone, Debug)]
pub struct AlienState {
    pub dy: Cell<isize>,
}

impl Default for AlienState {
    fn default() -> Self {
        Self {
            dy: Cell::new(1),
        }
    }
}

impl<S: Clone + PartialEq> Alien<S> {
    pub fn new(options: AlienOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let (a, b) = el.options.kind.frames();
        let rot = Rotator::<S>::new();
        rot.frames(vec![
            alien_frame(&a, el.options.kind.color()),
            alien_frame(&b, el.options.kind.color()),
        ])
        .interval(ALIEN_INTERVAL_MS)
        .handle("alien_rotator")
        .x(el.get_x())
        .y(el.get_y());
        // Hold the first frame: the march starts once fully on-screen.
        rot.animation(None);
        el.look(Look::from((
            el.options.kind.width(),
            ALIEN_HEIGHT,
            ' ',
        )))
        .handle("alien");
        el.add(rot);

        el
    }

    /// Look width in cells.
    pub fn width(&self) -> usize {
        self.options.kind.width()
    }

    /// One vertical step within the given absolute band,
    /// bouncing at its edges. Fixed formations hold still.
    /// The caller owns the band because
    /// only the game knows where it sits on screen.
    pub fn step_vertical(&self, min_y: isize, max_y: isize) -> &Self {
        if !self.options.roam {
            return self;
        }
        let mut dy = self.internal_state.dy.get();
        let mut y = self.get_y() + dy;
        if y <= min_y {
            y = min_y;
            dy = 1;
        } else if y >= max_y {
            y = max_y;
            dy = -1;
        }
        self.internal_state.dy.set(dy);
        self.y(y);
        self
    }
}

impl<S: Clone + PartialEq> Default for Alien<S> {
    fn default() -> Self {
        Self::new(AlienOptions::default())
    }
}

impl<S: Clone + PartialEq + 'static> Alien<S> {
    /// True once the march animation is running.
    pub fn marching(&self) -> bool {
        self.elements
            .cot::<Rotator<S>>()
            .first()
            .is_some_and(|rot| rot.get_animation().is_some())
    }

    /// Starts the march animation. Holds the first frame until then.
    pub fn start_marching(&self) -> &Self {
        for rot in self.elements.cot::<Rotator<S>>() {
            rot.refresh();
        }
        self
    }

    /// Freezes the march animation, holding the current frame.
    pub fn stop_marching(&self) -> &Self {
        for rot in self.elements.cot::<Rotator<S>>() {
            rot.animation(None);
        }
        self
    }
}
