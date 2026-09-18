use incredible::*;
use incredible_elements::Rectangle;
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::elements::Scenery;

/// Builds the whole splash screen.
pub(crate) fn build() -> Rectangle<State> {
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(152)))
        .handle("splash");

    let title = BlockCharsStr::<State>::default();
    title.text("Flappy").size(BlockSize::Small);
    title
        .x(
            (SCREEN_WIDTH as isize - title.visual.look.width() as isize) / 2,
        )
        .y((16 - title.visual.look.height() as isize) / 2);
    splash.add(title);

    let left = Scenery::<State>::default();
    left.x(0).y(16);
    splash.add(left);

    let right = Scenery::<State>::default();
    right.x(40).y(16);
    splash.add(right);

    splash
}
