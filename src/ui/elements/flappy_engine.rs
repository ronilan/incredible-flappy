use std::cell::Cell;
use std::rc::Rc;

use incredible::*;
use incredible_elements::{Button, Image};
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;
use crate::ui::elements::{BoxedTextFont, BoxedTextFontOptions};
use incredible_macros_decl::element;
use rand::Rng;
use rand::rng;

use crate::ui::assets::decode_png;

use crate::ui::elements::pipe::{PIPE_WIDTH, Pipe, PipeOptions};
use crate::ui::elements::{
    ALIEN_MAX_Y, ALIEN_MIN_Y, Alien, AlienKind, AlienOptions, BULLET_SPEED, BUSHING_WIDTH, Bullet,
    Bushing, BushingOptions, DeadBird,
    DeadCat, Floor, FLOWER_BUD_HEIGHT, FlowerBud, FlowerStem, FlowerStemOptions,
    FlyingBird, FlyingCat, ImageButton, ImageButtonKind, ImageButtonOptions, PAVEMENT_WIDTH, Pavement, PavementOptions, Scenery, SceneryOptions, Score, U16Image,
    SCORE_PANEL_PARK_Y, SCORE_PANEL_WIDTH, SCORE_PANEL_REST_Y, ScorePanel, Sparkle, Stoplight,
};

pub const GAME_WIDTH: usize = 80;
pub const GAME_HEIGHT: usize = 24;
pub const GAME_GROUND_Y: isize = 16;
pub const GAME_SPAWN_X: isize = 80;
pub const GAME_GAP_ROWS: isize = 11;
pub const GAME_GROUND_TOP_ROW: isize = 20;
pub const DEAD_REST_Y: f32 = 17.0;
pub const BIRD_X: isize = 21;
/// Kitty bump leeway: forgiven rows on the top and bottom of the cat.
/// Note: the cat image sits in a bigger rectangle than the bird,
/// so the leeway keeps a "leveled playing field" between them.
const KITTY_LEEWAY_TOP: isize = 0;
const KITTY_LEEWAY_BOTTOM: isize = 1;
const KITTY_LEEWAY_FRONT: isize = 0;
const KITTY_LEEWAY_BACK: isize = 4;
/// Pavement top row in game coordinates: the lethal ground surface.
const GROUND_SURFACE: f32 = 21.0;

/// True when a fall from prev_top to new_top of the given height sweeps
/// past the ground surface. Discrete overlap misses fast falls that hop
/// clean over the thin ground band in one step.
fn swept_ground(prev_top: f32, height: f32, new_top: f32) -> bool {
    new_top >= prev_top
        && prev_top + height <= GROUND_SURFACE
        && new_top + height >= GROUND_SURFACE
}

#[derive(Clone, Debug)]
pub struct GameOptions {
    pub interval_ms: u128,
    pub spawn_gap: usize,
    pub physics: BirdPhysics,
    pub kind: crate::ui::state::SelectedGame,
    pub handle: String,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            interval_ms: 100,
            spawn_gap: 30,
            physics: BirdPhysics::default(),
            kind: crate::ui::state::SelectedGame::Classic,
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
            gravity: 1.0,
            flap: 4.0,
            max_fall: 6.0,
            start_y: 10.0,
        }
    }
}

element! {
  pub struct Game<S> {
      options: GameOptions = GameOptions::default(),
      internal_state: GameState = GameState::default(),
  }
}

// -----------------------------
// GameState: live run data.
// -----------------------------
#[derive(Clone, Debug)]
pub struct GameState {
    pub running: Cell<bool>,
    pub spawning: Cell<bool>,
    pub kind: Cell<crate::ui::state::SelectedGame>,
    pub kitty: Cell<bool>,
    pub distance: Cell<usize>,
    pub bird_y: Cell<f32>,
    pub velocity: Cell<f32>,
    pub dying: Cell<bool>,
    pub crashed: Cell<bool>,
    pub landed: Cell<bool>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: Cell::new(false),
            spawning: Cell::new(false),
            kind: Cell::new(crate::ui::state::SelectedGame::Classic),
            kitty: Cell::new(false),
            distance: Cell::new(0),
            bird_y: Cell::new(10.0),
            velocity: Cell::new(0.0),
            dying: Cell::new(false),
            crashed: Cell::new(false),
            landed: Cell::new(false),
        }
    }
}

