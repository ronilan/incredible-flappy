use incredible::Color;
use incredible_helpers_effects::{GradientDirection, gradient_color, transform_rule};

// -----------------------------
// Palette: every color in the game lives here.
// Elements reference these; nothing hardcodes ansi values elsewhere.
// -----------------------------

// Sky and ground.
pub const SKY_BACKGROUND: u8 = 152;
pub const FLOOR_BACKGROUND: u8 = 187;
pub const PAVEMENT_BASE: u8 = 34;
pub const PAVEMENT_STEP: u8 = 6;

// Pipe and cap.
pub const PIPE_BACKGROUND: u8 = 34;
pub const PIPE_SEGMENTS: [u8; 6] = [28, 34, 34, 40, 46, 40];
pub const BUSHING_BACKGROUND: u8 = 34;
pub const BUSHING_SEGMENTS: [u8; 8] = [28, 34, 34, 40, 40, 40, 46, 40];

// Flora and architecture.
pub const BUSH_BACKGROUND: u8 = 156;
pub const BUSH_COLOR: u8 = 40;
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

// Title gradients (start, end).
pub const FLAPPY_GRADIENT: (u8, u8) = (156, 46);
pub const READY_GRADIENT: (u8, u8) = (214, 220);
pub const SCORE_GRADIENT: (u8, u8) = (15, 250);

pub(crate) fn theme_all() {
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
    transform_rule("ReadyGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(READY_GRADIENT.0), Color::ansi(READY_GRADIENT.1)],
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
