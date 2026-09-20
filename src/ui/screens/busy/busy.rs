use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::theme;

/// Builds the whole busy screen (stub).
pub(crate) fn build() -> Rectangle<State> {
    let busy = Rectangle::<State>::new();
    busy.width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("busy");

    let title = Text::<State>::default();
    title.text("BUSY").handle("busy_title").x(2).y(2);
    busy.add(title);
    busy.elements_to_center_x_of_type::<Text<State>>();

    busy
}
