use std::cell::{Cell, RefCell};

use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

#[derive(Clone, Debug)]
pub struct SparkleOptions {
    pub color: u8,
}

impl Default for SparkleOptions {
    fn default() -> Self {
        Self {
            color: theme::SPARKLE_COLOR,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SparkleState {
    pub live: Cell<bool>,
}

element! {
    pub struct Sparkle<S> {
      options: SparkleOptions = SparkleOptions::default(),
      internal_state: SparkleState = SparkleState::default(),
      created: RefCell<f64> = RefCell::new(Globals::now()),
    }
}

impl<S: Clone + PartialEq> Sparkle<S> {
    pub fn new(options: SparkleOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        // Style color survives decorate(); hand-painted block colors don't.
        el.color(Some(Color::Ansi(el.options.color)));

        el.internal_on_loop(|el, _, event| {
            let elapsed = event.timestamp - *el.created.borrow();

            if elapsed > 500.0 {
                el.internal_state.live.set(false);
            } else {
                el.look(Look::from("*"));
            }

            el.decorate();
            el.draw();
        });

        el.internal_state.live.set(true);

        el
    }

    /// False once its 500ms are up; the game collects the dead.
    pub fn is_live(&self) -> bool {
        self.internal_state.live.get()
    }
}

impl<S: Clone + PartialEq> Default for Sparkle<S> {
    fn default() -> Self {
        Self::new(SparkleOptions::default())
    }
}
