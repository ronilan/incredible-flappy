use incredible::*;
use incredible_elements::FramedText;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_WIDTH, State};
use crate::ui::app::SelectedGame;
use crate::ui::theme;

pub const HINT_WIDTH: usize = 30;

/// Builds the bottom info box. Horizontally centered but floating;
/// the composer sets its row.
pub(crate) fn build_hint() -> FramedText<State> {
    let hint = FramedText::<State>::default();
    hint.text(game_hint_for(&SelectedGame::Classic));
    hint.handle("game_hint");
    hint.width(HINT_WIDTH);
    hint.conf_unframed();
    hint.background(Some(Color::Ansi(theme::HINT_BACKGROUND)));
    hint.color(Some(Color::Ansi(theme::HINT_TEXT)));
    hint.x((SCREEN_WIDTH as isize - HINT_WIDTH as isize) / 2);
    hint
}

/// Hint text per selected game.
pub(crate) fn game_hint_for(selected: &SelectedGame) -> &'static str {
    match selected {
        SelectedGame::Classic => "Avoid the pipes. Avoid the ground. Avoid the sky.",
        SelectedGame::Busy => "Graze (but don't bump) the flowers for fame and fortune.",
        SelectedGame::Invaders => "Pipes? Invaders? Shooting. You are Doomed...",
    }
}
