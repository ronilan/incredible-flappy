use std::rc::Rc;

use incredible::*;
use incredible_elements::{App, Button, FramedText, Image, Label, Rectangle};
use incredible_elements_text_fonts::BlockCharsStr;
use incredible_helpers_layout::*;

use crate::settings;
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
    Paused,
    Score,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {
    pub phase: Phase,
    pub kitty: bool,
    pub selected: SelectedGame,
    pub focus: Option<SelectedGame>,
    pub countdown: u8,
    pub countdown_at: f64,
    pub best_classic: [u32; 3],
    pub best_busy: [u32; 3],
    pub best_invaders: [u32; 3],
}

impl State {
    fn best_mut(&mut self, game: SelectedGame) -> &mut [u32; 3] {
        match game {
            SelectedGame::Classic => &mut self.best_classic,
            SelectedGame::Busy => &mut self.best_busy,
            SelectedGame::Invaders => &mut self.best_invaders,
        }
    }

    /// Best score for the given game.
    pub fn best_for(&self, game: SelectedGame) -> u32 {
        match game {
            SelectedGame::Classic => self.best_classic[0],
            SelectedGame::Busy => self.best_busy[0],
            SelectedGame::Invaders => self.best_invaders[0],
        }
    }

    /// Records a score into the top 3. Returns the medal rank
    /// (0 gold, 1 silver, 2 bronze) or None when it places outside.
    /// Scoreless runs earn nothing.
    pub fn record_score(&mut self, game: SelectedGame, score: u32) -> Option<usize> {
        if score == 0 {
            return None;
        }
        let top = self.best_mut(game);
        let rank = top.iter().filter(|s| **s > score).count();
        if rank >= top.len() {
            return None;
        }
        top[rank..].rotate_right(1);
        top[rank] = score;
        Some(rank)
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

/// Game buttons in focus-cycle order with their handles.
const FOCUS_ORDER: [(&str, SelectedGame); 3] = [
    ("game_basic", SelectedGame::Classic),
    ("game_graze", SelectedGame::Busy),
    ("game_shoot", SelectedGame::Invaders),
];

/// Kitty-mode image twins of the game buttons.
const IMAGE_ORDER: [(&str, SelectedGame); 3] = [
    ("game_basic_image", SelectedGame::Classic),
    ("game_graze_image", SelectedGame::Busy),
    ("game_shoot_image", SelectedGame::Invaders),
];

/// Mirrors splash focus state onto the game buttons.
fn apply_focus(el: &App<State>, state: &State) {
    apply_focus_to::<Button<State>>(el, state);
    apply_focus_to::<elements::ImageButton<State>>(el, state);
}

/// Mirrors splash focus state onto buttons of one concrete kind.
fn apply_focus_to<E>(el: &App<State>, state: &State)
where
    E: ElementTrait<State> + Getters<State> + Setters<State> + 'static,
{
    for btn in el.elements.dcot_w::<E, _>(|e| {
        FOCUS_ORDER.iter().any(|(h, _)| h == &e.get_handle())
            || IMAGE_ORDER.iter().any(|(h, _)| h == &e.get_handle())
    }) {
        let mine = FOCUS_ORDER
            .iter()
            .chain(IMAGE_ORDER.iter())
            .any(|(h, g)| h == &btn.get_handle() && Some(*g) == state.focus);
        btn.focused(mine);
    }
}

/// Focus cycle order, ending out on nothing selected.
const FOCUS_CYCLE: [Option<SelectedGame>; 4] = [
    Some(SelectedGame::Classic),
    Some(SelectedGame::Busy),
    Some(SelectedGame::Invaders),
    None,
];

/// Moves splash focus, wrapping through empty.
fn cycle_focus(el: &App<State>, state: &mut State, dir: isize) {
    let i = FOCUS_CYCLE
        .iter()
        .position(|g| *g == state.focus)
        .unwrap_or(3);
    let n = FOCUS_CYCLE.len() as isize;
    state.focus = FOCUS_CYCLE[((i as isize + dir + n) % n) as usize];
    apply_focus(el, state);
    sync_hint(el, state);
    el.draw();
}

/// Back-to-splash button hit on the selected game.
fn clicked_back_button(el: &App<State>, state: &State, x: isize, y: isize) -> bool {
    for game in selected_games(el, state) {
        for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
            e.get_handle() == "back_button"
        }) {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
        for btn in game
            .elements
            .dcot_w::<elements::ImageButton<State>, _>(|e| {
                e.get_handle() == "back_image_button"
            })
        {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
    }
    false
}

/// True when a back button holds game focus.
fn game_back_focused(el: &App<State>, state: &State) -> bool {
    for game in selected_games(el, state) {
        for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
            e.get_handle() == "back_button"
        }) {
            if btn.status().focused.get() {
                return true;
            }
        }
        for btn in game.elements.dcot_w::<elements::ImageButton<State>, _>(|e| {
            e.get_handle() == "back_image_button"
        }) {
            if btn.status().focused.get() {
                return true;
            }
        }
    }
    false
}

