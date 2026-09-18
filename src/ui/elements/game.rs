use std::cell::Cell;

use incredible::*;
use incredible_elements::Text;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use super::bushing::{Bushing, BUSHING_WIDTH};
use super::dead_bird::DeadBird;
use super::flying_bird::FlyingBird;
use super::pipe::{PIPE_WIDTH, Pipe};
use super::scenery::{SCENERY_WIDTH, Scenery};

pub const GAME_WIDTH: usize = 80;
pub const GAME_HEIGHT: usize = 24;
pub const GAME_GROUND_Y: isize = 16;
pub const GAME_SPAWN_X: isize = 80;
pub const GAME_PIPE_Y: isize = 12;
pub const GAME_BUSHING_Y: isize = 11;

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub background: u8,
    pub interval_ms: u128,
    pub spawn_gap: usize,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            background: 152,
            interval_ms: 100,
            spawn_gap: 30,
        }
    }
}

element! {
  pub struct Game<S> {
      options: GameOptions = GameOptions::default(),
      running: Cell<bool> = Cell::new(false),
      distance: Cell<usize> = Cell::new(0),
  }
}

impl<S: Clone + PartialEq> Game<S> {
    pub fn new(options: GameOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((GAME_WIDTH, GAME_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("game");

        let title = Text::<S>::default();
        title.text("").handle("game_title").x(2).y(2);
        el.add(title);

        let hint = Text::<S>::default();
        hint.text("").handle("game_hint").x(2).y(4);
        el.add(hint);

        let flying_bird = FlyingBird::<S>::default();
        flying_bird.x(40).y(6);
        el.add(flying_bird);

        let dead_bird = DeadBird::<S>::default();
        dead_bird.x(48).y(6);
        el.add(dead_bird);

        // Always three ground tiles.
        el.add_ground();

        // Marquee method: step on animation progress each loop tick.
        el.internal_on_loop(|el, _, _event| {
            if !el.running.get() {
                return;
            }
            if let Some(anim) = el.get_animation() {
                if anim.progress.is_some() {
                    el.step();
                    el.draw();
                }
            }
        });

        el.refresh();
        el.decorate();

        el
    }

    /// (Re)arms the step animation from the configured interval.
    pub fn refresh(&self) -> &Self {
        let interval = self.options.interval_ms as f64;
        self.animation(Some(Animation::new(
            interval * 2.0,
            interval,
            f64::INFINITY,
        )));
        self
    }

    /// Starts (or stops) scrolling. Starting spawns the first pipe pair.
    pub fn set_running(&self, running: bool) -> &Self {
        let was = self.running.get();
        self.running.set(running);
        if running && !was {
            self.spawn_obstacle();
            self.distance.set(0);
        }
        self
    }

    /// Clears pipes and restores the three ground tiles.
    pub fn reset(&self) -> &Self {
        while self.elements.sot::<Pipe<S>>().is_some() {}
        while self.elements.sot::<Bushing<S>>().is_some() {}
        while self.elements.sot::<Scenery<S>>().is_some() {}
        self.add_ground();
        self.distance.set(0);
        self
    }

    /// Three 40-wide tiles across and past the window.
    fn add_ground(&self) {
        for x in [0, SCENERY_WIDTH as isize, SCENERY_WIDTH as isize * 2] {
            let tile = Scenery::<S>::default();
            tile.x(x).y(GAME_GROUND_Y);
            self.add(tile);
        }
    }

    /// Places a pipe + bushing cap at x 80.
    fn spawn_obstacle(&self) {
        let pipe = Pipe::<S>::default();
        pipe.x(GAME_SPAWN_X).y(GAME_PIPE_Y);
        self.add(pipe);

        let bushing = Bushing::<S>::default();
        bushing.x(GAME_SPAWN_X - 1).y(GAME_BUSHING_Y);
        self.add(bushing);
    }

    /// Moves the world one cell left, wraps ground, spawns and
    /// removes pipes. Title, hint and birds stay put.
    pub fn step(&self) {
        for tile in self.elements.cot::<Scenery<S>>() {
            tile.x(tile.get_x() - 1);
        }
        for pipe in self.elements.cot::<Pipe<S>>() {
            pipe.x(pipe.get_x() - 1);
        }
        for bushing in self.elements.cot::<Bushing<S>>() {
            bushing.x(bushing.get_x() - 1);
        }

        // Leftmost at -40 wraps to 80.
        for tile in self.elements.cot::<Scenery<S>>() {
            if tile.get_x() <= -(SCENERY_WIDTH as isize) {
                tile.x(GAME_WIDTH as isize);
            }
        }

        // Pipe (6) + gap (24) cadence.
        let distance = self.distance.get() + 1;
        if distance >= self.options.spawn_gap {
            self.spawn_obstacle();
            self.distance.set(0);
        } else {
            self.distance.set(distance);
        }

        // Drop fully off-screen pipes.
        while self
            .elements
            .sot_w::<Pipe<S>, _>(|p| p.get_x() + PIPE_WIDTH as isize <= 0)
            .is_some()
        {}
        while self
            .elements
            .sot_w::<Bushing<S>, _>(|b| b.get_x() + BUSHING_WIDTH as isize <= 0)
            .is_some()
        {}
    }
}

impl<S: Clone + PartialEq> Default for Game<S> {
    fn default() -> Self {
        Self::new(GameOptions::default())
    }
}
