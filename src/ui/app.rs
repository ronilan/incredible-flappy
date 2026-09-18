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
        (Key::Enter | Key::Char(' '), State::Splash) => Some(State::Ready),
        (Key::Enter | Key::Char(' '), State::Dead) => Some(State::Ready),
        (Key::Enter | Key::Char(' '), State::Ready) => Some(State::Flying),
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

    // Game screen: the scroller.
    let game = elements::Game::<State>::default();
    game.x(0).y(0);
    for t in game
        .elements
        .dcot_w::<Text<State>, _>(|e| e.get_handle() == "game_title")
    {
        t.text(game_title_for(&State::Ready));
    }
    for t in game
        .elements
        .dcot_w::<Text<State>, _>(|e| e.get_handle() == "game_hint")
    {
        t.text(game_hint_for(&State::Ready));
    }



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
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
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
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            match state {
                State::Ready => {
                    game.reset();
                    game.set_running(false);
                }
                State::Flying => {
                    game.set_running(true);
                }
                _ => {
                    game.set_running(false);
                }
            }
            let flying = game
                .elements
                .dcot_w::<elements::FlyingBird<State>, _>(|e| {
                    e.get_handle() == "flying_bird"
                });
            let dead = game
                .elements
                .dcot_w::<elements::DeadBird<State>, _>(|e| e.get_handle() == "dead_bird");
            match state {
                State::Dead => {
                    for b in flying {
                        b.showed(false);
                    }
                    for b in dead {
                        b.showed(true);
                    }
                }
                _ => {
                    for b in flying {
                        b.showed(true);
                    }
                    for b in dead {
                        b.showed(false);
                    }
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
        if *state == State::Flying && matches!(event.key, Key::Enter | Key::Char(' ')) {
            for game in el
                .elements
                .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
            {
                game.flap();
            }
        }
    });

    app.on_mouse(|el, state, event| {
        if !matches!(event.mouse, Mouse::Down) {
            return;
        }
        if let Some(next) = transition(state, &Key::Enter) {
            if next != *state {
                *state = next;
                el.draw();
            }
        }
        if *state == State::Flying {
            for game in el
                .elements
                .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
            {
                game.flap();
            }
        }
    });

    app.on_loop(|el, state, _event| {
        if *state != State::Flying {
            return;
        }
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            if game.check_crash() {
                *state = State::Dead;
            }
        }
    });

    app.add(splash);
    app.add(game);

    app
}
