use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const FLOWER_STEM_WIDTH: usize = 3;
pub const FLOWER_STEM_HEIGHT: usize = 6;

#[derive(Clone, Debug)]
pub struct FlowerStemOptions {
    pub height: usize,
}

impl Default for FlowerStemOptions {
    fn default() -> Self {
        Self {
            height: FLOWER_STEM_HEIGHT,
        }
    }
}

element! {
  pub struct FlowerStem<S> {
      options: FlowerStemOptions = FlowerStemOptions::default(),
  }
}

impl<S: Clone + PartialEq> FlowerStem<S> {
    pub fn new(options: FlowerStemOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((
            FLOWER_STEM_WIDTH,
            el.options.height,
            ' ',
        )))
        .handle("flower_stem");

        // Middle column.
        let middle = Rectangle::<S>::new();
        middle
            .width(1)
            .height(el.options.height)
            .fill(Some(' '))
            .background(Some(Color::Ansi(theme::FLOWER_STEM)))
            .x(el.get_x() + 1)
            .y(el.get_y());
        el.add(middle);

        // Leaves on the outside columns, mid-stem.
        let leaf_y = el.get_y() + (el.options.height.saturating_sub(2)) as isize / 2;
        for dx in [0, FLOWER_STEM_WIDTH as isize - 1] {
            let leaf = Rectangle::<S>::new();
            leaf.width(1)
                .height(2)
                .fill(Some(' '))
                .background(Some(Color::Ansi(theme::FLOWER_LEAF)))
                .x(el.get_x() + dx)
                .y(leaf_y);
            el.add(leaf);
        }

        el
    }
}

impl<S: Clone + PartialEq> Default for FlowerStem<S> {
    fn default() -> Self {
        Self::new(FlowerStemOptions::default())
    }
}
