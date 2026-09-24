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

// Invaders: basic colors in normal mode, logos in kitty mode.
pub const CRAB_COLOR: u8 = 9;
pub const SQUID_COLOR: u8 = 11;
pub const OCTOPUS_COLOR: u8 = 12;
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
    sky: 45,
    floor: 191,
    bush_bg: 148,
    bush_color: 24,
    pipe_base: 167,
};

// Title gradients (start, end).
pub const FLAPPY_GRADIENT: (u8, u8,u8, u8, u8, u8) = (229, 229, 229, 46, 46, 46);

// Selected game: dark gray.
pub const SELECTED_ITEM_COLOR: u8 = 240;

// Splash hint: white box, near-black text.
pub const HINT_BACKGROUND: u8 = 15;
pub const HINT_TEXT: u8 = 232;
pub const KITTY_HINT_TEXT: u8 = 15;

// Game buttons: burnt orange, white text.
pub const BUTTON_BACKGROUND: u8 = 166;
pub const BUTTON_TEXT: u8 = 15;

pub(crate) fn theme_all() {
    theme_rule::<Style>("Crab", |s| {
        s.base.decor.color.set(Some(Color::ansi(CRAB_COLOR)));
    });
    theme_rule::<Style>("Squid", |s| {
        s.base.decor.color.set(Some(Color::ansi(SQUID_COLOR)));
    });
    theme_rule::<Style>("Octopus", |s| {
        s.base.decor.color.set(Some(Color::ansi(OCTOPUS_COLOR)));
    });

    transform_rule("FlappyGradient", |flattened, progress| {
        gradient_color(
            &[
                Color::ansi(FLAPPY_GRADIENT.0),
                Color::ansi(FLAPPY_GRADIENT.1),
                Color::ansi(FLAPPY_GRADIENT.2),
                Color::ansi(FLAPPY_GRADIENT.3),
                Color::ansi(FLAPPY_GRADIENT.4),
                Color::ansi(FLAPPY_GRADIENT.5),
            ],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
}

// Normal-mode score: black box, white text.
pub const SCORE_BACKGROUND: u8 = 16;
pub const SCORE_TEXT: u8 = 15;

// Score panel: cream box, black text.
pub const SCORE_PANEL_BACKGROUND: u8 = 229;
/// Flip the panel box to exact RGB by commenting one line:
pub const SCORE_PANEL_BACKGROUND_RGB: Option<(u8, u8, u8)> =
    //None;
    Some((219, 218, 150));
pub const SCORE_PANEL_LABEL_BACKGROUND: (u8, u8, u8) = (219, 218, 150);
pub const SCORE_PANEL_TEXT: u8 = 16;

// Stoplight: pavement dark burns, dim gray rests.
pub const STOPLIGHT_LIT: u8 = PAVEMENT_BASE;
pub const STOPLIGHT_DIM: u8 = 240;

// Graze sparkles fall back to bright white.
pub const SPARKLE_COLOR: u8 = 231;

/// Panel box color: exact RGB when set, otherwise the ANSI cream.
pub fn score_panel_background() -> Color {
    match SCORE_PANEL_BACKGROUND_RGB {
        Some((r, g, b)) => Color::Rgba(Rgba::new(r, g, b, 255)),
        None => Color::Ansi(SCORE_PANEL_BACKGROUND),
    }
}

// Game titles: orange box, black text. Get Ready runs lighter.
pub const TITLE_BACKGROUND: u8 = 214;
pub const TITLE_TEXT: u8 = 16;
pub const GET_READY_BACKGROUND: u8 = 215;
