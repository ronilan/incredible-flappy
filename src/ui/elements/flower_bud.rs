use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;
use rand::rng;
use rand::seq::IndexedRandom;

use crate::ui::theme;

pub const FLOWER_BUD_WIDTH: usize = 5;
pub const FLOWER_BUD_HEIGHT: usize = 5;

#[derive(Clone, Debug)]
pub struct FlowerBudOptions {
    pub width: usize,
    pub height: usize,
    pub color: Option<u8>,
}

impl Default for FlowerBudOptions {
    fn default() -> Self {
        Self {
            width: FLOWER_BUD_WIDTH,
            height: FLOWER_BUD_HEIGHT,
            color: None,
        }
    }
}

element! {
  pub struct FlowerBud<S> {
      options: FlowerBudOptions = FlowerBudOptions::default(),
  }
}

impl<S: Clone + PartialEq> FlowerBud<S> {
    pub fn new(options: FlowerBudOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let width = el.options.width;
        let height = el.options.height;
        let color = el.options.color.unwrap_or_else(|| {
            *theme::FLOWER_BUD_COLORS
                .choose(&mut rng())
                .unwrap_or(&theme::FLOWER_BUD_COLORS[0])
        }) as i16;

        el.look(Look::from((width, height, ' ')))
            .handle("flower_bud");

        for index in 0..width {
            let h = height - index % 2;
            let col_h = h - usize::from(index == 0 || index == width - 1);
            let bg = (color - (index as i16 - (width - 1) as i16 / 2).abs() * 36).max(0) as u8;
            let fill = if index == width / 2 {
                '.'
            } else if index % 2 == 1 {
                '^'
            } else {
                '*'
            };
            let col = Rectangle::<S>::new();
            col.width(1)
                .height(col_h)
                .fill(Some(fill))
                .color(Some(Color::Ansi(theme::FLOWER_BUD_FG)))
                .background(Some(Color::Ansi(bg)))
                .x(el.get_x() + index as isize)
                .y(el.get_y() + (height - h) as isize);
            el.add(col);
        }

        el
    }
}

impl<S: Clone + PartialEq> Default for FlowerBud<S> {
    fn default() -> Self {
        Self::new(FlowerBudOptions::default())
    }
}
