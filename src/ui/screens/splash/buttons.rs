use incredible::*;
use incredible_helpers_styling::*;
use incredible_elements::Button;

use crate::ui::app::State;
use crate::ui::theme;

fn game_button(text: &str, handle: &str, focused: bool) -> Button<State> {
    let btn = Button::<State>::new();
    btn.text(text);
    btn.background(Some(Color::Ansi(theme::BUTTON_BACKGROUND)));
    btn.color(Some(Color::Ansi(theme::BUTTON_TEXT)));
    btn.handle(handle);
    btn.focused(focused);
    btn
}

/// Builds the three game buttons: BASIC, GRAZE, SHOOT.
/// Unpositioned; the composer lays out the row.
pub(crate) fn build_buttons() -> [Button<State>; 3] {
    [
        game_button("BASIC", "game_basic", true),
        game_button("GRAZE", "game_graze", false),
        game_button("SHOOT", "game_shoot", false),
    ]
}
