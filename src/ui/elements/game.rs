use std::cell::Cell;
use std::rc::Rc;

use incredible::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_macros_decl::element;
use rand::Rng;
use rand::rng;

use super::bushing::{Bushing, BUSHING_WIDTH};
use super::dead_bird::DeadBird;
use super::dead_cat::DeadCat;
use super::floor::Floor;
use super::flower::Flower;
use super::flying_bird::FlyingBird;
use super::flying_cat::FlyingCat;
use super::pavement::Pavement;
use super::pipe::{PIPE_WIDTH, Pipe};
use super::scenery::{SCENERY_WIDTH, Scenery};
use super::score::Score;

pub const GAME_WIDTH: usize = 80;
pub const GAME_HEIGHT: usize = 24;
pub const GAME_GROUND_Y: isize = 16;
pub const GAME_SPAWN_X: isize = 80;
pub const GAME_GAP_ROWS: isize = 11;
pub const GAME_GROUND_TOP_ROW: isize = 20;
pub const DEAD_REST_Y: f32 = 17.0;
pub const BIRD_X: isize = 20;

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub background: u8,
    pub interval_ms: u128,
    pub spawn_gap: usize,
    pub physics: BirdPhysics,
    pub flowers: bool,
    pub base: u8,
    pub handle: String,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            background: crate::ui::theme::SKY_BACKGROUND,
            interval_ms: 100,
            spawn_gap: 30,
            physics: BirdPhysics::default(),
            flowers: false,
            base: crate::ui::theme::PIPE_BACKGROUND,
            handle: "game".to_string(),
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
            flap: 2.5,
            max_fall: 3.0,
            start_y: 10.0,
        }
    }
}

element! {
  pub struct Game<S> {
      options: GameOptions = GameOptions::default(),
      running: Cell<bool> = Cell::new(false),
      spawning: Cell<bool> = Cell::new(false),
      base: Cell<u8> = Cell::new(34),
      kitty: Cell<bool> = Cell::new(false),
      distance: Cell<usize> = Cell::new(0),
      bird_y: Cell<f32> = Cell::new(10.0),
      velocity: Cell<f32> = Cell::new(0.0),
      dying: Cell<bool> = Cell::new(false),
      crashed: Cell<bool> = Cell::new(false),
  }
}

fn ready_effects<S: Clone + PartialEq + 'static>(el: &BlockCharsStr<S>) {
    decorate_rules::<S, BlockCharsStr<S>>(el, ready_effects);
}

