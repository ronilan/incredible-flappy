use std::cell::Cell;

use incredible::*;
use incredible_macros_decl::element;

use crate::ui::elements::BoxedText;

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
      internal_state: ScoreState = ScoreState::default(),
  }
}

#[derive(Clone, Debug, Default)]
pub struct ScoreState {
    pub current: Cell<u32>,
}

impl<S: Clone + PartialEq + 'static> Score<S> {
    pub fn new(options: ScoreOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        el.internal_state.current.set(el.options.value);

        let display = BoxedText::<S>::default();
        display
            .text(el.internal_state.current.get().to_string().as_str())
            .handle("score_display");

        el.look(Look::from((
            display.visual.look.width(),
            display.visual.look.height(),
            ' ',
        )))
        .handle("score");

        el.add(display);
        el.showed(false);

        el
    }

    pub fn get_value(&self) -> u32 {
        self.internal_state.current.get()
    }

    pub fn set_value(&self, value: u32) -> &Self {
        self.internal_state.current.set(value);
        if let Some(display) = self.elements.cot::<BoxedText<S>>().first() {
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
