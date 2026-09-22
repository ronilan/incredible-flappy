use incredible::*;

use crate::ui::app::State;
use crate::ui::elements::{Game, GameOptions};

/// Builds a whole game screen with the given handle.
pub(crate) fn build_as(handle: &str) -> Game<State> {
    let game = Game::<State>::new(GameOptions {
        handle: handle.to_string(),
        ..Default::default()
    });
    game.x(0).y(0).showed(false).draw_override(Some(DrawOverride::default()));

    game
}

/// Builds the whole game screen.
pub(crate) fn build() -> Game<State> {
    build_as("game")
}
