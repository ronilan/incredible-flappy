use incredible::*;
use incredible_elements_text_fonts::{BlockCharsStr, BlockSize};
use incredible_macros_decl::element;

#[derive(Clone, Debug)]
pub struct ScoreOptions {
    pub value: u32,
}

impl Default for ScoreOptions {
    fn default() -> Self {
        Self { value: 42 }
    }
}

element! {
  pub struct Score<S> {
      options: ScoreOptions = ScoreOptions::default(),
  }
}

impl<S: Clone + PartialEq> Score<S> {
    pub fn new(options: ScoreOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let display = BlockCharsStr::<S>::default();
        display
            .text(el.options.value.to_string().as_str())
            .size(BlockSize::Small)
            .handle("score_display");

        el.look(Look::from((
            display.visual.look.width(),
            display.visual.look.height(),
            ' ',
        )))
        .handle("score");
        el.add(display);

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Score<S> {
    fn default() -> Self {
        Self::new(ScoreOptions::default())
    }
}
