use incredible::*;
use incredible_elements::{Button, Rectangle};
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

/// Launches a game straight from app state. No widget state involved.
fn launch(selected: SelectedGame, state: &mut State) {
    state.selected = selected;
    state.phase = crate::ui::app::Phase::Ready;
}

fn game_button(label: &str, game: SelectedGame) -> Button<State> {
    let btn = Button::<State>::default();
    btn.width(label.len() + 2).text(label);
    let pick = game;
    btn.on_key(move |_el, state, event| {
        if event.key == Key::Enter {
            launch(pick, state);
        }
    })
    .on_mouse(move |_el, state, event| {
        if event.mouse == Mouse::Click {
            launch(pick, state);
        }
    });
    btn
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

    let classic = game_button("Classic", SelectedGame::Classic);
    let busy = game_button("Busy", SelectedGame::Busy);
    let invaders = game_button("Invaders", SelectedGame::Invaders);
    let total = 9 + 1 + 6 + 1 + 10;
    classic.x((SCREEN_WIDTH as isize - total) / 2).y(11);
    busy.x((SCREEN_WIDTH as isize - total) / 2 + 10).y(11);
    invaders.x((SCREEN_WIDTH as isize - total) / 2 + 17).y(11);
    splash.add(classic);
    splash.add(busy);
    splash.add(invaders);

    splash
}