/// True when a play button holds game focus.
fn game_play_focused(el: &App<State>, state: &State) -> bool {
    for game in selected_games(el, state) {
        for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
            e.get_handle() == "play_button"
        }) {
            if btn.status().focused.get() {
                return true;
            }
        }
        for btn in game.elements.dcot_w::<elements::ImageButton<State>, _>(|e| {
            e.get_handle() == "play_image_button"
        }) {
            if btn.status().focused.get() {
                return true;
            }
        }
    }
    false
}

/// Flips game focus between play and back.
fn cycle_game_focus(el: &App<State>, state: &State) {
    let play = game_play_focused(el, state);
    for game in selected_games(el, state) {
        set_game_focus(&game, !play);
    }
}

/// Sets game focus on one game: play on, back off, or the reverse.
fn set_game_focus(game: &elements::Game<State>, play: bool) {
    for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
        e.get_handle() == "back_button" || e.get_handle() == "play_button"
    }) {
        btn.focused((btn.get_handle() == "play_button") == play);
    }
    for btn in game.elements.dcot_w::<elements::ImageButton<State>, _>(|e| {
        e.get_handle() == "back_image_button" || e.get_handle() == "play_image_button"
    }) {
        btn.focused((btn.get_handle() == "play_image_button") == play);
    }
}

/// Pause slot hit (top-left): text, pause art or resume art.
fn clicked_pause_slot(el: &App<State>, state: &State, x: isize, y: isize) -> bool {
    for game in selected_games(el, state) {
        for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
            e.get_handle() == "pause_button"
        }) {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
        for btn in game
            .elements
            .dcot_w::<elements::ImageButton<State>, _>(|e| {
                e.get_handle() == "pause_image_button" || e.get_handle() == "resume_image_button"
            })
        {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
    }
    false
}

