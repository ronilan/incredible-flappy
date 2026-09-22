use std::rc::Rc;

use incredible::*;
use incredible_elements::{App, FramedText, Image, Rectangle, Select};
use incredible_elements_text_fonts::BlockCharsStr;
use incredible_helpers_layout::*;

use crate::ui::elements;
use crate::ui::screens;
use crate::ui::settings;
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
    pub hinted: SelectedGame,
    pub best_classic: u32,
    pub best_busy: u32,
    pub best_invaders: u32,
}

impl State {
    /// Best score for the given game.
    pub fn best_for(&self, game: SelectedGame) -> u32 {
        match game {
            SelectedGame::Classic => self.best_classic,
            SelectedGame::Busy => self.best_busy,
            SelectedGame::Invaders => self.best_invaders,
        }
    }

    /// Records a best score for the given game.
    pub fn set_best(&mut self, game: SelectedGame, value: u32) {
        match game {
            SelectedGame::Classic => self.best_classic = value,
            SelectedGame::Busy => self.best_busy = value,
            SelectedGame::Invaders => self.best_invaders = value,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SelectedGame {
    #[default]
    Classic,
    Busy,
    Invaders,
}

/// Palette per selected game.
pub(crate) fn palette_for(selected: &SelectedGame) -> theme::Palette {
    match selected {
        SelectedGame::Classic => theme::CLASSIC_PALETTE,
        SelectedGame::Busy => theme::BUSY_PALETTE,
        SelectedGame::Invaders => theme::INVADERS_PALETTE,
    }
}

/// Launches whichever game the splash select points at.
fn launch_from_select(el: &App<State>, state: &mut State) {
    if let Some(select) = el
        .elements
        .dcot_w::<Select<State>, _>(|e| e.get_handle() == "game_select")
        .first()
    {
        let idx = select.get_selected().or_else(|| select.get_focused_index());
        if let Some(idx) = idx {
            if let Some(val) = select.item_value(idx) {
                state.selected = game_for_value(val.as_str());
                state.phase = Phase::Ready;
                el.draw();
            }
        }
    }
}

/// Maps a select item value to its game.
fn game_for_value(val: &str) -> SelectedGame {
    match val {
        "busy" => SelectedGame::Busy,
        "invaders" => SelectedGame::Invaders,
        _ => SelectedGame::Classic,
    }
}

/// Game the splash cursor points at right now.
fn hovered_game(el: &App<State>) -> SelectedGame {
    if let Some(select) = el
        .elements
        .dcot_w::<Select<State>, _>(|e| e.get_handle() == "game_select")
        .first()
    {
        let idx = select
            .get_focused_index()
            .or_else(|| select.get_selected());
        if let Some(idx) = idx {
            if let Some(val) = select.item_value(idx) {
                return game_for_value(val.as_str());
            }
        }
    }
    SelectedGame::Classic
}
pub fn transition(phase: &Phase, key: &Key) -> Option<Phase> {
    match (key, phase) {
        (Key::Escape, _) => Some(Phase::Splash),
        (Key::Enter, Phase::Splash) => Some(Phase::Ready),
        (Key::Enter | Key::Char(' '), Phase::Dead) => Some(Phase::Ready),
        (Key::Enter | Key::Char(' '), Phase::Ready) => Some(Phase::Flying),
        _ => None,
    }
}

/// Games of the selected screen: three independent clones,
/// each a direct app child under its own handle.
fn selected_games(el: &App<State>, state: &State) -> Vec<Rc<elements::Game<State>>> {
    let handle = match state.selected {
        SelectedGame::Classic => "game",
        SelectedGame::Busy => "busy_game",
        SelectedGame::Invaders => "invaders_game",
    };
    el.elements
        .cot::<elements::Game<State>>()
        .into_iter()
        .filter(|g| g.get_handle() == handle)
        .collect()
}

/// Drives one game from app state: sim flags, kitty, score and creatures.
fn drive_game(game: &elements::Game<State>, state: &State) {
    match state.phase {
        Phase::Ready => {
            game.set_kind(state.selected);
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
        score.showed((state.phase == Phase::Flying || state.phase == Phase::Dead) && !state.kitty);
    }
    for image in game
        .elements
        .dcot_w::<elements::U16Image<State>, _>(|e| e.get_handle() == "image_score")
    {
        image.showed((state.phase == Phase::Flying || state.phase == Phase::Dead) && state.kitty);
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
        r.showed(state.phase == Phase::Ready && !state.kitty);
    }
    for over in game
        .elements
        .dcot_w::<BlockCharsStr<State>, _>(|e| e.get_handle() == "gameover_title")
    {
        over.showed(state.phase == Phase::Dead && !state.kitty);
    }
    for img in game
        .elements
        .dcot_w::<Image<State>, _>(|e| e.get_handle() == "ready_image")
    {
        img.showed(state.phase == Phase::Ready && state.kitty);
    }
    for img in game
        .elements
        .dcot_w::<Image<State>, _>(|e| e.get_handle() == "gameover_image")
    {
        img.showed(state.phase == Phase::Dead && state.kitty);
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

pub fn build() -> App<State> {
    let app = App::default();
    app.showed(false);

    theme::theme_all();

    app.on_key(|el, state: &mut State, event| {
        if matches!(event.key, Key::Char('k') | Key::Char('K')) {
            state.kitty = !state.kitty;
            settings::persist_now(state);
            el.draw();
        }
        if state.phase == Phase::Splash && matches!(event.key, Key::Enter) {
            launch_from_select(el, state);
        } else if let Some(next) = transition(&state.phase, &event.key) {
            if next != state.phase {
                state.phase = next;
                el.draw();
            }
        }
        if state.phase == Phase::Flying && matches!(event.key, Key::Enter | Key::Char(' ')) {
            for game in selected_games(el, state) {
                game.flap();
                game.shoot();
            }
        }
    }).on_mouse(|el, state, event| {
        // Splash launching is Enter-only now; clicks never start the game.
        if state.phase == Phase::Splash {
            return;
        }
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
            for game in selected_games(el, state) {
                game.flap();
                game.shoot();
            }
        }
    }).on_window(|el, _state, event| {
        if event.window == Window::Resize {
            el.showed(true);
            el.elements_to_center();
            el.draw();
        }

        
    }).on_loop(|el, state, _event| {
        if state.phase == Phase::Splash {
            let hovered = hovered_game(el);
            if hovered != state.hinted {
                state.hinted = hovered;
                for hint in el
                    .elements
                    .dcot_w::<FramedText<State>, _>(|e| e.get_handle() == "game_hint")
                {
                    hint.text(screens::splash::splash::game_hint_for(&hovered));
                }
                el.draw();
            }
            return;
        }
        if state.phase != Phase::Flying {
            return;
        }
        for game in selected_games(el, state) {
            if game.check_crash() {
                let score = game.score_value();
                if score > state.best_for(state.selected) {
                    state.set_best(state.selected, score);
                }
                state.phase = Phase::Dead;
                settings::persist_now(state);
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
        let hinted = if is_splash {
            hovered_game(el)
        } else {
            state.selected
        };
        for hint in el
            .elements
            .dcot_w::<FramedText<State>, _>(|e| e.get_handle() == "game_hint")
        {
            hint.text(screens::splash::splash::game_hint_for(&hinted));
        }
        for title in el
            .elements
            .dcot_w::<BlockCharsStr<State>, _>(|e| e.get_handle() == "flappy_title")
        {
            title.showed(!state.kitty);
        }
        for fluffy in el
            .elements
            .dcot_w::<Image<State>, _>(|e| e.get_handle() == "fluffy_title")
        {
            fluffy.showed(state.kitty);
        }
        for rect in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "game")
        {
            rect.showed(!is_splash && classic);
        }
        for rect in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "busy_game")
        {
            rect.showed(!is_splash && busy);
        }
        for rect in el
            .elements
            .dcot_w::<elements::Game<State>, _>(|e| e.get_handle() == "invaders_game")
        {
            rect.showed(!is_splash && !classic && !busy);
        }
        for game in el.elements.cot::<elements::Game<State>>() {
            let handle = game.get_handle();
            let mine = (handle == "game" && classic)
                || (handle == "busy_game" && busy)
                || (handle == "invaders_game" && !classic && !busy);
            if mine {
                drive_game(&game, state);
            } else {
                game.set_running(false);
                game.set_spawning(false);
            }
        }
        el.draw();
    });


    // Splash screen: 80x24 rectangle.
    let splash = screens::splash::splash::build();
    app.add(splash);

    // Game screen: the shared scroller.
    let game = screens::game::game::build();
    app.add(game);

    // Busy game: independent clone, purple palette.
    let busy = screens::game::game::build_as("busy_game");
    app.add(busy);

    // Invaders game: independent clone, invaders palette.
    let invaders = screens::game::game::build_as("invaders_game");
    app.add(invaders);

    app
}
