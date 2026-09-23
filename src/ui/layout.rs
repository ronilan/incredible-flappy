use incredible::*;
use incredible_elements::{Button, FramedText, Image, Label, Link, Rectangle};
use incredible_elements_text_fonts::BlockCharsStr;

use crate::ui::app::State;
use crate::ui::elements::ImageButton;

/// Lifts everything but the ground to leave one row at the top.
pub(crate) fn lift_content(splash: &Rectangle<State>, top_gap: isize) {
    let dy = top_gap - 1;
    if dy == 0 {
        return;
    }
    for el in splash.elements.cot::<Label<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<Link<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<BlockCharsStr<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<Image<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<Button<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<FramedText<State>>() {
        el.y(el.get_y() - dy);
    }
    for el in splash.elements.cot::<ImageButton<State>>() {
        el.y(el.get_y() - dy);
    }
}