/// Play button hit on the selected game.
fn clicked_play_button(el: &App<State>, state: &State, x: isize, y: isize) -> bool {
    for game in selected_games(el, state) {
        for btn in game.elements.dcot_w::<Button<State>, _>(|e| {
            e.get_handle() == "play_button"
        }) {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
        for btn in game
            .elements
            .dcot_w::<elements::ImageButton<State>, _>(|e| {
                e.get_handle() == "play_image_button"
            })
        {
            if !btn.status().showed.get() {
                continue;
            }
            let (bx, by) = (btn.get_x(), btn.get_y());
            if x >= bx
                && x < bx + btn.visual.look.width() as isize
                && y >= by
                && y < by + btn.visual.look.height() as isize
            {
                return true;
            }
        }
    }
    false
}

/// Starts the Ready countdown on the selected game.
fn start_countdown(el: &App<State>, state: &mut State) {
    for game in selected_games(el, state) {
        game.reset();
    }
    state.countdown = 3;
    state.countdown_at = Globals::now();
    for game in selected_games(el, state) {
        for light in game.elements.dcot_w::<elements::Stoplight<State>, _>(|e| {
            e.get_handle() == "stoplight"
        }) {
            light.set_lit(1);
        }
    }
    state.phase = Phase::Ready;
    el.draw();
}

/// Launches the game under the given button, if any sits there.
fn launch_clicked_button(el: &App<State>, state: &mut State, x: isize, y: isize) -> bool {
    if launch_clicked_button_of::<Button<State>>(el, state, x, y) {
        return true;
    }
    launch_clicked_button_of::<elements::ImageButton<State>>(el, state, x, y)
}

/// Launches the game under the given button of one concrete kind.
fn launch_clicked_button_of<E>(el: &App<State>, state: &mut State, x: isize, y: isize) -> bool
where
    E: ElementTrait<State> + Getters<State> + 'static,
{
    for btn in el.elements.dcot_w::<E, _>(|e| {
        FOCUS_ORDER.iter().any(|(h, _)| h == &e.get_handle())
            || IMAGE_ORDER.iter().any(|(h, _)| h == &e.get_handle())
    }) {
        let bx = btn.get_x();
        let by = btn.get_y();
        if x >= bx
            && x < bx + btn.visual().look.width() as isize
            && y >= by
            && y < by + btn.visual().look.height() as isize
        {
            if let Some((_, game)) = FOCUS_ORDER
                .iter()
                .chain(IMAGE_ORDER.iter())
                .find(|(h, _)| h == &btn.get_handle())
            {
                state.focus = Some(*game);
                state.selected = *game;
                apply_focus(el, state);
                return true;
            }
        }
    }
    false
}

/// Refreshes the splash hint from the focused button.
fn sync_hint(el: &App<State>, state: &mut State) {
    for hint in el
        .elements
        .dcot_w::<FramedText<State>, _>(|e| e.get_handle() == "game_hint")
    {
        hint.text(screens::splash::hint::game_hint_for(&state.focus));
    }
    el.draw();
}
pub fn transition(phase: &Phase, key: &Key) -> Option<Phase> {
    match (key, phase) {
        (Key::Enter, Phase::Splash) => Some(Phase::Ready),
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
            game.set_running(true);
            game.set_spawning(false);
            set_game_focus(game, true);
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
        score.showed(state.phase == Phase::Flying && !state.kitty);
    }
    for image in game
        .elements
        .dcot_w::<elements::U16Image<State>, _>(|e| e.get_handle() == "image_score")
    {
        image.showed(state.phase == Phase::Flying && state.kitty);
    }
    for back in game
        .elements
        .dcot_w::<Button<State>, _>(|e| e.get_handle() == "back_button")
    {
        back.showed(state.phase == Phase::Score && !state.kitty);
    }
    for back in game
        .elements
        .dcot_w::<elements::ImageButton<State>, _>(|e| e.get_handle() == "back_image_button")
    {
        back.showed(state.phase == Phase::Score && state.kitty);
    }
    for play in game
        .elements
        .dcot_w::<Button<State>, _>(|e| e.get_handle() == "play_button")
    {
        play.showed(state.phase == Phase::Score && !state.kitty);
    }
    for play in game
        .elements
        .dcot_w::<elements::ImageButton<State>, _>(|e| e.get_handle() == "play_image_button")
    {
        play.showed(state.phase == Phase::Score && state.kitty);
    }
    for pause in game
        .elements
        .dcot_w::<Button<State>, _>(|e| e.get_handle() == "pause_button")
    {
        pause.showed(
            (state.phase == Phase::Flying || state.phase == Phase::Paused) && !state.kitty,
        );
        pause.text(if state.phase == Phase::Paused {
            ">"
        } else {
            "II"
        });
        pause.focused(state.phase == Phase::Flying);
    }
    for pause in game
        .elements
        .dcot_w::<elements::ImageButton<State>, _>(|e| e.get_handle() == "pause_image_button")
    {
        pause.showed(state.phase == Phase::Flying && state.kitty);
        pause.focused(state.phase == Phase::Flying);
    }
    for resume in game
        .elements
        .dcot_w::<elements::ImageButton<State>, _>(|e| e.get_handle() == "resume_image_button")
    {
        resume.showed(state.phase == Phase::Paused && state.kitty);
        resume.focused(state.phase == Phase::Paused);
    }
    for panel in game
        .elements
        .dcot_w::<elements::ScorePanel<State>, _>(|e| e.get_handle() == "score_panel")
    {
        panel.showed(state.phase == Phase::Score);
    }
    for light in game
        .elements
        .dcot_w::<elements::Stoplight<State>, _>(|e| e.get_handle() == "stoplight")
    {
        light.showed(state.phase == Phase::Ready);
        if state.phase == Phase::Ready {
            light.set_lit(4 - state.countdown.min(3));
        }
    }
    for frame in game
        .elements
        .dcot_w::<incredible_elements::Frame<State>, _>(|e| e.get_handle() == "score_panel_frame")
    {
        frame.showed(!state.kitty);
    }
    for image in game
        .elements
        .dcot_w::<Image<State>, _>(|e| e.get_handle() == "score_panel_image")
    {
        image.showed(state.kitty);
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
        .dcot_w::<elements::BoxedTextFont<State>, _>(|e| e.get_handle() == "ready_title");
    for r in ready {
        r.showed(state.phase == Phase::Ready && !state.kitty);
    }
    for over in game
        .elements
        .dcot_w::<elements::BoxedTextFont<State>, _>(|e| e.get_handle() == "gameover_title")
    {
        over.showed(state.phase == Phase::Score && !state.kitty);
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
        img.showed(state.phase == Phase::Score && state.kitty);
    }
    let flying_cat = game
        .elements
        .dcot_w::<elements::FlyingCat<State>, _>(|e| {
            e.get_handle() == "flying_cat"
        });
    let dead_cat = game
        .elements
        .dcot_w::<elements::DeadCat<State>, _>(|e| e.get_handle() == "dead_cat");
    let is_dead = state.phase == Phase::Score;
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
        if matches!(event.key, Key::Char('k') | Key::Char('K'))
            && Platform::output_provider().images()
        {
            state.kitty = !state.kitty;
            settings::persist_now(state);
            el.draw();
        }
        if state.phase == Phase::Splash && matches!(event.key, Key::Enter) {
            if let Some(game) = state.focus {
                state.selected = game;
                start_countdown(el, state);
            }
        } else if state.phase == Phase::Splash && matches!(event.key, Key::Tab) {
            cycle_focus(el, state, 1);
        } else if state.phase == Phase::Splash && matches!(event.key, Key::BackTab) {
            cycle_focus(el, state, -1);
        } else if state.phase == Phase::Score && matches!(event.key, Key::Enter) {
            // New games wait for the score panel slide and play focus,
            // then count down again.
            if game_play_focused(el, state)
                && selected_games(el, state)
                    .iter()
                    .all(|game| game.panel_settled())
            {
                start_countdown(el, state);
            }
        } else if state.phase == Phase::Score
            && matches!(event.key, Key::Enter)
            && game_back_focused(el, state)
        {
            state.phase = Phase::Splash;
            el.draw();
        } else if state.phase == Phase::Ready && matches!(event.key, Key::Enter | Key::Char(' ')) {
            // Enter and Space skip the countdown: straight up with a bump.
            state.phase = Phase::Flying;
            for game in selected_games(el, state) {
                game.flap();
                game.shoot();
            }
            el.draw();
        } else if state.phase == Phase::Flying && matches!(event.key, Key::Escape) {
            for game in selected_games(el, state) {
                game.still();
            }
            state.phase = Phase::Paused;
            el.draw();
        } else if state.phase == Phase::Paused && matches!(event.key, Key::Enter) {
            state.phase = Phase::Flying;
            el.draw();
            return;
        } else if state.phase == Phase::Score && matches!(event.key, Key::Tab) {
            cycle_game_focus(el, state);
            el.draw();
        } else if state.phase == Phase::Score && matches!(event.key, Key::BackTab) {
            cycle_game_focus(el, state);
            el.draw();
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
        // Splash clicks launch the clicked game button, never anything else.
        if state.phase == Phase::Splash {
            if matches!(event.mouse, Mouse::Click)
                && launch_clicked_button(el, state, event.x, event.y)
            {
                start_countdown(el, state);
                return;
            }
            sync_hint(el, state);
            return;
        }
        if !matches!(event.mouse, Mouse::Click) {
            return;
        }
        if state.phase != Phase::Splash && clicked_back_button(el, state, event.x, event.y) {
            state.phase = Phase::Splash;
            el.draw();
            return;
        }
        if state.phase == Phase::Score {
            // Click restarts like Enter: play button, settled slide,
            // then counts down again.
            if clicked_play_button(el, state, event.x, event.y)
                && selected_games(el, state)
                    .iter()
                    .all(|game| game.panel_settled())
            {
                start_countdown(el, state);
            }
            return;
        }
        if state.phase == Phase::Ready {
            // Clicks bump like Enter and Space: straight to Flying.
            if matches!(event.mouse, Mouse::Click) {
                state.phase = Phase::Flying;
                for game in selected_games(el, state) {
                    game.flap();
                    game.shoot();
                }
                el.draw();
            }
            return;
        }
        if state.phase == Phase::Flying && clicked_pause_slot(el, state, event.x, event.y) {
            for game in selected_games(el, state) {
                game.still();
            }
            state.phase = Phase::Paused;
            el.draw();
            return;
        }
        if state.phase == Phase::Paused && clicked_pause_slot(el, state, event.x, event.y) {
            state.phase = Phase::Flying;
            el.draw();
            return;
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
        if state.phase == Phase::Ready {
            // Countdown ticks down in whole seconds, then we fly.
            if Globals::now() - state.countdown_at >= 1000.0 {
                state.countdown_at = Globals::now();
                if state.countdown > 1 {
                    state.countdown -= 1;
                } else {
                    state.countdown = 0;
                    state.phase = Phase::Flying;
                }
                el.draw();
            }
            return;
        }
        if state.phase == Phase::Score {
            let mut moving = false;
            for game in selected_games(el, state) {
                moving |= game.slide_panel();
            }
            if moving {
                el.draw();
            }
            return;
        }
        for game in selected_games(el, state) {
            if game.check_crash() {
                let score = game.score_value();
                let medal = state.record_score(state.selected, score);
                game.clear_bullets();
                for panel in game.elements.cot::<elements::ScorePanel<State>>() {
                    panel.set_result(score, state.best_for(state.selected), medal);
                }
                game.park_panel();
                set_game_focus(&game, true);
                state.phase = Phase::Score;
                settings::persist_now(state);
                el.draw();
            }
        }
    }).on_state(|el, state, _event| {
        let is_splash = state.phase == Phase::Splash;
        let classic = state.selected == SelectedGame::Classic;
        let busy = state.selected == SelectedGame::Busy;
        if is_splash {
            apply_focus(el, state);
        }
        for rect in el
            .elements
            .dcot_w::<Rectangle<State>, _>(|e| e.get_handle() == "splash")
        {
            rect.showed(is_splash);
        }
        let for_hint = if is_splash {
            state.focus
        } else {
            Some(state.selected)
        };
        for hint in el
            .elements
            .dcot_w::<FramedText<State>, _>(|e| e.get_handle() == "game_hint")
        {
            hint.text(screens::splash::hint::game_hint_for(&for_hint));
        }
        // Kitty needs images; hide its teaser where unsupported.
        for line in el
            .elements
            .dcot_w::<Label<State>, _>(|e| e.get_handle() == "kitty_hint")
        {
            line.showed(Platform::output_provider().images());
        }
        for title in el
            .elements
            .dcot_w::<BlockCharsStr<State>, _>(|e| e.get_handle() == "flappy_title")
        {
            title.showed(!state.kitty);
        }
        for btn in el.elements.dcot_w::<Button<State>, _>(|e| {
            FOCUS_ORDER.iter().any(|(h, _)| h == &e.get_handle())
        }) {
            btn.showed(!state.kitty);
        }
        for btn in el.elements.dcot_w::<elements::ImageButton<State>, _>(|e| {
            IMAGE_ORDER.iter().any(|(h, _)| h == &e.get_handle())
        }) {
            btn.showed(state.kitty);
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
