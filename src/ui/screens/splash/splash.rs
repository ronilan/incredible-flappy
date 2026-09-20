use incredible::*;
use incredible_elements::{Image, ImageData, Label, Rectangle, Select};
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::app::SelectedGame;
use crate::ui::elements::Scenery;
use crate::ui::theme;

fn title_effects(el: &BlockCharsStr<State>) {
    decorate_rules::<State, BlockCharsStr<State>>(el, title_effects);
}

fn decode_png(bytes: &[u8]) -> ImageData {
    let img = image::load_from_memory(bytes).expect("splash asset decodes");
    let rgba = img.to_rgba8();
    let (width_px, height_px) = (rgba.width(), rgba.height());
    ImageData {
        bytes: rgba.into_raw(),
        width_px,
        height_px,
    }
}

/// Builds the whole splash screen.
pub(crate) fn build() -> Rectangle<State> {
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("splash");

    let title = BlockCharsStr::<State>::default();
    title
        .text("Flappy")
        .size(BlockSize::Small)
        .style_handle("FlappyGradient");
    title.y((16 - title.visual.look.height() as isize) / 2);
    title.handle("flappy_title");
    effect(&title, title_effects);
    splash.add(title);
    splash.elements_to_center_x_of_type::<BlockCharsStr<State>>();

    let fluffy = Image::<State>::new();
    fluffy.width(20);
    fluffy.data(decode_png(include_bytes!("../../../../assets/fluffy.png")));
    fluffy.handle("fluffy_title");
    fluffy.showed(false);
    fluffy.y((16 - fluffy.visual.look.height() as isize) / 2);
    splash.add(fluffy);
    splash.elements_to_center_x_of_type::<Image<State>>();

    let left = Scenery::<State>::default();
    left.x(0).y(16);
    splash.add(left);

    let right = Scenery::<State>::default();
    right.x(40).y(16);
    splash.add(right);

    let select = Select::<State>::new();
    select
        .add_item("Classic", "classic")
        .add_item("Busy", "busy")
        .add_item("Invaders", "invaders");
    select.width(20).height(5);
    select.allow_unselect(false);
    select.handle("game_select");
    select.y(11);
    select.select_action(0);
    select.focused(true);
    splash.add(select);
    splash.elements_to_center_x_of_type::<Select<State>>();

    let hint = Label::<State>::default();
    hint.text(game_hint_for(&SelectedGame::Classic));
    hint.handle("game_hint");
    hint.y(23);
    splash.add(hint);
    splash.elements_to_center_x_of_type::<Label<State>>();

    splash
}

/// Hint text per selected game.
pub(crate) fn game_hint_for(selected: &SelectedGame) -> &'static str {
    match selected {
        SelectedGame::Classic => "Avoid the pipes. Avoid the ground.",
        SelectedGame::Busy => "Graze (but don't bump) the flowers for fame and fortune.",
        SelectedGame::Invaders => "Shoot the invaders",
    }
}
