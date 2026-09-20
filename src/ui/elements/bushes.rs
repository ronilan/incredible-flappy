use incredible::*;
use incredible_macros_decl::element;

use crate::ui::elements::{Bush, BushOptions};

pub const BUSHES_WIDTH: usize = 40;
pub const BUSHES_HEIGHT: usize = 2;

#[derive(Clone, Debug)]
pub struct BushesOptions {
    pub width: usize,
    pub bush_background: u8,
    pub bush_color: u8,
}

impl Default for BushesOptions {
    fn default() -> Self {
        Self {
            width: BUSHES_WIDTH,
            bush_background: crate::ui::theme::BUSH_BACKGROUND,
            bush_color: crate::ui::theme::BUSH_COLOR,
        }
    }
}

element! {
  pub struct Bushes<S> {
      options: BushesOptions = BushesOptions::default(),
  }
}

impl<S: Clone + PartialEq> Bushes<S> {
    pub fn new(options: BushesOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((
            el.options.width,
            BUSHES_HEIGHT
        )))
        .handle("bushes");

        for index in 0..el.options.width {
            let bush = Bush::<S>::new(BushOptions {
                index,
                background: el.options.bush_background,
                color: el.options.bush_color,
            });
            // Bottom-align within the 2-row strip.
            let h = bush.visual.look.height();
            bush.x(index as isize).y((BUSHES_HEIGHT - h) as isize);
            el.add(bush);
        }


        el
    }
}

impl<S: Clone + PartialEq> Default for Bushes<S> {
    fn default() -> Self {
        Self::new(BushesOptions::default())
    }
}
