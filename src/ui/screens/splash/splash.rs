use incredible::*;
use incredible_elements::{FrameKind, FramedText, Image, Label, LabelOptions, Rectangle, Select};
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::app::SelectedGame;
use crate::ui::assets::decode_png;
use crate::ui::elements::Scenery;
use crate::ui::theme;

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
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("splash")
        .draw_override(Some(DrawOverride::default()));

    splash.on_key(|el, state, event|{
        el.draw();
    });

    let kicker = Label::<State>::new(LabelOptions::default());
    kicker.text("The Incredible");
    kicker.color(Some(Color::Ansi(theme::SELECTED_ITEM_COLOR)));
    let kicker_h = kicker.visual.look.height() as isize;
    kicker.handle("incredible_kicker");

    let title = BlockCharsStr::<State>::default();
    title
        .text("Flappy")
        .size(BlockSize::Small)
        .style_handle("FlappyGradient");
    title.y((16 - title.visual.look.height() as isize) / 2);
    title.handle("flappy_title");
    effect(&title, title_effects);
    let title_h = title.visual.look.height();
    kicker.y(title.get_y() - kicker_h - 1);
    let top_gap = kicker.get_y();
    splash.add(kicker);
    splash.add(title);
    splash.elements_to_center_x_of_type::<BlockCharsStr<State>>();
    splash.elements_to_center_x_of_type::<Label<State>>();

    let fluffy = Image::<State>::new();
    fluffy.height(title_h * 3 / 2);
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
    select.frame_kind(Some(FrameKind::Double));
    select.selected_color(Some(Color::Ansi(theme::SELECTED_ITEM_COLOR)));
    select.handle("game_select");
    select.y(11);
    select.select_action(0);
    select.focused(true);
    splash.add(select);
    splash.elements_to_center_x_of_type::<Select<State>>();

    let hint = FramedText::<State>::default();
    hint.text(game_hint_for(&SelectedGame::Classic));
    hint.handle("game_hint");
    hint.width(30);
    hint.color(Some(Color::Ansi(theme::SELECTED_ITEM_COLOR)));
    hint.x((SCREEN_WIDTH as isize - 30) / 2);
    hint.y(SCREEN_HEIGHT as isize - hint.visual.look.height() as isize - 2);
    splash.add(hint);

    // Lift everything but the ground to leave one row at the top.
    let dy = top_gap - 1;
    if dy != 0 {
        for el in splash.elements.cot::<Label<State>>() {
            el.y(el.get_y() - dy);
        }
        for el in splash.elements.cot::<BlockCharsStr<State>>() {
            el.y(el.get_y() - dy);
        }
        for el in splash.elements.cot::<Image<State>>() {
            el.y(el.get_y() - dy);
        }
        for el in splash.elements.cot::<Select<State>>() {
            el.y(el.get_y() - dy);
        }
        for el in splash.elements.cot::<FramedText<State>>() {
            el.y(el.get_y() - dy);
        }
    }

    splash
}

/// Hint text per selected game.
pub(crate) fn game_hint_for(selected: &SelectedGame) -> &'static str {
    match selected {
        SelectedGame::Classic => "Avoid the pipes. Avoid the ground. Avoid the sky.",
        SelectedGame::Busy => "Graze (but don't bump) the flowers for fame and fortune.",
        SelectedGame::Invaders => "Pipes? Invaders? Shooting. You are Doomed...",
    }
}
