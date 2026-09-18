use incredible::*;
use incredible_elements::Rectangle;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};

/// Builds the splash screen container.
pub(crate) fn build() -> Rectangle<State> {
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .handle("splash");

    splash
}
