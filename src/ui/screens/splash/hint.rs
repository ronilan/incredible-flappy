use incredible::*;
use incredible_elements::FramedText;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_WIDTH, State};
use crate::ui::app::SelectedGame;
use crate::ui::theme;

pub const HINT_WIDTH: usize = 26;

/// Builds the bottom info box. Horizontally centered but floating;
/// the composer sets its row.
pub(crate) fn build_hint() -> FramedText<State> {
    let hint = FramedText::<State>::default();
    hint.text(game_hint_for(&None));
    hint.handle("game_hint");
    hint.width(HINT_WIDTH);
    hint.conf_unframed();
    hint.background(Some(Color::Ansi(theme::HINT_BACKGROUND)));
    hint.color(Some(Color::Ansi(theme::HINT_TEXT)));
    hint.x((SCREEN_WIDTH as isize - HINT_WIDTH as isize) / 2);
    hint
}

/// Hint text per focused game, or the default line when empty.
pub(crate) fn game_hint_for(focused: &Option<SelectedGame>) -> &'static str {
    match focused {
        Some(SelectedGame::Classic) => "Avoid the pipes. Avoid the ground. Avoid the sky. All the basics.",
        Some(SelectedGame::Busy) => "Same, same but it sunsine and flowers. Graze (but don't bump) them.",
        Some(SelectedGame::Invaders) => "Pipes? Invaders? Shooting?\nOh, boy...You are so, so doomed...",
        None => "Arrows to select. Enter to start. Space to bump. Esc to back.",
    }
}