impl<S: Clone + PartialEq + 'static> Game<S> {
    pub fn new(options: GameOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        el.internal_state.kind.set(el.options.kind);

        el.look(Look::from((GAME_WIDTH, GAME_HEIGHT, ' ')))
            .background(Some(Color::Ansi(crate::ui::state::palette_for(&el.options.kind).sky)))
            .handle(el.options.handle.clone());

        let flying_bird = FlyingBird::<S>::default();
        flying_bird.x(BIRD_X).y(el.options.physics.start_y as isize);
        el.add(flying_bird);

        let flying_cat = FlyingCat::<S>::default();
        flying_cat.x(BIRD_X).y(el.options.physics.start_y as isize);
        flying_cat.showed(false);
        el.add(flying_cat);

        // Pause on top, resume twin for kitty. Clicks handled in app.
        let pause = Button::<S>::new();
        pause.text("II");
        pause.style_handle("GameButton");
        pause.pointer(Some(PointerShape::Pointer));
        pause.handle("pause_button");
        pause.showed(false);
        pause.x(1).y(1);
        el.add(pause);

        let pause_img = ImageButton::<S>::new(ImageButtonOptions {
            kind: ImageButtonKind::Pause,
            ..Default::default()
        });
        pause_img.handle("pause_image_button");
        pause_img.showed(false);
        pause_img.pointer(Some(PointerShape::Pointer));
        pause_img.x(1).y(1);
        el.add(pause_img);

        let resume_img = ImageButton::<S>::new(ImageButtonOptions {
            kind: ImageButtonKind::Play,
            ..Default::default()
        });
        resume_img.handle("resume_image_button");
        resume_img.showed(false);
        resume_img.pointer(Some(PointerShape::Pointer));
        resume_img.x(1).y(1);
        el.add(resume_img);

        // Back and play, bottom row. Clicks handled in app.
        let back = Button::<S>::new();
        back.text("←");
        back.style_handle("GameButton");
        back.pointer(Some(PointerShape::Pointer));
        back.handle("back_button");
        let back_w = back.visual.look.width() as isize;

        let play = Button::<S>::new();
        play.text("PLAY");
        play.style_handle("GameButton");
        play.pointer(Some(PointerShape::Pointer));
        play.handle("play_button");
        play.focused(true);
        let play_w = play.visual.look.width() as isize;

        let row_x = (GAME_WIDTH as isize - (back_w + play_w + 1)) / 2;
        back.x(row_x).y(14);
        el.add(back);
        play.x(row_x + back_w + 1).y(14);
        el.add(play);

        let back_img = ImageButton::<S>::new(ImageButtonOptions {
            kind: ImageButtonKind::Back,
            ..Default::default()
        });
        back_img.handle("back_image_button");
        back_img.showed(false);
        back_img.pointer(Some(PointerShape::Pointer));
        let back_img_w = back_img.visual.look.width() as isize;

        let play_img = ImageButton::<S>::new(ImageButtonOptions {
            kind: ImageButtonKind::Play,
            ..Default::default()
        });
        play_img.handle("play_image_button");
        play_img.showed(false);
        play_img.pointer(Some(PointerShape::Pointer));
        play_img.focused(true);
        let play_img_w = play_img.visual.look.width() as isize;

        let img_x = (GAME_WIDTH as isize - (back_img_w + play_img_w + 1)) / 2;
        back_img.x(img_x).y(14);
        el.add(back_img);
        play_img.x(img_x + back_img_w + 1).y(14);
        el.add(play_img);

        let score = Score::<S>::default();
        score
            .x((GAME_WIDTH as isize - score.visual.look.width() as isize) / 2)
            .y(1);
        el.add(score);

        // Countdown stoplight, center screen. Shown in Ready only.
        let stoplight = Stoplight::<S>::default();
        stoplight.set_lit(1);
        stoplight.handle("stoplight");
        stoplight.showed(false);
        stoplight
            .x((GAME_WIDTH as isize - stoplight.visual.look.width() as isize) / 2)
            .y((GAME_HEIGHT as isize - stoplight.visual.look.height() as isize) / 2);
        el.add(stoplight);

        // Kitty-mode image score in the same slot.
        let image_score = U16Image::<S>::default();
        image_score
            .x((GAME_WIDTH as isize - image_score.visual.look.width() as isize) / 2)
            .y(1)
            .handle("image_score")
            .showed(false);
        el.add(image_score);

        // Game-over score panel in the middle of the screen.
        let panel = ScorePanel::<S>::new();
        panel
            .x((GAME_WIDTH as isize - SCORE_PANEL_WIDTH as isize) / 2)
            .y(SCORE_PANEL_REST_Y)
            .handle("score_panel")
            .showed(false);
        el.add(panel);

        let ready = BoxedTextFont::<S>::new(BoxedTextFontOptions {
            background: crate::ui::theme::GET_READY_BACKGROUND,
            foreground: crate::ui::theme::TITLE_TEXT,
        });
        ready.text("Get Ready");
        ready
            .x((GAME_WIDTH as isize - ready.visual.look.width() as isize) / 2)
            .y(1)
            .handle("ready_title")
            .showed(false)
            .focused(false);
        let ready_w = ready.visual.look.width();
        el.add(ready);

        let over = BoxedTextFont::<S>::new(BoxedTextFontOptions {
            background: crate::ui::theme::TITLE_BACKGROUND,
            foreground: crate::ui::theme::TITLE_TEXT,
        });
        over.text("Game Over");
        over
            .x((GAME_WIDTH as isize - over.visual.look.width() as isize) / 2)
            .y(1)
            .handle("gameover_title")
            .showed(false)
            .focused(false);
        let over_w = over.visual.look.width();
        el.add(over);

        // Kitty-mode images replacing the text titles, title height
        // with width derived from the image aspect ratio.
        let ready_img = Image::<S>::new();
        ready_img
            .width(ready_w)
            .data(decode_png(include_bytes!("../../../assets/get_ready.png")))
            .handle("ready_image")
            .showed(false);
        ready_img
            .x((GAME_WIDTH as isize - ready_img.visual.look.width() as isize) / 2)
            .y(2);
        el.add(ready_img);

        let over_img = Image::<S>::new();
        over_img
            .width(over_w + 2)
            .data(decode_png(include_bytes!("../../../assets/game_over.png")))
            .handle("gameover_image")
            .showed(false);
        over_img
            .x((GAME_WIDTH as isize - over_img.visual.look.width() as isize) / 2)
            .y(2);
        el.add(over_img);

        let dead_bird = DeadBird::<S>::default();
        dead_bird.x(48).y(6);
        dead_bird.showed(false);

        let dead_cat = DeadCat::<S>::default();
        dead_cat.x(48).y(6);
        dead_cat.showed(false);

        el.internal_state.bird_y.set(el.options.physics.start_y);

        // Always three ground tiles.
        el.add_ground();

        // After the ground so they paint on top of it.
        el.add(dead_bird);
        el.add(dead_cat);

        // Marquee method: step on animation progress each loop tick.
        el.internal_on_loop(|el, _, _event| {
            if !el.internal_state.running.get() {
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
    fn refresh(&self) -> &Self {
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
        self.internal_state.running.set(running);
        self
    }

    /// Mirrors the kitty toggle from app state.
    pub fn set_kitty(&self, kitty: bool) -> &Self {
        self.internal_state.kitty.set(kitty);
        for panel in self.elements.cot::<ScorePanel<S>>() {
            panel.background(if kitty {
                None
            } else {
                Some(crate::ui::theme::score_panel_background())
            });
        }
        self
    }

    /// Switches the game kind. Sky applies now, rest on next reset/spawn.
    pub fn set_kind(&self, kind: crate::ui::state::SelectedGame) -> &Self {
        self.internal_state.kind.set(kind);
        self.background(Some(Color::Ansi(crate::ui::state::palette_for(&kind).sky)));
        for light in self.elements.cot::<Stoplight<S>>() {
            light.set_lit_color(crate::ui::state::palette_for(&kind).pipe_base);
        }
        self
    }

    /// Starts (or stops) pipe spawning. Starting spawns the first duo.
    /// Starts (or stops) pipe spawning. Starting spawns the first duo,
    /// but only into an empty sky: resuming mid-run must not inject one.
    pub fn set_spawning(&self, spawning: bool) -> &Self {
        let was = self.internal_state.spawning.get();
        self.internal_state.spawning.set(spawning);
        let fresh_sky = self.elements.cot::<Pipe<S>>().is_empty()
            && self.elements.cot::<FlowerBud<S>>().is_empty()
            && self.elements.cot::<Alien<S>>().is_empty();
        if spawning && !was && fresh_sky {
            self.spawn_obstacle_at(GAME_SPAWN_X);
            self.internal_state.distance.set(0);
        }
        self
    }

    /// Clears pipes, restores ground tiles, bird and score.
    pub fn reset(&self) -> &Self {
        while self.elements.sot::<Pipe<S>>().is_some() {}
        while self.elements.sot::<Bushing<S>>().is_some() {}
        while self.elements.sot::<FlowerStem<S>>().is_some() {}
        while self.elements.sot::<FlowerBud<S>>().is_some() {}
        while self.elements.sot::<Alien<S>>().is_some() {}
        while self.elements.sot::<Bullet<S>>().is_some() {}
        while self.elements.sot::<Sparkle<S>>().is_some() {}
        while self.elements.sot::<Pavement<S>>().is_some() {}
        while self.elements.sot::<Scenery<S>>().is_some() {}
        self.add_ground();
        self.internal_state.distance.set(0);
        self.internal_state.velocity.set(0.0);
        self.internal_state.dying.set(false);
        self.internal_state.crashed.set(false);
        self.internal_state.landed.set(false);
        self.internal_state.spawning.set(false);
        self.internal_state.bird_y.set(self.options.physics.start_y);
        self.place_bird(self.options.physics.start_y);
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.show_ready();
        }
        for score in self.elements.cot::<Score<S>>() {
            score.set_value(0);
            self.center_score(score.as_ref());
        }
        for image in self.elements.cot::<U16Image<S>>() {
            image.value(0);
            self.center_image_score(image.as_ref());
        }
        self
    }

    /// Bumps the score and keeps it centered.
    fn add_score(&self, points: u32) {
        for score in self.elements.cot::<Score<S>>() {
            score.set_value(score.get_value() + points);
            self.center_score(score.as_ref());
        }
        for image in self.elements.cot::<U16Image<S>>() {
            image.value(image.get_value() + points as u16);
            self.center_image_score(image.as_ref());
        }
    }

    /// Current score value.
    pub fn score_value(&self) -> u32 {
        self.elements
            .cot::<Score<S>>()
            .first()
            .map(|score| score.get_value())
            .unwrap_or(0)
    }

    /// Centers the score display across the window.
    fn center_score(&self, score: &Score<S>) {
        score.x(self.get_x() + (GAME_WIDTH as isize - score.visual.look.width() as isize) / 2);
    }

    /// Centers the image score display across the window.
    fn center_image_score(&self, image: &U16Image<S>) {
        image.x(self.get_x() + (GAME_WIDTH as isize - image.visual.look.width() as isize) / 2);
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

    /// Upward push. Dead birds don't flap. Take off when perched.
    pub fn flap(&self) -> &Self {
        if !self.internal_state.dying.get() {
            self.internal_state.landed.set(false);
            self.internal_state.velocity.set(-self.options.physics.flap);
        }
        self
    }

    /// Fires a bullet from the beak. Invaders only, never while dying.
    pub fn shoot(&self) -> &Self {
        if !matches!(
            self.internal_state.kind.get(),
            crate::ui::state::SelectedGame::Invaders
        ) || self.internal_state.dying.get()
        {
            return self;
        }
        let Some(flier) = self.active_flier() else {
            return self;
        };
        let w = flier.visual().look.width() as isize;
        let h = flier.visual().look.height() as isize;
        let bullet = Bullet::<S>::default();
        bullet
            .x(BIRD_X + w)
            .y(self.internal_state.bird_y.get() as isize + h / 2);
        self.add(bullet);
        self
    }

    /// Re-skins all aliens for the kitty flag: logos in kitty,
    /// marchers out. Keeps positions, roam and bob direction.
    pub fn reskin_aliens(&self) -> &Self {
        let kitty = self.internal_state.kitty.get();
        for alien in self.elements.cot::<Alien<S>>() {
            let kind = if kitty {
                AlienKind::random_logo()
            } else {
                AlienKind::random()
            };
            let (x, y) = (
                alien.get_x() - self.get_x(),
                alien.get_y() - self.get_y(),
            );
            let (roam, dy) = (alien.options.roam, alien.internal_state.dy.get());
            let ptr = Rc::as_ptr(&alien);
            while self
                .elements
                .sot_w::<Alien<S>, _>(|a| std::ptr::eq(a as *const _, ptr))
                .is_some()
            {}
            let fresh = Alien::<S>::new(AlienOptions { kind, roam });
            fresh.internal_state.dy.set(dy);
            fresh.x(x).y(y);
            self.add(fresh);
        }
        self.send_scenery_to_back();
        self
    }

    /// Removes all bullets, e.g. so none freeze over the game-over panel.
    pub fn clear_bullets(&self) -> &Self {
        while self.elements.sot::<Bullet<S>>().is_some() {}
        self
    }

    /// Stills the bird: zero velocity, e.g. on pause.
    pub fn still(&self) -> &Self {
        self.internal_state.velocity.set(0.0);
        self
    }

    /// Parks the score panel below the screen for the slide-in.
    pub fn park_panel(&self) -> &Self {
        let y = self.get_y() + SCORE_PANEL_PARK_Y;
        for panel in self.elements.cot::<ScorePanel<S>>() {
            panel.y(y);
        }
        self
    }

    /// Slides the score panel one row toward rest. True while moving.
    pub fn slide_panel(&self) -> bool {
        let rest = self.get_y() + SCORE_PANEL_REST_Y;
        let mut moving = false;
        for panel in self.elements.cot::<ScorePanel<S>>() {
            if panel.get_y() > rest {
                panel.y(panel.get_y() - 1);
                moving = true;
            }
        }
        moving
    }

    /// True once the score panel slide-in finished.
    pub fn panel_settled(&self) -> bool {
        let rest = self.get_y() + SCORE_PANEL_REST_Y;
        self.elements
            .cot::<ScorePanel<S>>()
            .first()
            .is_some_and(|panel| panel.get_y() <= rest)
    }

    /// Sparkles shimmering off the flier's look box while grazing,
    /// tinted by the bud under it. The bigger kitty box throws them
    /// further out. Positions are game-relative; add() folds in
    /// the game offset.
    fn emit_sparkles(&self) {
        let Some(flier) = self.active_flier() else {
            return;
        };
        let color = self
            .perched_bud()
            .map(|bud| bud.base_color())
            .unwrap_or(crate::ui::theme::SPARKLE_COLOR);
        let fx = flier.get_x() - self.get_x();
        let fy = flier.get_y() - self.get_y();
        let h = flier.visual().look.height() as isize;
        for _ in 0..2 {
            // Exhaust scattered behind the tail; the scroll trails them.
            let dx = -1 - rng().random_range(0..4) as isize;
            let dy = rng().random_range(-1..h as i32 + 1) as isize;
            let sparkle: Rc<dyn ElementTrait<S>> =
                Rc::new(Sparkle::<S>::new(crate::ui::elements::SparkleOptions {
                    color,
                }));
            sparkle.x(fx + dx).y(fy + dy);
            self.add_any(sparkle.clone());
            self.to_back_where(|e| Rc::ptr_eq(e, &sparkle));
        }
    }

    /// True once when the bird has hit the ground.
    pub fn check_crash(&self) -> bool {
        if self.internal_state.crashed.get() {
            self.internal_state.crashed.set(false);
            true
        } else {
            false
        }
    }

    /// Gravity pull for one step.
    fn fall(&self) {
        let phys = &self.options.physics;
        let v = (self.internal_state.velocity.get() + phys.gravity).min(phys.max_fall);
        self.internal_state.velocity.set(v);        for cat in self.elements.cot::<FlyingCat<S>>() {
            if self.internal_state.landed.get() && self.over_perch() {
                cat.show_ready();
            } else {
                cat.set_rising(v < 0.0);
            }
        }
        if self.internal_state.dying.get() {
            let prev = self.internal_state.bird_y.get();
            let y = prev + v;
            self.internal_state.bird_y.set(y);
            self.place_dead(y);
            if self.dead_hits_ground() {
                self.internal_state.bird_y.set(DEAD_REST_Y);
                self.place_dead(DEAD_REST_Y);
                self.internal_state.crashed.set(true);
            } else if v >= 0.0 {
                // Fast falls can hop the thin ground band: snap on sweep.
                if let Some(dead) = self.active_dead() {
                    let dh = dead.visual().look.height() as f32;
                    if swept_ground(prev, dh, y) {
                        self.internal_state.bird_y.set(DEAD_REST_Y);
                        self.place_dead(DEAD_REST_Y);
                        self.internal_state.crashed.set(true);
                    }
                }
            }
        } else if self.internal_state.landed.get() && self.over_perch() {
            // Perched: hold position, no gravity. Pipes can still kill.
            self.internal_state.velocity.set(0.0);
            self.place_bird(self.internal_state.bird_y.get());
            self.emit_sparkles();
            if self.flying_hits_obstacle() {
                self.start_dying(self.internal_state.bird_y.get());
            }
        } else {
            self.internal_state.landed.set(false);
            let prev = self.internal_state.bird_y.get();
            let y = prev + v;
            self.internal_state.bird_y.set(y);
            // Touching down onto a bud or bushing top while falling
            // = land, not kill.
            if v >= 0.0 {
                if let Some(flier) = self.active_flier() {
                    let fh = flier.visual().look.height() as f32;
                    if let Some(top) = self.perch_top_under(prev + fh, y + fh) {
                        self.internal_state.landed.set(true);
                        self.internal_state.velocity.set(0.0);
                        self.internal_state.bird_y.set(top - fh);
                        self.place_bird(top - fh);
                        self.add_score(1);
                        return;
                    }
                    // Fast falls can hop the thin ground band: die on sweep.
                    if swept_ground(prev, fh, y) {
                        let gy = GROUND_SURFACE - fh;
                        self.internal_state.bird_y.set(gy);
                        self.place_bird(gy);
                        self.start_dying(gy);
                        if self.dead_hits_ground() {
                            self.internal_state.bird_y.set(DEAD_REST_Y);
                            self.place_dead(DEAD_REST_Y);
                            self.internal_state.crashed.set(true);
                        }
                        return;
                    }
                }
            }
            self.place_bird(y);
            if y <= 0.0 || self.flying_hits_obstacle() {
                self.start_dying(y);
                if self.dead_hits_ground() {
                    self.internal_state.bird_y.set(DEAD_REST_Y);
                    self.place_dead(DEAD_REST_Y);
                    self.internal_state.crashed.set(true);
                }
            }
        }
    }

    /// True while the flier sits over a bud top.
    fn over_perch(&self) -> bool {
        self.perched_bud().is_some()
    }

    /// The bud top under the flier, if any.
    fn perched_bud(&self) -> Option<Rc<FlowerBud<S>>> {
        let flier = self.active_flier()?;
        let fx0 = flier.get_x();
        let fx1 = fx0 + flier.visual().look.width() as isize;
        self.elements.cot::<FlowerBud<S>>().into_iter().find(|bud| {
            let w = bud.visual.look.width() as isize;
            fx0 < bud.get_x() + w && bud.get_x() < fx1
        })
    }

    /// Bud top crossed falling from prev_bottom to new_bottom, if any.
    /// Touching down = land, not kill.
    fn perch_top_under(&self, prev_bottom: f32, new_bottom: f32) -> Option<f32> {
        let Some(flier) = self.active_flier() else {
            return None;
        };
        let fx0 = flier.get_x();
        let fx1 = fx0 + flier.visual().look.width() as isize;
        for bud in self.elements.cot::<FlowerBud<S>>() {
            let w = bud.visual.look.width() as isize;
            let top = (bud.get_y() - self.get_y()) as f32;
            if prev_bottom <= top
                && top <= new_bottom
                && fx0 < bud.get_x() + w
                && bud.get_x() < fx1
            {
                return Some(top);
            }
        }
        None
    }

    /// Hit: swap to the matching dead creature, boosting stops.
    fn start_dying(&self, y: f32) {
        self.internal_state.dying.set(true);
        for bird in self.elements.cot::<FlyingBird<S>>() {
            bird.showed(false);
        }
        for cat in self.elements.cot::<FlyingCat<S>>() {
            cat.showed(false);
        }
        self.lay_dead(y);
        if self.internal_state.kitty.get() {
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
        if self.internal_state.kitty.get() {
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
        if self.internal_state.kitty.get() {
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

    /// Hit test with kitty bump leeway: one forgiven row on the
    /// top and bottom of the cat. Birds test the full look.
    /// Note: the cat image sits in a bigger rectangle than the bird,
    /// so the leeway keeps a "leveled playing field" between them.
    fn flier_hits(&self, flier: &dyn ElementTrait<S>, other: &dyn ElementTrait<S>) -> bool {
        if self.internal_state.kitty.get() {
            let outer = flier.get_outside();
            let height = outer
                .height
                .get()
                .saturating_sub((KITTY_LEEWAY_TOP + KITTY_LEEWAY_BOTTOM) as usize)
                .max(1);
            let width = outer
                .width
                .get()
                .saturating_sub((KITTY_LEEWAY_FRONT + KITTY_LEEWAY_BACK) as usize)
                .max(1);
            let shrunk = Bounds {
                x: Cell::new(outer.x.get() + KITTY_LEEWAY_BACK),
                y: Cell::new(outer.y.get() + KITTY_LEEWAY_TOP),
                width: Cell::new(width),
                height: Cell::new(height),
            };
            return other.intersects(&shrunk);
        }
        flier.intersects_element(other)
    }

    /// True when the flown creature touches any pipe, bushing or floor.
    fn flying_hits_obstacle(&self) -> bool {
        let Some(bird) = self.active_flier() else {
            return false;
        };
        let bird = bird.as_ref();
        for pipe in self.elements.dcot_w::<Pipe<S>, _>(|_| true) {
            if self.flier_hits(bird, pipe.as_ref()) {
                return true;
            }
        }
        for bushing in self.elements.dcot_w::<Bushing<S>, _>(|_| true) {
            if self.flier_hits(bird, bushing.as_ref()) {
                return true;
            }
        }
        for flower in self.elements.dcot_w::<FlowerStem<S>, _>(|_| true) {
            if self.flier_hits(bird, flower.as_ref()) {
                return true;
            }
        }
        for flower in self.elements.dcot_w::<FlowerBud<S>, _>(|_| true) {
            if self.flier_hits(bird, flower.as_ref()) {
                return true;
            }
        }
        for alien in self.elements.dcot_w::<Alien<S>, _>(|_| true) {
            if self.flier_hits(bird, alien.as_ref()) {
                return true;
            }
        }
        for pavement in self.elements.dcot_w::<Pavement<S>, _>(|_| true) {
            if self.flier_hits(bird, pavement.as_ref()) {
                return true;
            }
        }
        for floor in self.elements.dcot_w::<Floor<S>, _>(|_| true) {
            if self.flier_hits(bird, floor.as_ref()) {
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
    /// Static ground tile plus one scrolling pavement marquee.
    fn add_ground(&self) {
        let tile = Scenery::<S>::new(SceneryOptions {
            floor_background: crate::ui::state::palette_for(&self.internal_state.kind.get()).floor,
            bush_background: crate::ui::state::palette_for(&self.internal_state.kind.get()).bush_bg,
            bush_color: crate::ui::state::palette_for(&self.internal_state.kind.get()).bush_color,
        });
        tile.x(0).y(GAME_GROUND_Y);
        self.add(tile);
        let pavement = Pavement::<S>::new(PavementOptions {
            background: crate::ui::state::palette_for(&self.internal_state.kind.get()).pipe_base,
            width: PAVEMENT_WIDTH,
        });
        pavement.x(0).y(GAME_GROUND_Y + 5);
        self.add(pavement);
        self.send_scenery_to_back();
    }

    /// Single flower on the ground: random stem height with the
    /// specified bud on top, bottom row on 20.
    fn spawn_flower_at(&self, x: isize) {

        let stem_height = rng().random_range(2..=7) as isize;
        let top_y = 21 - (FLOWER_BUD_HEIGHT as isize + stem_height);

        let bud = FlowerBud::<S>::default();
        bud.x(x).y(top_y);
        self.add(bud);

        let stem = FlowerStem::<S>::new(FlowerStemOptions {
            height: stem_height as usize,
        });
        stem.x(x + 1).y(top_y + FLOWER_BUD_HEIGHT as isize);
        self.add(stem);

        self.send_scenery_to_back();
    }

    /// Two aliens side by side, each random kind and height. Roam.
    fn spawn_alien_at(&self, x: isize) {
        let mut ax = x;
        for _ in 0..2 {
            let kind = if self.internal_state.kitty.get() {
                AlienKind::random_logo()
            } else {
                AlienKind::random()
            };
            let alien = Alien::<S>::new(AlienOptions { kind, roam: true });
            alien
                .x(ax)
                .y(rng().random_range(ALIEN_MIN_Y as i32..=ALIEN_MAX_Y as i32) as isize);
            ax += alien.width() as isize + 2;
            self.add(alien);
        }
        self.send_scenery_to_back();
    }

    /// Places a top + bottom pipe duo at the given x. Top pipe starts
    /// at row 0, bottom pipe ends at row 20, 11-row gap between bushings.
    /// Busy games swap the duo for a flower half the time,
    /// invaders games for an alien.
    fn spawn_obstacle_at(&self, x: isize) {
        if matches!(self.internal_state.kind.get(), crate::ui::state::SelectedGame::Busy)
            && rng().random_bool(0.5)
        {
            self.spawn_flower_at(x);
            return;
        }
        if matches!(
            self.internal_state.kind.get(),
            crate::ui::state::SelectedGame::Invaders
        ) && rng().random_bool(0.5)
        {
            self.spawn_alien_at(x);
            return;
        }
        self.spawn_pipe_at(x);
    }

    /// Places a top + bottom pipe duo at the given x. Top pipe starts
    /// at row 0, bottom pipe ends at row 20, 11-row gap between bushings.
    fn spawn_pipe_at(&self, x: isize) {
        let top_height = rng().random_range(2i32..=5) as isize;
        let bottom_height = GAME_GROUND_TOP_ROW - (top_height + 1 + GAME_GAP_ROWS + 1) + 1;

        let top_pipe = Pipe::<S>::new(PipeOptions {
            background: crate::ui::state::palette_for(&self.internal_state.kind.get()).pipe_base,
            height: top_height as usize,
        });
        top_pipe.x(x).y(0);
        self.add(top_pipe);

        let top_bushing = Bushing::<S>::new(BushingOptions {
            background: crate::ui::state::palette_for(&self.internal_state.kind.get()).pipe_base,
        });
        top_bushing.x(x - 1).y(top_height);
        self.add(top_bushing);

        let bottom_bushing = Bushing::<S>::new(BushingOptions {
            background: crate::ui::state::palette_for(&self.internal_state.kind.get()).pipe_base,
        });
        bottom_bushing
            .x(x - 1)
            .y(top_height + 1 + GAME_GAP_ROWS);
        self.add(bottom_bushing);

        let bottom_pipe = Pipe::<S>::new(PipeOptions {
            background: crate::ui::state::palette_for(&self.internal_state.kind.get()).pipe_base,
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
        self.send_overlays_to_front();
    }

    /// Brings scores, titles and fliers in front of everything else
    /// so they always stay on top of freshly spawned obstacles.
    fn send_overlays_to_front(&self) {
        for score in self.elements.cot::<Score<S>>() {
            let ptr = Rc::as_ptr(&score);
            self.to_front_of_type_where::<Score<S>, _>(|s| {
                std::ptr::eq(s as *const _, ptr)
            });
        }
        for image in self.elements.cot::<U16Image<S>>() {
            let ptr = Rc::as_ptr(&image);
            self.to_front_of_type_where::<U16Image<S>, _>(|i| {
                std::ptr::eq(i as *const _, ptr)
            });
        }
        for panel in self.elements.cot::<ScorePanel<S>>() {
            let ptr = Rc::as_ptr(&panel);
            self.to_front_of_type_where::<ScorePanel<S>, _>(|p| {
                std::ptr::eq(p as *const _, ptr)
            });
        }
        for title in self.elements.cot::<BoxedTextFont<S>>() {
            let ptr = Rc::as_ptr(&title);
            self.to_front_of_type_where::<BoxedTextFont<S>, _>(|t| {
                std::ptr::eq(t as *const _, ptr)
            });
        }
        for image in self.elements.cot::<Image<S>>() {
            let ptr = Rc::as_ptr(&image);
            self.to_front_of_type_where::<Image<S>, _>(|i| {
                std::ptr::eq(i as *const _, ptr)
            });
        }
        for bird in self.elements.cot::<FlyingBird<S>>() {
            let ptr = Rc::as_ptr(&bird);
            self.to_front_of_type_where::<FlyingBird<S>, _>(|b| {
                std::ptr::eq(b as *const _, ptr)
            });
        }
        for cat in self.elements.cot::<FlyingCat<S>>() {
            let ptr = Rc::as_ptr(&cat);
            self.to_front_of_type_where::<FlyingCat<S>, _>(|c| {
                std::ptr::eq(c as *const _, ptr)
            });
        }
        for bird in self.elements.cot::<DeadBird<S>>() {
            let ptr = Rc::as_ptr(&bird);
            self.to_front_of_type_where::<DeadBird<S>, _>(|b| {
                std::ptr::eq(b as *const _, ptr)
            });
        }
        for cat in self.elements.cot::<DeadCat<S>>() {
            let ptr = Rc::as_ptr(&cat);
            self.to_front_of_type_where::<DeadCat<S>, _>(|c| {
                std::ptr::eq(c as *const _, ptr)
            });
        }
        for btn in self.elements.cot::<Button<S>>() {
            let ptr = Rc::as_ptr(&btn);
            self.to_front_of_type_where::<Button<S>, _>(|b| {
                std::ptr::eq(b as *const _, ptr)
            });
        }
        for btn in self.elements.cot::<ImageButton<S>>() {
            let ptr = Rc::as_ptr(&btn);
            self.to_front_of_type_where::<ImageButton<S>, _>(|b| {
                std::ptr::eq(b as *const _, ptr)
            });
        }
    }

    /// Flies bullets right, killing aliens, dying on pipes,
    /// expiring at the right edge.
    fn step_bullets(&self) {
        for bullet in self.elements.cot::<Bullet<S>>() {
            bullet.x(bullet.get_x() + BULLET_SPEED);
        }
        for bullet in self.elements.cot::<Bullet<S>>() {
            let bx = bullet.get_x();
            let by = bullet.get_y();
            if bx >= self.get_x() + GAME_WIDTH as isize {
                while self
                    .elements
                    .sot_w::<Bullet<S>, _>(|b| b.get_x() == bx && b.get_y() == by)
                    .is_some()
                {}
                continue;
            }
            let pipe_hit = self
                .elements
                .cot::<Pipe<S>>()
                .into_iter()
                .any(|p| bullet.intersects_element(p.as_ref()))
                || self
                    .elements
                    .cot::<Bushing<S>>()
                    .into_iter()
                    .any(|b| bullet.intersects_element(b.as_ref()));
            if pipe_hit {
                while self
                    .elements
                    .sot_w::<Bullet<S>, _>(|b| b.get_x() == bx && b.get_y() == by)
                    .is_some()
                {}
                continue;
            }
            let alien_hit = self
                .elements
                .cot::<Alien<S>>()
                .into_iter()
                .find(|a| bullet.intersects_element(a.as_ref()));
            if let Some(hit) = alien_hit {
                let (ax, ay, aw) = (hit.get_x(), hit.get_y(), hit.width() as isize);
                self.add_score(1);
                while self
                    .elements
                    .sot_w::<Alien<S>, _>(|a| {
                        a.get_x() == ax && a.get_y() == ay && a.width() as isize == aw
                    })
                    .is_some()
                {}
                while self
                    .elements
                    .sot_w::<Bullet<S>, _>(|b| b.get_x() == bx && b.get_y() == by)
                    .is_some()
                {}
            }
        }
    }

    /// Moves the world one cell left, wraps ground, spawns and
    /// removes pipes. Title, hint and birds stay put.
    fn step(&self) {
        if self.internal_state.spawning.get() || self.internal_state.dying.get() {
            self.fall();
        }
        if self.internal_state.dying.get() {
            return;
        }
        for pipe in self.elements.cot::<Pipe<S>>() {
            pipe.x(pipe.get_x() - 1);
        }
        for stem in self.elements.cot::<FlowerStem<S>>() {
            stem.x(stem.get_x() - 1);
        }
        for bud in self.elements.cot::<FlowerBud<S>>() {
            bud.x(bud.get_x() - 1);
        }
        for alien in self.elements.cot::<Alien<S>>() {
            alien.x(alien.get_x() - 1);
            alien.step_vertical(
                self.get_y() + ALIEN_MIN_Y,
                self.get_y() + ALIEN_MAX_Y,
            );
        }
        for bushing in self.elements.cot::<Bushing<S>>() {
            bushing.x(bushing.get_x() - 1);
        }

        // March starts once the alien is fully on-screen, freezes at the wall.
        for alien in self.elements.cot::<Alien<S>>() {
            let w = alien.width() as isize;
            if !alien.marching()
                && alien.get_x() > self.get_x()
                && alien.get_x() + w <= self.get_x() + GAME_WIDTH as isize
            {
                alien.start_marching();
            }
            if alien.marching() && alien.get_x() <= self.get_x() {
                alien.stop_marching();
            }
        }
        self.step_bullets();

        // Pavement marquee: one cell left per step, wrapping by its
        // 2-wide stripe period for a seamless scroll illusion.
        for pavement in self.elements.cot::<Pavement<S>>() {
            let x = pavement.get_x() - 1;
            if x - self.get_x() <= -2 {
                pavement.x(x + 2);
            } else {
                pavement.x(x);
            }
        }

        // Pipe (6) + gap (24) cadence.
        if self.internal_state.spawning.get() {
            let distance = self.internal_state.distance.get() + 1;
            if distance >= self.options.spawn_gap {
                self.spawn_obstacle_at(GAME_SPAWN_X);
                self.internal_state.distance.set(0);
            } else {
                self.internal_state.distance.set(distance);
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

        // Alien passed: right edge reaches the bird.
        for alien in self.elements.cot::<Alien<S>>() {
            if alien.get_x() + alien.width() as isize == self.get_x() + BIRD_X {
                self.add_score(1);
            }
        }

        // Drop fully off-screen pipes.
        while self
            .elements
            .sot_w::<Sparkle<S>, _>(|s| !s.is_live())
            .is_some()
        {}
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
        while self
            .elements
            .sot_w::<FlowerStem<S>, _>(|s| {
                s.get_x() + 3 <= self.get_x()
            })
            .is_some()
        {}
        while self
            .elements
            .sot_w::<FlowerBud<S>, _>(|b| {
                b.get_x() + 5 <= self.get_x()
            })
            .is_some()
        {}
        while self
            .elements
            .sot_w::<Alien<S>, _>(|a| {
                a.get_x() + a.width() as isize <= self.get_x()
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
