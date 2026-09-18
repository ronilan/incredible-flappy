use incredible::*;
use incredible_elements::{Rectangle, Text};

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};

/// Builds the whole splash screen.
pub(crate) fn build() -> Rectangle<State> {
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .handle("splash");

    let title = Text::<State>::default();
    title.text("SPLASH").handle("splash_title").x(2).y(2);
    splash.add(title);

    splash
}
