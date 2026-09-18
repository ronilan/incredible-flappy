use std::cell::Cell;

use incredible::*;
use incredible_elements::Text;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;
use rand::Rng;
use rand::rng;

use super::bushing::{Bushing, BUSHING_WIDTH};
use super::dead_bird::DeadBird;
use super::floor::Floor;
use super::flying_bird::FlyingBird;
use super::pavement::Pavement;
use super::pipe::{PIPE_WIDTH, Pipe};
use super::scenery::{SCENERY_WIDTH, Scenery};

pub const GAME_WIDTH: usize = 80;
pub const GAME_HEIGHT: usize = 24;
pub const GAME_GROUND_Y: isize = 16;
pub const GAME_SPAWN_X: isize = 80;
pub const GAME_GAP_ROWS: isize = 11;
pub const GAME_GROUND_TOP_ROW: isize = 20;
pub const DEAD_REST_Y: f32 = 17.0;

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub background: u8,
    pub interval_ms: u128,
    pub spawn_gap: usize,
    pub physics: BirdPhysics,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            background: 152,
            interval_ms: 100,
            spawn_gap: 30,
            physics: BirdPhysics::default(),
        }
    }
}

/// Tweakable bird flight params. All in cells and loop steps.
#[derive(Clone, Debug)]
pub struct BirdPhysics {
    /// Downward pull added to velocity every step.
    pub gravity: f32,
    /// Upward impulse on flap (applied negative).
    pub flap: f32,
    /// Terminal fall speed.
    pub max_fall: f32,
    /// Bird y at run start. x stays 20.
    pub start_y: f32,
}

impl Default for BirdPhysics {
    fn default() -> Self {
        Self {
            gravity: 0.5,
            flap: 3.0,
            max_fall: 3.0,
            start_y: 10.0,
        }
    }
}

