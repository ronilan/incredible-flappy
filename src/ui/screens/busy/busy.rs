use incredible::*;

use crate::ui::app::State;
use crate::ui::elements::{Game, GameOptions};
use crate::ui::theme;

/// Builds the whole busy screen: a purple copy of the classic game.
pub(crate) fn build() -> Game<State> {
    let game = Game::<State>::new(GameOptions {
        base: theme::BUSY_BASE,
        handle: "busy_game".to_string(),
        ..Default::default()
    });
    game.x(0).y(0);

    game
}
