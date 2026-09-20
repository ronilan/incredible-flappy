use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::theme;

/// Builds the whole invaders screen (stub).
pub(crate) fn build() -> Rectangle<State> {
    let invaders = Rectangle::<State>::new();
    invaders
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("invaders");

    let title = Text::<State>::default();
    title
        .text("INVADERS")
        .handle("invaders_title")
        .x(2)
        .y(2);
    invaders.add(title);
    invaders.elements_to_center_x_of_type::<Text<State>>();

    invaders
}
