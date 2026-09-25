use incredible::*;
use incredible_elements::{FrameKind, FrameStyle, FramedText, Label, LabelOptions};
use incredible_helpers_styling::*;

use crate::ui::app::SCREEN_WIDTH;
use crate::ui::state::State;
use crate::ui::state::SelectedGame;
use crate::ui::theme;

pub const HINT_WIDTH: usize = 28;

/// Builds the bottom info box. Horizontally centered but floating;
/// the composer sets its row.
pub(crate) fn build_hint() -> FramedText<State> {
    let hint = FramedText::<State>::default();
    hint.text(game_hint_for(&None))
        .conf_unframed()
        .handle("game_hint")
        .focused(false)
        .width(HINT_WIDTH)
        .clip_padding(ClipPadding::new(0, 1, 0, 1))
        .frame_style({
            let style = FrameStyle::default();
            style.base.kind.set(Some(FrameKind::Blank));
            style
        })
        .background(Some(Color::Ansi(theme::HINT_BACKGROUND)))
        .color(Some(Color::Ansi(theme::HINT_TEXT)));
    hint.x((SCREEN_WIDTH as isize - HINT_WIDTH as isize) / 2);
    hint
}

/// Kitty toggle line above the pavement. White, no background.
/// Kitty toggle line above the pavement. White, no background.
/// Clicking toggles kitty mode, same as K.
pub(crate) fn build_kitty_hint() -> Label<State> {
    let label = Label::<State>::new(LabelOptions::default());
    label
        .text("K is for kitty")
        .color(Some(Color::Ansi(theme::KITTY_HINT_TEXT)))
        .handle("kitty_hint")
        .interactive(true);
    label.on_mouse(|_el, state, event| {
        if matches!(event.mouse, Mouse::Click) && Platform::output_provider().images() {
            state.kitty = !state.kitty;
            crate::settings::persist_now(state);
        }
    });
    label
}

/// Hint text per focused game, or the default line when empty.
pub(crate) fn game_hint_for(focused: &Option<SelectedGame>) -> &'static str {    match focused {
        Some(SelectedGame::Classic) => "Avoid the pipes. Avoid the ground. Avoid the sky. All the basics.",
        Some(SelectedGame::Busy) => "Same, same but it sunsine and flowers. Graze (but don't bump) them.",
        Some(SelectedGame::Invaders) => "Pipes? Invaders? Shooting?\nOh, boy...You are so, so doomed...",
        None => "Tab to select. Enter to start. Space/Enter to bump. Esc to pause.",
    }
}
