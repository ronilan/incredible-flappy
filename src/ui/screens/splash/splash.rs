use incredible::*;
use incredible_elements::{Rectangle, Select};
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::app::{SCREEN_HEIGHT, SCREEN_WIDTH, SelectedGame, State};
use crate::ui::elements::Scenery;
use crate::ui::theme;

fn title_effects(el: &BlockCharsStr<State>) {
    decorate_rules::<State, BlockCharsStr<State>>(el, title_effects);
}

/// Launches the selected game.
fn launch(select: &Select<State>, state: &mut State) {
    let idx = select
        .get_selected()
        .or(select.get_focused_index())
        .unwrap_or(0);
    state.selected = match idx {
        1 => SelectedGame::Busy,
        2 => SelectedGame::Invaders,
        _ => SelectedGame::Classic,
    };
    state.phase = crate::ui::app::Phase::Ready;
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
    effect(&title, title_effects);
    splash.add(title);
    splash.elements_to_center_x_of_type::<BlockCharsStr<State>>();

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
    select.focused_index(Some(0));
    select.handle("game_select");
    select.y(11);
    select.on_key(|el, state: &mut State, event| {
        if event.key == Key::Enter {
            launch(el, state);
        }
    });
    select.on_mouse(|el, state: &mut State, event| {
        if event.mouse == Mouse::Click {
            launch(el, state);
        }
    });
    splash.add(select);
    splash.elements_to_center_x_of_type::<Select<State>>();

    splash
}
