use std::rc::Rc;

use incredible::*;
use incredible_elements::Rectangle;
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::elements::{FlyingBird, Scenery};

fn title_effects(el: &BlockCharsStr<State>) {
    decorate_rules::<State, BlockCharsStr<State>>(el, title_effects);
}

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
    title
        .text("Flappy")
        .size(BlockSize::Small)
        .style_handle("FlappyGradient");
    title.x(0).y(6);
    effect(&title, title_effects);
    splash.add(title);

    let bird = FlyingBird::<State>::default();
    splash.add(bird);

    // Bird after the word, pair centered as a group.
    let mut pair: Vec<Rc<dyn ElementTrait<State>>> = splash
        .elements
        .cot::<BlockCharsStr<State>>()
        .into_iter()
        .map(|el| el as Rc<dyn ElementTrait<State>>)
        .collect();
    for el in splash.elements.cot::<FlyingBird<State>>() {
        pair.push(el as Rc<dyn ElementTrait<State>>);
    }
    splash.elements_flow_filtered(
        &pair,
        FlowConfig {
            direction: Direction::Right,
            wrap_direction: Direction::Right,
            align: Align::Start,
            item_space: 1,
            wrap_space: 0,
        },
    );
    splash.elements_to_center_x_filtered(&pair);

    let left = Scenery::<State>::default();
    left.x(0).y(16);
    splash.add(left);

    let right = Scenery::<State>::default();
    right.x(40).y(16);
    splash.add(right);

    splash
}
