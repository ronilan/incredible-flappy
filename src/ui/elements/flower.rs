use incredible::*;
use incredible_macros_decl::element;

use super::flower_bud::{FLOWER_BUD_HEIGHT, FLOWER_BUD_WIDTH, FlowerBud, FlowerBudOptions};
use super::flower_stem::{FLOWER_STEM_WIDTH, FlowerStem, FlowerStemOptions};

#[derive(Clone, Debug)]
pub struct FlowerOptions {
    pub stem_height: usize,
    pub bud_width: usize,
    pub bud_height: usize,
    pub bud_color: Option<u8>,
}

impl Default for FlowerOptions {
    fn default() -> Self {
        Self {
            stem_height: 6,
            bud_width: FLOWER_BUD_WIDTH,
            bud_height: FLOWER_BUD_HEIGHT,
            bud_color: None,
        }
    }
}

element! {
  pub struct Flower<S> {
      options: FlowerOptions = FlowerOptions::default(),
  }
}

impl<S: Clone + PartialEq> Flower<S> {
    pub fn new(options: FlowerOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let total_height = el.options.bud_height + el.options.stem_height;
        let footprint = FLOWER_STEM_WIDTH + 2;
        let look_width = el.options.bud_width.max(footprint);
        el.look(Look::from((look_width, total_height, ' ')))
            .handle("flower");

        // Bud on top, stem centered under it. Both inside the parent look.
        let bud_x = el.get_x() + (look_width as isize - el.options.bud_width as isize) / 2;
        let stem_x = el.get_x() + (look_width as isize - footprint as isize) / 2;
        let bud = FlowerBud::<S>::new(FlowerBudOptions {
            width: el.options.bud_width,
            height: el.options.bud_height,
            color: el.options.bud_color,
        });
        bud.x(bud_x).y(el.get_y());
        el.add(bud);

        let stem = FlowerStem::<S>::new(FlowerStemOptions {
            height: el.options.stem_height,
        });
        stem
            .x(stem_x)
            .y(el.get_y() + el.options.bud_height as isize);
        el.add(stem);

        el
    }
}

impl<S: Clone + PartialEq> Default for Flower<S> {
    fn default() -> Self {
        Self::new(FlowerOptions::default())
    }
}
