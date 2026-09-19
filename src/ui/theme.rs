use incredible::Color;
use incredible_helpers_effects::{GradientDirection, gradient_color, transform_rule};

pub(crate) fn theme_all() {
    transform_rule("FlappyGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(156), Color::ansi(46)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
    transform_rule("ReadyGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(214), Color::ansi(220)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
    transform_rule("ScoreGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(15), Color::ansi(250)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
}
