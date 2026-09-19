use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const PAVEMENT_WIDTH: usize = 40;
pub const PAVEMENT_HEIGHT: usize = 1;

#[derive(Clone, Debug)]
pub struct PavementOptions {
    pub background: u8,
    pub width: usize,
}

impl Default for PavementOptions {
    fn default() -> Self {
        Self {
            background: theme::PAVEMENT_BASE,
            width: PAVEMENT_WIDTH,
        }
    }
}

element! {
  pub struct Pavement<S> {
      options: PavementOptions = PavementOptions::default(),
  }
}

impl<S: Clone + PartialEq> Pavement<S> {
    pub fn new(options: PavementOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((el.options.width, PAVEMENT_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pavement");

        for i in 0..el.options.width {
            let bg = if i % 2 == 0 {
                el.options.background
            } else {
                el.options.background + theme::PAVEMENT_STEP
            };
            let block = Rectangle::<S>::new();
            block
                .width(1)
                .height(1)
                .fill(Some(' '))
                .background(Some(Color::Ansi(bg)))
                .x(i as isize)
                .y(0);
            el.add(block);
        }


        el
    }
}

impl<S: Clone + PartialEq> Default for Pavement<S> {
    fn default() -> Self {
        Self::new(PavementOptions::default())
    }
}
