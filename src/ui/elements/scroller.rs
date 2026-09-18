use std::cell::Cell;

use incredible::*;
use incredible_macros_decl::element;

use super::bushing::{Bushing, BUSHING_WIDTH};
use super::pipe::{PIPE_WIDTH, Pipe};
use super::scenery::{SCENERY_WIDTH, Scenery};

pub const SCROLLER_WIDTH: usize = 80;
pub const SCROLLER_HEIGHT: usize = 24;
pub const SCROLLER_GROUND_Y: isize = 16;
pub const SCROLLER_SPAWN_X: isize = 80;
pub const SCROLLER_PIPE_Y: isize = 12;
pub const SCROLLER_BUSHING_Y: isize = 11;

#[derive(Clone, Debug)]
pub struct ScrollerOptions {
    pub interval_ms: u128,
    pub spawn_gap: usize,
}

impl Default for ScrollerOptions {
    fn default() -> Self {
        Self {
            interval_ms: 100,
            spawn_gap: 30,
        }
    }
}

element! {
  pub struct Scroller<S> {
      options: ScrollerOptions = ScrollerOptions::default(),
      running: Cell<bool> = Cell::new(false),
      distance: Cell<usize> = Cell::new(0),
  }
}

impl<S: Clone + PartialEq> Scroller<S> {
    pub fn new(options: ScrollerOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((SCROLLER_WIDTH, SCROLLER_HEIGHT, ' ')))
            .handle("scroller");

        // Two 40-wide ground tiles covering the 80-wide window.
        let left = Scenery::<S>::default();
        left.x(0).y(SCROLLER_GROUND_Y);
        el.add(left);

        let right = Scenery::<S>::default();
        right.x(SCENERY_WIDTH as isize).y(SCROLLER_GROUND_Y);
        el.add(right);

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

    /// Clears all pipes and restarts the spawn cadence.
    pub fn reset(&self) -> &Self {
        while self.elements.sot::<Pipe<S>>().is_some() {}
        while self.elements.sot::<Bushing<S>>().is_some() {}
        self.distance.set(0);
        self
    }

    /// Places a pipe + bushing cap at x 80.
    fn spawn_obstacle(&self) {
        let pipe = Pipe::<S>::default();
        pipe.x(SCROLLER_SPAWN_X).y(SCROLLER_PIPE_Y);
        self.add(pipe);

        let bushing = Bushing::<S>::default();
        bushing.x(SCROLLER_SPAWN_X - 1).y(SCROLLER_BUSHING_Y);
        self.add(bushing);
    }

    /// Moves everything one cell left, wraps ground tiles, spawns and
    /// removes pipes.
    pub fn step(&self) {
        for child in self.elements.iter() {
            child.x(child.get_x() - 1);
        }

        // Leapfrog ground tiles for an endless floor.
        for tile in self.elements.cot::<Scenery<S>>() {
            if tile.get_x() + SCENERY_WIDTH as isize <= 0 {
                tile.x(tile.get_x() + SCENERY_WIDTH as isize * 2);
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

impl<S: Clone + PartialEq> Default for Scroller<S> {
    fn default() -> Self {
        Self::new(ScrollerOptions::default())
    }
}
