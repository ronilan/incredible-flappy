use incredible::*;
use incredible_elements::{App, Rectangle};
use incredible_elements_text_fonts::BlockCharsStr;
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;

use crate::ui::elements;
use crate::ui::screens;

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

    transform_rule("FlappyGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(156), Color::ansi(46)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
    });
    transform_rule("ReadyGradient", |flattened, progress| {
        gradient_color(
            &[Color::ansi(214), Color::ansi(220)],
            GradientDirection::Vertical,
            flattened,
            progress,
        )
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
    }).on_mouse(|el, state, event| {
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
    }).on_window(|el, _state, _event| {
        el.elements_to_center();
    }).on_loop(|el, state, _event| {
        if *state != State::Flying {
            return;
        }
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            if game.check_crash() {
                *state = State::Dead;
                el.draw();
            }
        }
    }).on_state(|el, state, _event| {
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
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            match state {
                State::Ready => {
                    game.reset();
                    game.set_running(true);
                    game.set_spawning(false);
                }
                State::Flying => {
                    game.set_running(true);
                    game.set_spawning(true);
                }
                _ => {
                    game.set_running(false);
                    game.set_spawning(false);
                }
            }
            for score in game
                .elements
                .dcot_w::<elements::Score<State>, _>(|e| e.get_handle() == "score")
            {
                score.showed(*state == State::Flying || *state == State::Dead);
            }
            let flying = game
                .elements
                .dcot_w::<elements::FlyingBird<State>, _>(|e| {
                    e.get_handle() == "flying_bird"
                });
            let dead = game
                .elements
                .dcot_w::<elements::DeadBird<State>, _>(|e| e.get_handle() == "dead_bird");
            let ready = game
                .elements
                .dcot_w::<BlockCharsStr<State>, _>(|e| e.get_handle() == "ready_title");
            for r in ready {
                r.showed(*state == State::Ready);
            }
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


    // Splash screen: 80x24 rectangle.
    let splash = screens::splash::splash::build();
    app.add(splash);

    // Game screen: the scroller.
    let game = screens::game::game::build();
    app.add(game);

    app
}
