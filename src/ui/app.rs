use incredible::*;
use incredible_elements::{App, Rectangle, Text};
use incredible_helpers_layout::*;

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
        el.elements_flow_down(1);
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

    // Static game elements (hand-drawn reusable elements).
    let buildings = elements::Buildings::<State>::default();
    buildings.x(2).y(6);
    game.add(buildings);

    let flying_bird = elements::FlyingBird::<State>::default();
    flying_bird.x(40).y(6);
    game.add(flying_bird);

    let dead_bird = elements::DeadBird::<State>::default();
    dead_bird.x(48).y(6);
    game.add(dead_bird);

    let bush = elements::Bushes::<State>::default();
    bush.x(40).y(20);
    game.add(bush);

    let pipe = elements::Pipe::<State>::default();
    pipe.x(2).y(12);
    game.add(pipe);

    let bushing = elements::Bushing::<State>::default();
    bushing.x(12).y(12);
    game.add(bushing);

    let pavement = elements::Pavement::<State>::default();
    pavement.x(0).y(22);
    game.add(pavement);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_splash() {
        assert_eq!(State::default(), State::Splash);
    }

    #[test]
    fn screens_are_80x24() {
        Globals::draw_enabled(false);
        let app = build();
        let splash = app
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "splash");
        let game = app
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "game");
        assert_eq!(splash.len(), 1);
        assert_eq!(game.len(), 1);
        assert_eq!(splash[0].get_width(), SCREEN_WIDTH);
        assert_eq!(splash[0].get_height(), SCREEN_HEIGHT);
        assert_eq!(game[0].get_width(), SCREEN_WIDTH);
        assert_eq!(game[0].get_height(), SCREEN_HEIGHT);
        assert_eq!(SCREEN_WIDTH, 80);
        assert_eq!(SCREEN_HEIGHT, 24);
    }

    #[test]
    fn transitions_match_spec() {
        // Starts with Splash, Enter moves to Game-Ready.
        assert_eq!(transition(&State::Splash, &Key::Enter), Some(State::Ready));
        // Space moves to Flying.
        assert_eq!(
            transition(&State::Ready, &Key::Char(' ')),
            Some(State::Flying)
        );
        // Dead wired to d.
        assert_eq!(
            transition(&State::Flying, &Key::Char('d')),
            Some(State::Dead)
        );
        assert_eq!(
            transition(&State::Ready, &Key::Char('d')),
            Some(State::Dead)
        );
        // Enter moves to Ready (from Dead).
        assert_eq!(transition(&State::Dead, &Key::Enter), Some(State::Ready));
        // Esc in any state moves to Splash.
        for s in [State::Splash, State::Ready, State::Flying, State::Dead] {
            assert_eq!(transition(&s, &Key::Escape), Some(State::Splash));
        }
    }

    #[test]
    fn game_screen_presents_all_elements() {
        Globals::draw_enabled(false);
        let app = build();
        let game = app
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "game");
        assert_eq!(game.len(), 1);
        let inside = &game[0].elements;
        assert_eq!(
            inside
                .dcot_w::<elements::Pipe<State>, _>(|e| e.get_handle() == "pipe")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::Bushing<State>, _>(|e| e.get_handle() == "bushing")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::FlyingBird<State>, _>(|e| e.get_handle()
                    == "flying_bird")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::DeadBird<State>, _>(|e| e.get_handle() == "dead_bird")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::Bushes<State>, _>(|e| e.get_handle() == "bushes")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::Pavement<State>, _>(|e| e.get_handle() == "pavement")
                .len(),
            1
        );
        assert_eq!(
            inside
                .dcot_w::<elements::Buildings<State>, _>(|e| e.get_handle()
                    == "buildings")
                .len(),
            1
        );
    }
}