impl<S: Clone + PartialEq + 'static> Game<S> {
    pub fn new(options: GameOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        el.base.set(el.options.base);

        el.look(Look::from((GAME_WIDTH, GAME_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle(el.options.handle.clone());

        let flying_bird = FlyingBird::<S>::default();
        flying_bird.x(BIRD_X).y(el.options.physics.start_y as isize);
        el.add(flying_bird);

        let flying_cat = FlyingCat::<S>::default();
        flying_cat.x(BIRD_X).y(el.options.physics.start_y as isize);
        flying_cat.showed(false);
        el.add(flying_cat);

        let score = Score::<S>::default();
        score
            .x((GAME_WIDTH as isize - score.visual.look.width() as isize) / 2)
            .y(1);
        el.add(score);

        let ready = BlockCharsStr::<S>::default();
        ready.text("Ready").size(BlockSize::Small);
        ready.style_handle("ReadyGradient");
        ready.y(6);
        ready.handle("ready_title");
        ready.showed(false);
        effect(&ready, ready_effects);
        el.add(ready);
        el.elements_to_center_x_of_type::<BlockCharsStr<S>>();

        let dead_bird = DeadBird::<S>::default();
        dead_bird.x(48).y(6);
        dead_bird.showed(false);

        let dead_cat = DeadCat::<S>::default();
        dead_cat.x(48).y(6);
        dead_cat.showed(false);

        el.bird_y.set(el.options.physics.start_y);

        // Always three ground tiles.
        el.add_ground();

        // After the ground so they paint on top of it.
        el.add(dead_bird);
        el.add(dead_cat);

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

    /// Starts (or stops) scrolling.
    pub fn set_running(&self, running: bool) -> &Self {
        self.running.set(running);
        self
    }

    /// Mirrors the kitty toggle from app state.
    pub fn set_kitty(&self, kitty: bool) -> &Self {
        self.kitty.set(kitty);
        self
    }

    /// Switches the palette base. Takes effect on next reset/spawn.
    pub fn set_base(&self, base: u8) -> &Self {
        self.base.set(base);
        self
    }

    /// Starts (or stops) pipe spawning. Starting spawns the first duo.
    pub fn set_spawning(&self, spawning: bool) -> &Self {
        let was = self.spawning.get();
        self.spawning.set(spawning);
        if spawning && !was {
            self.spawn_obstacle_at(GAME_SPAWN_X);
            self.distance.set(0);
        }
        self
    }

    /// Clears pipes, restores ground tiles, bird and score.
    pub fn reset(&self) -> &Self {
        while self.elements.sot::<Pipe<S>>().is_some() {}
        while self.elements.sot::<Bushing<S>>().is_some() {}
        while self.elements.sot::<Flower<S>>().is_some() {}
        while self.elements.sot::<Scenery<S>>().is_some() {}
        self.add_ground();
        self.distance.set(0);
        self.velocity.set(0.0);
        self.dying.set(false);
        self.crashed.set(false);
        self.spawning.set(false);
        self.bird_y.set(self.options.physics.start_y);
        self.place_bird(self.options.physics.start_y);
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.show_ready();
        }
        for score in self.elements.cot::<Score<S>>() {
            score.set_value(0);
            self.center_score(score.as_ref());
        }
        self
    }

    /// Bumps the score and keeps it centered.
    fn add_score(&self, points: u32) {
        for score in self.elements.cot::<Score<S>>() {
            score.set_value(score.get_value() + points);
            self.center_score(score.as_ref());
        }
    }

    /// Centers the score display across the window.
    fn center_score(&self, score: &Score<S>) {
        score.x(self.get_x() + (GAME_WIDTH as isize - score.visual.look.width() as isize) / 2);
    }

    /// Lays the dead bird where the flight ended, on top of the scenery.
    /// Rests 3 rows above the crash row, on the pavement.
    fn lay_dead(&self, y: f32) {
        let bx = match self.active_flier() {
            Some(flier) => flier.get_x(),
            None => return,
        };
        if let Some((_, dead)) = self.elements.sot::<DeadBird<S>>() {
            dead.x(bx).y(self.get_y() + y.floor() as isize);
            self.elements.inner.borrow_mut().push(dead);
        }
        if let Some((_, dead)) = self.elements.sot::<DeadCat<S>>() {
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
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.set_rising(v < 0.0);
        }
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
                    self.bird_y.set(DEAD_REST_Y);
                    self.place_dead(DEAD_REST_Y);
                    self.crashed.set(true);
                }
            }
        }
    }

    /// Hit: swap to the matching dead creature, boosting stops.
    fn start_dying(&self, y: f32) {
        self.dying.set(true);
        for bird in self.elements.cot::<FlyingBird<S>>() {
            bird.showed(false);
        }
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.showed(false);
        }
        self.lay_dead(y);
        if self.kitty.get() {
            for dead in self.elements.cot::<DeadCat<S>>() {
                dead.showed(true);
            }
        } else {
            for dead in self.elements.cot::<DeadBird<S>>() {
                dead.showed(true);
            }
        }
    }

    /// Positions the flying bird and cat at the given game-relative y.
    fn place_bird(&self, y: f32) {
        for bird in self.elements.cot::<FlyingBird<S>>() {
            bird.y(self.get_y() + y.floor() as isize);
        }
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.y(self.get_y() + y.floor() as isize);
        }
    }

    /// Positions the dead bird and cat at the given game-relative y.
    fn place_dead(&self, y: f32) {
        for dead in self.elements.cot::<DeadBird<S>>() {
            dead.y(self.get_y() + y.floor() as isize);
        }
        for dead in self.elements.cot::<DeadCat<S>>() {
            dead.y(self.get_y() + y.floor() as isize);
        }
    }

    /// The currently flown creature, bird or cat.
    fn active_flier(&self) -> Option<Rc<dyn ElementTrait<S>>> {
        if self.kitty.get() {
            self.elements
                .cot::<FlyingCat<S>>()
                .first()
                .cloned()
                .map(|el| el as Rc<dyn ElementTrait<S>>)
        } else {
            self.elements
                .cot::<FlyingBird<S>>()
                .first()
                .cloned()
                .map(|el| el as Rc<dyn ElementTrait<S>>)
        }
    }

    /// The currently dying creature, bird or cat.
    fn active_dead(&self) -> Option<Rc<dyn ElementTrait<S>>> {
        if self.kitty.get() {
            self.elements
                .cot::<DeadCat<S>>()
                .first()
                .cloned()
                .map(|el| el as Rc<dyn ElementTrait<S>>)
        } else {
            self.elements
                .cot::<DeadBird<S>>()
                .first()
                .cloned()
                .map(|el| el as Rc<dyn ElementTrait<S>>)
        }
    }

    /// True when the flown creature touches any pipe, bushing or floor.
    fn flying_hits_obstacle(&self) -> bool {
        let Some(bird) = self.active_flier() else {
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
        for flower in self.elements.dcot_w::<Flower<S>, _>(|_| true) {
            if bird.intersects_element(flower.as_ref()) {
                return true;
            }
        }
        for pavement in self.elements.dcot_w::<Pavement<S>, _>(|_| true) {
            if bird.intersects_element(pavement.as_ref()) {
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

    /// True when the falling dead creature touches pavement or floor.
    fn dead_hits_ground(&self) -> bool {
        let Some(dead) = self.active_dead() else {
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
        use super::scenery::SceneryOptions;

        for x in [0, SCENERY_WIDTH as isize, SCENERY_WIDTH as isize * 2] {
            let tile = Scenery::<S>::new(SceneryOptions {
                pavement_background: self.base.get(),
            });
            tile.x(x).y(GAME_GROUND_Y);
            self.add(tile);
        }
    }

    /// Places a ground flower at the given x, bottom row on 20.
    fn spawn_flower_at(&self, x: isize) {
        let flower = Flower::<S>::default();
        let h = flower.visual.look.height() as isize;
        flower.x(x).y(self.get_y() + 21 - h);
        self.add(flower);
        self.send_scenery_to_back();
    }

    /// Places a top + bottom pipe duo at the given x. Top pipe starts
    /// at row 0, bottom pipe ends at row 20, 11-row gap between bushings.
    /// With flowers on, flips a coin for a ground flower instead.
    fn spawn_obstacle_at(&self, x: isize) {
        if self.options.flowers && rng().random_bool(0.5) {
            self.spawn_flower_at(x);
            return;
        }
        use super::bushing::BushingOptions;
        use super::pipe::PipeOptions;

        let top_height = rng().random_range(2..=7) as isize;
        let bottom_height = GAME_GROUND_TOP_ROW - (top_height + 1 + GAME_GAP_ROWS + 1) + 1;

        let top_pipe = Pipe::<S>::new(PipeOptions {
            background: self.base.get(),
            height: top_height as usize,
        });
        top_pipe.x(x).y(0);
        self.add(top_pipe);

        let top_bushing = Bushing::<S>::new(BushingOptions {
            background: self.base.get(),
        });
        top_bushing.x(x - 1).y(top_height);
        self.add(top_bushing);

        let bottom_bushing = Bushing::<S>::new(BushingOptions {
            background: self.base.get(),
        });
        bottom_bushing
            .x(x - 1)
            .y(top_height + 1 + GAME_GAP_ROWS);
        self.add(bottom_bushing);

        let bottom_pipe = Pipe::<S>::new(PipeOptions {
            background: self.base.get(),
            height: bottom_height as usize,
        });
        bottom_pipe
            .x(x)
            .y(top_height + 1 + GAME_GAP_ROWS + 1);
        self.add(bottom_pipe);

        self.send_scenery_to_back();
    }

    /// Sends all ground tiles behind everything else so the bird
    /// always stays in front of the scenery.
    fn send_scenery_to_back(&self) {
        let tiles: Vec<Rc<Scenery<S>>> = self.elements.cot::<Scenery<S>>();
        for tile in tiles.iter().rev() {
            let ptr = Rc::as_ptr(tile);
            self.to_back_of_type_where::<Scenery<S>, _>(|t| {
                std::ptr::eq(t as *const _, ptr)
            });
        }
    }

    /// Moves the world one cell left, wraps ground, spawns and
    /// removes pipes. Title, hint and birds stay put.
    pub fn step(&self) {
        if self.spawning.get() || self.dying.get() {
            self.fall();
        }
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
        if self.spawning.get() {
            let distance = self.distance.get() + 1;
            if distance >= self.options.spawn_gap {
                self.spawn_obstacle_at(GAME_SPAWN_X);
                self.distance.set(0);
            } else {
                self.distance.set(distance);
            }
        }

        // Gap passed: top pipe right edge reaches the bird.
        for pipe in self.elements.cot::<Pipe<S>>() {
            if pipe.get_y() == self.get_y()
                && pipe.get_x() + PIPE_WIDTH as isize == self.get_x() + BIRD_X
            {
                self.add_score(1);
            }
        }

        // Flower passed: right edge reaches the bird.
        for flower in self.elements.cot::<Flower<S>>() {
            if flower.get_x() + flower.visual.look.width() as isize == self.get_x() + BIRD_X
            {
                self.add_score(1);
            }
        }

        // Drop fully off-screen pipes.
        while self
            .elements
            .sot_w::<Pipe<S>, _>(|p| {
                p.get_x() + PIPE_WIDTH as isize <= self.get_x()
            })
            .is_some()
        {}
        while self
            .elements
            .sot_w::<Bushing<S>, _>(|b| {
                b.get_x() + BUSHING_WIDTH as isize <= self.get_x()
            })
            .is_some()
        {}
    }
}

impl<S: Clone + PartialEq> Default for Game<S> {
    fn default() -> Self {
        Self::new(GameOptions::default())
    }
}
