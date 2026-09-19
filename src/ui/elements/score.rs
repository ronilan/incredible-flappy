use std::cell::Cell;

use incredible::*;
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_helpers_effects::*;
use incredible_macros_decl::element;

#[derive(Clone, Debug)]
pub struct ScoreOptions {
    pub value: u32,
}

impl Default for ScoreOptions {
    fn default() -> Self {
        Self { value: 0 }
    }
}

element! {
  pub struct Score<S> {
      options: ScoreOptions = ScoreOptions::default(),
      current: Cell<u32> = Cell::new(0),
  }
}

fn score_effects<S: Clone + PartialEq + 'static>(el: &BlockCharsStr<S>) {
    decorate_rules::<S, BlockCharsStr<S>>(el, score_effects);
}

impl<S: Clone + PartialEq + 'static> Score<S> {
    pub fn new(options: ScoreOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        el.current.set(el.options.value);

        let display = BlockCharsStr::<S>::default();
        display
            .text(el.current.get().to_string().as_str())
            .size(BlockSize::Small)
            .style_handle("ScoreGradient")
            .handle("score_display");
        effect(&display, score_effects);

        el.look(Look::from((
            display.visual.look.width(),
            display.visual.look.height(),
            ' ',
        )))
        .handle("score");
        el.add(display);


        el
    }

    pub fn get_value(&self) -> u32 {
        self.current.get()
    }

    pub fn set_value(&self, value: u32) -> &Self {
        self.current.set(value);
        if let Some(display) = self.elements.cot::<BlockCharsStr<S>>().first() {
            display.text(value.to_string().as_str());
            self.look(Look::from((
                display.visual.look.width(),
                display.visual.look.height(),
                ' ',
            )));
        }
        self
    }
}

impl<S: Clone + PartialEq> Default for Score<S> {
    fn default() -> Self {
        Self::new(ScoreOptions::default())
    }
}
