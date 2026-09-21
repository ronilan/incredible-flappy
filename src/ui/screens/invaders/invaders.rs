use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::elements::{Crab, Octopus, Squid};
use crate::ui::theme;

/// Builds the whole invaders screen.
pub(crate) fn build() -> Rectangle<State> {
    let invaders = Rectangle::<State>::new();
    invaders
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("invaders");

    let squid = Squid::<State>::default();
    squid.x(38).y(3);
    invaders.add(squid);

    let crab = Crab::<State>::default();
    crab.x(37).y(8);
    invaders.add(crab);

    let octopus = Octopus::<State>::default();
    octopus.x(37).y(14);
    invaders.add(octopus);

    invaders
}
