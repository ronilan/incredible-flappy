use incredible::*;
use incredible_helpers_effects::{GradientDirection, gradient_color, transform_rule};

// -----------------------------
// Palette: every color in the game lives here.
// Elements reference these; nothing hardcodes ansi values elsewhere.
// -----------------------------

// Sky and ground.
pub const SKY_BACKGROUND: u8 = CLASSIC_PALETTE.sky;
pub const FLOOR_BACKGROUND: u8 = CLASSIC_PALETTE.floor;
pub const PAVEMENT_BASE: u8 = CLASSIC_PALETTE.pipe_base;
pub const PAVEMENT_STEP: u8 = 6;

// Pipe and cap bases. Striping derives from the base, see below.
pub const PIPE_BACKGROUND: u8 = CLASSIC_PALETTE.pipe_base;
pub const BUSHING_BACKGROUND: u8 = CLASSIC_PALETTE.pipe_base;

/// Pipe striping for any base: base-6, base, base, base+6, base+12, base+6.
pub const fn pipe_segments(base: u8) -> [u8; 6] {
    [
        base.saturating_sub(6),
        base,
        base,
        base.saturating_add(6),
        base.saturating_add(12),
        base.saturating_add(6),
    ]
}

/// Bushing striping for any base.
pub const fn bushing_segments(base: u8) -> [u8; 8] {
    [
        base.saturating_sub(6),
        base,
        base,
        base.saturating_add(6),
        base.saturating_add(6),
        base.saturating_add(6),
        base.saturating_add(12),
        base.saturating_add(6),
    ]
}

// Flora and architecture.
pub const BUSH_BACKGROUND: u8 = CLASSIC_PALETTE.bush_bg;
pub const BUSH_COLOR: u8 = CLASSIC_PALETTE.bush_color;
pub const BUILDINGS_COLOR: u8 = 244;

// Birds (eye, wing/tail, body, belly/beak/head).
pub const BIRD_DARK: u8 = 16;
pub const BIRD_LIGHT: u8 = 231;
pub const BIRD_WING_BACKGROUND: u8 = 160;
pub const BIRD_BODY_BACKGROUND: u8 = 220;
pub const BIRD_BELLY_BACKGROUND: u8 = 214;

// Flowers.
pub const FLOWER_STEM: u8 = 34;
pub const FLOWER_LEAF: u8 = 40;
pub const FLOWER_BUD_COLORS: [u8; 4] = [129, 208, 162, 207];
pub const FLOWER_BUD_FG: u8 = 255;

// Invaders.
pub const CRAB_COLOR: u8 = 46;
pub const SQUID_COLOR: u8 = 46;
pub const OCTOPUS_COLOR: u8 = 46;
pub const BULLET_COLOR: u8 = 231;

// Per-game palettes. Classic is the default everywhere.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub sky: u8,
    pub floor: u8,
    pub bush_bg: u8,
    pub bush_color: u8,
    pub pipe_base: u8,
}

pub const CLASSIC_PALETTE: Palette = Palette {
    sky: 152,
    floor: 187,
    bush_bg: 156,
    bush_color: 40,
    pipe_base: 34,
};

pub const BUSY_PALETTE: Palette = Palette {
    sky: 153,
    floor: 173,
    bush_bg: 148,
    bush_color: 24,
    pipe_base: 98,
};

pub const INVADERS_PALETTE: Palette = Palette {
    sky: 152,
    floor: 191,
    bush_bg: 148,
    bush_color: 24,
    pipe_base: 167,
};

// Title gradients (start, end).
pub const FLAPPY_GRADIENT: (u8, u8) = (156, 46);
pub const SCORE_GRADIENT: (u8, u8) = (15, 250);

// Flat title color: orange end of the old ready gradient.
pub const TITLE_ORANGE: u8 = 214;

// Selected game: dark gray.
pub const SELECTED_ITEM_COLOR: u8 = 240;

pub(crate) fn theme_all() {

    theme_rule::<Style>("Select", |s| {
        s.base.decor.color.set(Some(Color::ansi(SELECTED_ITEM_COLOR)));
    });

    theme_rule::<Style>("TitleOrange", |s| {
        s.base.decor.color.set(Some(Color::ansi(TITLE_ORANGE)));
    });

    transform_rule("FlappyGradient", |flattened, progress| {
        gradient_color(
            &[
                Color::ansi(FLAPPY_GRADIENT.0),
                Color::ansi(FLAPPY_GRADIENT.1),
            ],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
    transform_rule("ScoreGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(SCORE_GRADIENT.0), Color::ansi(SCORE_GRADIENT.1)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
}
