use incredible::*;

use crate::ui::app::State;
use crate::ui::elements::Game;

/// Builds the whole game screen.
pub(crate) fn build() -> Game<State> {
    let game = Game::<State>::default();
    game.x(0).y(0);

    game
}
