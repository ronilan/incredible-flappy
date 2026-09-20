use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const BUSHING_WIDTH: usize = 8;
pub const BUSHING_HEIGHT: usize = 1;

#[derive(Clone, Debug)]
pub struct BushingOptions {
    pub background: u8,
}

impl Default for BushingOptions {
    fn default() -> Self {
        Self {
            background: theme::BUSHING_BACKGROUND,
        }
    }
}

element! {
  pub struct Bushing<S> {
      options: BushingOptions = BushingOptions::default(),
  }
}

impl<S: Clone + PartialEq> Bushing<S> {
    pub fn new(options: BushingOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((BUSHING_WIDTH, BUSHING_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("bushing");

        for (i, bg) in theme::bushing_segments(el.options.background).iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(1)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(i as isize)
                .y(0);
            el.add(seg);
        }


        el
    }
}

impl<S: Clone + PartialEq> Default for Bushing<S> {
    fn default() -> Self {
        Self::new(BushingOptions::default())
    }
}
