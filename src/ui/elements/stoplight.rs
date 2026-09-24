use std::cell::Cell;

use incredible::*;
use incredible_elements::{Label, LabelOptions};
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

#[derive(Clone, Debug)]
pub struct StoplightState {
    pub lit: Cell<u8>,
    pub lit_color: Cell<u8>,
}

impl Default for StoplightState {
    fn default() -> Self {
        Self {
            lit: Cell::new(0),
            lit_color: Cell::new(theme::STOPLIGHT_LIT),
        }
    }
}

element! {
  pub struct Stoplight<S> {
      internal_state: StoplightState = StoplightState::default(),
  }
}

impl<S: Clone + PartialEq> Stoplight<S> {
    pub fn new() -> Self {
        let el = Self::blank();
        el.handle("stoplight");
        el.refresh();
        el
    }

    /// Rebuilds the three cells with the first `lit` filled green.
    pub fn refresh(&self) {
        let _ = self.elements.saot::<Label<S>>();

        let lit = self.internal_state.lit.get().min(3) as usize;
        for i in 0..3 {
            let cell = Label::<S>::new(LabelOptions::default());
            cell.text(if i < lit { "●" } else { "○" });
            cell.color(Some(Color::Ansi(if i < lit {
                self.internal_state.lit_color.get()
            } else {
                theme::STOPLIGHT_DIM
            })));
            cell.x(i as isize * 2).y(0);
            cell.capture(Capture::all()).fused(true);
            self.add(cell);
        }
        self.look(Look::from((5, 1, ' ')));
        self.draw();
    }
}

impl<S: Clone + PartialEq> Stoplight<S> {
    /// Sets how many of the three cells burn green.
    pub fn set_lit(&self, lit: u8) -> &Self {
        self.internal_state.lit.set(lit.min(3));
        self.refresh();
        self
    }

    /// Swaps the lit color, e.g. on palette changes.
    pub fn set_lit_color(&self, lit_color: u8) -> &Self {
        self.internal_state.lit_color.set(lit_color);
        self.refresh();
        self
    }
}

impl<S: Clone + PartialEq> Default for Stoplight<S> {
    fn default() -> Self {
        Self::new()
    }
}