element! {
  pub struct Game<S> {
      options: GameOptions = GameOptions::default(),
      running: Cell<bool> = Cell::new(false),
      distance: Cell<usize> = Cell::new(0),
      bird_y: Cell<f32> = Cell::new(10.0),
      velocity: Cell<f32> = Cell::new(0.0),
      dying: Cell<bool> = Cell::new(false),
      crashed: Cell<bool> = Cell::new(false),
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
        flying_bird.x(20).y(el.options.physics.start_y as isize);
        el.add(flying_bird);

        let dead_bird = DeadBird::<S>::default();
        dead_bird.x(48).y(6);
        dead_bird.showed(false);

        el.bird_y.set(el.options.physics.start_y);

        // Always three ground tiles.
        el.add_ground();

        // After the ground so it paints on top of it.
        el.add(dead_bird);

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

    /// Starts (or stops) scrolling. Starting spawns the first pipe duo.
    pub fn set_running(&self, running: bool) -> &Self {
        let was = self.running.get();
        self.running.set(running);
        if running && !was {
            self.spawn_obstacle_at(GAME_SPAWN_X);
            self.distance.set(0);
        }
        self
    }

    /// Clears pipes, restores ground tiles and the bird.
    pub fn reset(&self) -> &Self {
        while self.elements.sot::<Pipe<S>>().is_some() {}
        while self.elements.sot::<Bushing<S>>().is_some() {}
        while self.elements.sot::<Scenery<S>>().is_some() {}
        self.add_ground();
        self.distance.set(0);
        self.velocity.set(0.0);
        self.dying.set(false);
        self.crashed.set(false);
        self.bird_y.set(self.options.physics.start_y);
        self.place_bird(self.options.physics.start_y);
        self
    }

    /// Lays the dead bird where the flight ended, on top of the scenery.
    /// Rests 3 rows above the crash row, on the pavement.
    fn lay_dead(&self, y: f32) {
        let bx = match self.elements.cot::<FlyingBird<S>>().first() {
            Some(flying) => flying.get_x(),
            None => return,
        };
        if let Some((_, dead)) = self.elements.sot::<DeadBird<S>>() {
            dead.x(bx).y(self.get_y() + y.floor() as isize);
            self.elements.inner.borrow_mut().push(dead);
        }
    }

    /// Upward push. Dead birds don't flap.
    pub fn flap(&self) -> &Self {
        if !self.dying.get() {
            self.velocity.set(-self.options.physics.flap);
        }
        self
    }

    /// True once when the bird has hit the ground.
    pub fn check_crash(&self) -> bool {
        if self.crashed.get() {
            self.crashed.set(false);
            true
        } else {
            false
        }
    }

    /// Gravity pull for one step.
    fn fall(&self) {
        let phys = &self.options.physics;
        let v = (self.velocity.get() + phys.gravity).min(phys.max_fall);
        self.velocity.set(v);
        let y = self.bird_y.get() + v;
        self.bird_y.set(y);
        if self.dying.get() {
            self.place_dead(y);
            if self.dead_hits_ground() {
                self.bird_y.set(DEAD_REST_Y);
                self.place_dead(DEAD_REST_Y);
                self.crashed.set(true);
            }
        } else {
            self.place_bird(y);
            if self.flying_hits_obstacle() {
                self.start_dying(y);
                if self.dead_hits_ground() {
                    self.crashed.set(true);
                }
            }
        }
    }

    /// Hit: swap to the dead bird, boosting stops, it keeps dropping.
    fn start_dying(&self, y: f32) {
        self.dying.set(true);
        for bird in self.elements.cot::<FlyingBird<S>>() {
            bird.showed(false);
        }
        self.lay_dead(y);
        for dead in self.elements.cot::<DeadBird<S>>() {
            dead.showed(true);
        }
    }

    /// Positions the flying bird at the given game-relative y.
    fn place_bird(&self, y: f32) {
        for bird in self.elements.cot::<FlyingBird<S>>() {
            bird.y(self.get_y() + y.floor() as isize);
        }
    }

    /// Positions the dead bird at the given game-relative y.
    fn place_dead(&self, y: f32) {
        for dead in self.elements.cot::<DeadBird<S>>() {
            dead.y(self.get_y() + y.floor() as isize);
        }
    }

    /// True when the flying bird touches any pipe, bushing or floor.
    fn flying_hits_obstacle(&self) -> bool {
        let birds = self.elements.cot::<FlyingBird<S>>();
        let Some(bird) = birds.first() else {
            return false;
        };
        let bird = bird.as_ref();
        for pipe in self.elements.dcot_w::<Pipe<S>, _>(|_| true) {
            if bird.intersects_element(pipe.as_ref()) {
                return true;
            }
        }
        for bushing in self.elements.dcot_w::<Bushing<S>, _>(|_| true) {
            if bird.intersects_element(bushing.as_ref()) {
                return true;
            }
        }
        for floor in self.elements.dcot_w::<Floor<S>, _>(|_| true) {
            if bird.intersects_element(floor.as_ref()) {
                return true;
            }
        }
        false
    }

    /// True when the falling dead bird touches pavement or floor.
    fn dead_hits_ground(&self) -> bool {
        let deads = self.elements.cot::<DeadBird<S>>();
        let Some(dead) = deads.first() else {
            return false;
        };
        let dead = dead.as_ref();
        for pavement in self.elements.dcot_w::<Pavement<S>, _>(|_| true) {
            if dead.intersects_element(pavement.as_ref()) {
                return true;
            }
        }
        for floor in self.elements.dcot_w::<Floor<S>, _>(|_| true) {
            if dead.intersects_element(floor.as_ref()) {
                return true;
            }
        }
        false
    }

    /// Three 40-wide tiles across and past the window.
    fn add_ground(&self) {
        for x in [0, SCENERY_WIDTH as isize, SCENERY_WIDTH as isize * 2] {
            let tile = Scenery::<S>::default();
            tile.x(x).y(GAME_GROUND_Y);
            self.add(tile);
        }
    }

    /// Places a top + bottom pipe duo at the given x. Top pipe starts
    /// at row 0, bottom pipe ends at row 20, 11-row gap between bushings.
    fn spawn_obstacle_at(&self, x: isize) {
        use super::pipe::PipeOptions;

        let top_height = rng().random_range(2..=7) as isize;
        let bottom_height = GAME_GROUND_TOP_ROW - (top_height + 1 + GAME_GAP_ROWS + 1) + 1;

        let top_pipe = Pipe::<S>::new(PipeOptions {
            height: top_height as usize,
            ..Default::default()
        });
        top_pipe.x(x).y(0);
        self.add(top_pipe);

        let top_bushing = Bushing::<S>::default();
        top_bushing.x(x - 1).y(top_height);
        self.add(top_bushing);

        let bottom_bushing = Bushing::<S>::default();
        bottom_bushing
            .x(x - 1)
            .y(top_height + 1 + GAME_GAP_ROWS);
        self.add(bottom_bushing);

        let bottom_pipe = Pipe::<S>::new(PipeOptions {
            height: bottom_height as usize,
            ..Default::default()
        });
        bottom_pipe
            .x(x)
            .y(top_height + 1 + GAME_GAP_ROWS + 1);
        self.add(bottom_pipe);
    }

    /// Moves the world one cell left, wraps ground, spawns and
    /// removes pipes. Title, hint and birds stay put.
    pub fn step(&self) {
        self.fall();
        if self.dying.get() {
            return;
        }
        for tile in self.elements.cot::<Scenery<S>>() {
            tile.x(tile.get_x() - 1);
        }
        for pipe in self.elements.cot::<Pipe<S>>() {
            pipe.x(pipe.get_x() - 1);
        }
        for bushing in self.elements.cot::<Bushing<S>>() {
            bushing.x(bushing.get_x() - 1);
        }

        // Leftmost at -40 wraps past the right edge, relative to self.
        for tile in self.elements.cot::<Scenery<S>>() {
            if tile.get_x() <= self.get_x() - SCENERY_WIDTH as isize {
                tile.x(tile.get_x() + SCENERY_WIDTH as isize * 3);
            }
        }

        // Pipe (6) + gap (24) cadence.
        let distance = self.distance.get() + 1;
        if distance >= self.options.spawn_gap {
            self.spawn_obstacle_at(GAME_SPAWN_X);
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
