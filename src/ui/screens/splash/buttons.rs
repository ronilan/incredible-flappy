use incredible::*;
use incredible_helpers_styling::*;
use incredible_elements::Button;

use crate::ui::app::{Phase, SelectedGame, State};
use crate::ui::theme;

fn game_button(text: &str, handle: &str, game: SelectedGame) -> Button<State> {
    let btn = Button::<State>::new();
    btn.text(text)
        .background(Some(Color::Ansi(theme::BUTTON_BACKGROUND)))
        .color(Some(Color::Ansi(theme::BUTTON_TEXT)))
        .handle(handle)
        .focused(false)
        .pointer(Some(PointerShape::Pointer));
    // Hovering steers splash focus; on_state paints the hint from it.
    btn.on_change(move |_el, state: &mut State, event| {
        if state.phase == Phase::Splash && event.changes.contains(&Change::Hovered(true)) {
            state.focus = Some(game);
        }
    });
    btn
}

/// Builds the three game buttons: BASIC, GRAZE, SHOOT.
/// Unpositioned; the composer lays out the row.
pub(crate) fn build_buttons() -> [Button<State>; 3] {
    [
        game_button("BASIC", "game_basic", SelectedGame::Classic),
        game_button("GRAZE", "game_graze", SelectedGame::Busy),
        game_button("SHOOT", "game_shoot", SelectedGame::Invaders),
    ]
}
