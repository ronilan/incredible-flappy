use incredible::*;
use incredible_elements::{App, Rectangle, Text};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

use crate::ui::elements;

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 24;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum State {
    #[default]
    Splash,
    Ready,
    Flying,
    Dead,
}

fn game_title_for(state: &State) -> &'static str {
    match state {
        State::Splash => "GAME",
        State::Ready => "GAME - READY",
        State::Flying => "GAME - FLYING",
        State::Dead => "GAME - DEAD",
    }
}

fn game_hint_for(state: &State) -> &'static str {
    match state {
        State::Splash => "",
        State::Ready => "Space -> Flying | d -> Dead | Esc -> Splash",
        State::Flying => "d -> Dead | Esc -> Splash",
        State::Dead => "Enter -> Ready | Esc -> Splash",
    }
}

pub fn transition(state: &State, key: &Key) -> Option<State> {
    match (key, state) {
        (Key::Escape, _) => Some(State::Splash),
        (Key::Enter, State::Splash) => Some(State::Ready),
        (Key::Enter, State::Dead) => Some(State::Ready),
        (Key::Char(' '), State::Ready) => Some(State::Flying),
        (Key::Char('d'), State::Ready) => Some(State::Dead),
        (Key::Char('D'), State::Ready) => Some(State::Dead),
        (Key::Char('d'), State::Flying) => Some(State::Dead),
        (Key::Char('D'), State::Flying) => Some(State::Dead),
        _ => None,
    }
}

pub fn build() -> App<State> {
    let app = App::default();

    app.on_window(|el, _state, _event| {
        el.elements_to_center();
    });

    // Splash screen: 80x24 rectangle.
    let splash = Rectangle::<State>::new();
    splash
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .handle("splash");

    let splash_title = Text::<State>::default();
    splash_title
        .text("SPLASH")
        .handle("splash_title")
        .x(2)
        .y(2);

    let splash_hint = Text::<State>::default();
    splash_hint
        .text("Enter -> Game-Ready | Esc -> Splash")
        .handle("splash_hint")
        .x(2)
        .y(4);

    splash.add(splash_title);
    splash.add(splash_hint);

    // Game screen: 80x24 rectangle.
    let game = Rectangle::<State>::new();
    game.width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::from(152)))
        .handle("game");

    let title = Text::<State>::default();
    title
        .text(game_title_for(&State::Ready))
        .handle("game_title")
        .x(2)
        .y(2);

    let hint = Text::<State>::default();
    hint.text(game_hint_for(&State::Ready))
        .handle("game_hint")
        .x(2)
        .y(4);

    game.add(title);
    game.add(hint);

    // Scroller background layer, then static foreground elements.
    let scroller = elements::Scroller::<State>::default();
    scroller.x(0).y(0);
    game.add(scroller);

    let flying_bird = elements::FlyingBird::<State>::default();
    flying_bird.x(40).y(6);
    game.add(flying_bird);

    let dead_bird = elements::DeadBird::<State>::default();
    dead_bird.x(48).y(6);
    game.add(dead_bird);



    // Initial visibility: start on Splash.
    splash.showed(true);
    game.showed(false);

    app.on_state(|el, state, _event| {
        let is_splash = *state == State::Splash;
        for rect in el
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "splash")
        {
            rect.showed(is_splash);
        }
        for rect in el
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "game")
        {
            rect.showed(!is_splash);
        }
        for t in el
            .elements
            .dcot_w::<Text<State>, _>(|e| e.get_handle() == "game_title")
        {
            t.text(game_title_for(state));
        }
        for t in el
            .elements
            .dcot_w::<Text<State>, _>(|e| e.get_handle() == "game_hint")
        {
            t.text(game_hint_for(state));
        }
        for scroller in el
            .elements
            .dcot_w::<elements::Scroller<State>, _>(|e| e.get_handle() == "scroller")
        {
            match state {
                State::Ready => {
                    scroller.reset();
                    scroller.set_running(false);
                }
                State::Flying => {
                    scroller.set_running(true);
                }
                _ => {
                    scroller.set_running(false);
                }
            }
        }
        el.draw();
    });

    app.on_key(|el, state, event| {
        if let Some(next) = transition(state, &event.key) {
            if next != *state {
                *state = next;
                el.draw();
            }
        }
    });

    app.add(splash);
    app.add(game);

    app
}
