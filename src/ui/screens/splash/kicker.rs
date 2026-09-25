use incredible::*;
use incredible_helpers_styling::*;
use incredible_elements::{Label, LabelOptions, Link};

use crate::ui::state::State;
use crate::ui::theme;

/// Builds the kicker pair: "The " label and "Incredible" link,
/// arranged side by side from x=0. The composer places the pair.
pub(crate) fn build_kicker() -> (Label<State>, Link<State>) {
    let the = Label::<State>::new(LabelOptions::default());
    the.text("The ")
        .color(Some(Color::Ansi(theme::SELECTED_ITEM_COLOR)))
        .handle("incredible_kicker_the");

    let link = Link::<State>::new();
    link.text("Incredible")
        .url("https://www.incredible.rs")
        .color(Some(Color::Ansi(theme::SELECTED_ITEM_COLOR)))
        .handle("incredible_kicker_link")
        .pointer(Some(PointerShape::Pointer));
    link.x(the.visual.look.width() as isize);

    (the, link)
}
