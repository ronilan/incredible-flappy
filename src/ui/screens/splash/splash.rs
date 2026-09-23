use incredible::*;
use incredible_elements::{Label, Rectangle};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, State};
use crate::ui::theme;

use super::{buttons, fluffy, hint, image_buttons, kicker, scenery, title};
use crate::ui::layout::lift_content;

/// Builds the whole splash screen.
///
/// Layout lives here so x/y can be tuned per screen: kicker above
/// the title, fluffy over it in kitty mode, buttons and their image
/// twins sharing one row, hint floating near the bottom.
pub(crate) fn build() -> Rectangle<State> {
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::Ansi(theme::SKY_BACKGROUND)))
        .handle("splash")
        .draw_override(Some(DrawOverride::default()));

    splash.on_key(|el, _state, _event|{
        el.draw();
    });

    let (kicker_the, kicker_link) = kicker::build_kicker();
    let kicker_the_w = kicker_the.visual.look.width() as isize;
    let kicker_link_w = kicker_link.visual.look.width() as isize;
    let kicker_h = kicker_the.visual.look.height() as isize;
    let kicker_x = (SCREEN_WIDTH as isize - (kicker_the_w + kicker_link_w)) / 2;

    let title = title::build_title();
    title.y((16 - title.visual.look.height() as isize) / 2);
    let title_w = title.visual.look.width();
    let title_y = title.get_y();
    kicker_the.y(title_y - kicker_h - 1).x(kicker_x);
    kicker_link
        .y(title_y - kicker_h - 1)
        .x(kicker_x + kicker_the_w);
    let top_gap = kicker_the.get_y();
    splash.add(kicker_the);
    splash.add(kicker_link);
    splash.add(title);
    splash.elements_to_center_x_of_type::<incredible_elements_text_fonts::BlockCharsStr<State>>();

    let fluffy = fluffy::build_fluffy(title_w);
    fluffy.y(title_y);
    splash.add(fluffy);
    splash.elements_to_center_x_of_type::<incredible_elements::Image<State>>();

    let [left, right] = scenery::build_scenery();
    left.x(0).y(16);
    splash.add(left);
    right.x(40).y(16);
    splash.add(right);

    let [basic, graze, shoot] = buttons::build_buttons();
    let (basic_w, graze_w, shoot_w) = (
        basic.visual.look.width() as isize,
        graze.visual.look.width() as isize,
        shoot.visual.look.width() as isize,
    );
    let row_x = (SCREEN_WIDTH as isize - (basic_w + graze_w + shoot_w + 2)) / 2;
    basic.x(row_x).y(11);
    splash.add(basic);
    graze.x(row_x + basic_w + 1).y(11);
    splash.add(graze);
    shoot.x(row_x + basic_w + 1 + graze_w + 1).y(11);
    splash.add(shoot);

    let [basic_img, graze_img, shoot_img] = image_buttons::build_image_buttons();
    let (bw0, bw1, bw2) = (
        basic_img.visual.look.width() as isize,
        graze_img.visual.look.width() as isize,
        shoot_img.visual.look.width() as isize,
    );
    let img_x = (SCREEN_WIDTH as isize - (bw0 + bw1 + bw2 + 2)) / 2;
    basic_img.x(img_x).y(12);
    splash.add(basic_img);
    graze_img.x(img_x + bw0 + 1).y(12);
    splash.add(graze_img);
    shoot_img.x(img_x + bw0 + 1 + bw1 + 1).y(12);
    splash.add(shoot_img);

    let hint = hint::build_hint();
    hint.y(SCREEN_HEIGHT as isize - hint.visual.look.height() as isize - 4);
    splash.add(hint);

    let kitty_hint = hint::build_kitty_hint();
    kitty_hint.x(
        (SCREEN_WIDTH as isize - kitty_hint.visual.look.width() as isize) / 2,
    );
    splash.add(kitty_hint);

    lift_content(&splash, top_gap);

    // Kitty line on the pavement row.
    for line in splash.elements.dcot_w::<Label<State>, _>(|e| {
        e.get_handle() == "kitty_hint"
    }) {
        line.y(21);
    }

    splash
}
