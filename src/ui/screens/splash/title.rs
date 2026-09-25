use incredible::*;
use incredible_elements_text_fonts::BlockCharsStr;
use incredible_helpers_effects::*;

use crate::ui::state::State;

fn title_effects(el: &BlockCharsStr<State>) {
    decorate_rules::<State, BlockCharsStr<State>>(el, title_effects);
}

/// Builds the Flappy title. Unpositioned; the composer places it.
pub(crate) fn build_title() -> BlockCharsStr<State> {
    let title = BlockCharsStr::<State>::default();
    title
        .text("Flappy")
        .style_handle("FlappyGradient")
        .handle("flappy_title");
    effect(&title, title_effects);
    title
}
