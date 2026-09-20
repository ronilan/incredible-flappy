use incredible::*;
use incredible_elements::{App, Rectangle};
use incredible_elements_text_fonts::BlockCharsStr;
use incredible_helpers_layout::*;

use crate::ui::elements;
use crate::ui::screens;
use crate::ui::theme;

pub const SCREEN_WIDTH: usize = 80;
pub const SCREEN_HEIGHT: usize = 24;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Phase {
    #[default]
    Splash,
    Ready,
    Flying,
    Dead,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {
    pub phase: Phase,
    pub kitty: bool,
    pub selected: SelectedGame,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum SelectedGame {
    #[default]
    Classic,
    Busy,
    Invaders,
}

pub fn transition(phase: &Phase, key: &Key) -> Option<Phase> {
    match (key, phase) {
        (Key::Escape, _) => Some(Phase::Splash),
        (Key::Enter | Key::Char(' '), Phase::Splash) => Some(Phase::Ready),
        (Key::Enter | Key::Char(' '), Phase::Dead) => Some(Phase::Ready),
        (Key::Enter | Key::Char(' '), Phase::Ready) => Some(Phase::Flying),
        _ => None,
    }
}

pub fn build() -> App<State> {
    let app = App::default();

    theme::theme_all();

    app.on_key(|el, state: &mut State, event| {
        if matches!(event.key, Key::Char('k') | Key::Char('K')) {
            state.kitty = !state.kitty;
            el.draw();
        }
        if let Some(next) = transition(&state.phase, &event.key) {
            if next != state.phase {
                state.phase = next;
                el.draw();
            }
        }
        if state.phase == Phase::Flying && matches!(event.key, Key::Enter | Key::Char(' ')) {
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
        if let Some(next) = transition(&state.phase, &Key::Enter) {
            if next != state.phase {
                state.phase = next;
                el.draw();
            }
        }
        if state.phase == Phase::Flying {
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
        if state.phase != Phase::Flying {
            return;
        }
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            if game.check_crash() {
                state.phase = Phase::Dead;
                el.draw();
            }
        }
    }).on_state(|el, state, _event| {
        let is_splash = state.phase == Phase::Splash;
        let classic = state.selected == SelectedGame::Classic;
        let busy = state.selected == SelectedGame::Busy;
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
            rect.showed(!is_splash && classic);
        }
        for rect in el
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "busy")
        {
            rect.showed(!is_splash && busy);
        }
        for rect in el
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "invaders")
        {
            rect.showed(!is_splash && !classic && !busy);
        }
        for game in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            if state.selected != SelectedGame::Classic {
                game.set_running(false);
                game.set_spawning(false);
                continue;
            }
            match state.phase {
                Phase::Ready => {
                    game.reset();
                    game.set_running(true);
                    game.set_spawning(false);
                }
                Phase::Flying => {
                    game.set_running(true);
                    game.set_spawning(true);
                }
                _ => {
                    game.set_running(false);
                    game.set_spawning(false);
                }
            }
            game.set_kitty(state.kitty);
            for score in game
                .elements
                .dcot_w::<elements::Score<State>, _>(|e| e.get_handle() == "score")
            {
                score.showed(state.phase == Phase::Flying || state.phase == Phase::Dead);
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
                r.showed(state.phase == Phase::Ready);
            }
            let flying_cat = game
                .elements
                .dcot_w::<elements::FlyingCat<State>, _>(|e| {
                    e.get_handle() == "flying_cat"
                });
            let dead_cat = game
                .elements
                .dcot_w::<elements::DeadCat<State>, _>(|e| e.get_handle() == "dead_cat");
            let is_dead = state.phase == Phase::Dead;
            for b in flying {
                b.showed(!is_dead && !state.kitty);
            }
            for b in flying_cat {
                b.showed(!is_dead && state.kitty);
            }
            for b in dead {
                b.showed(is_dead && !state.kitty);
            }
            for b in dead_cat {
                b.showed(is_dead && state.kitty);
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

    // Stub screens for the next games.
    let busy = screens::busy::busy::build();
    busy.showed(false);
    app.add(busy);

    let invaders = screens::invaders::invaders::build();
    invaders.showed(false);
    app.add(invaders);

    app
}
