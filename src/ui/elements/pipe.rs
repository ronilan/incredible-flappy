use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const PIPE_WIDTH: usize = 6;
pub const PIPE_HEIGHT: usize = 10;

#[derive(Clone, Debug)]
pub struct PipeOptions {
    pub background: u8,
    pub height: usize,
}

impl Default for PipeOptions {
    fn default() -> Self {
        Self {
            background: theme::PIPE_BACKGROUND,
            height: PIPE_HEIGHT,
        }
    }
}

element! {
  pub struct Pipe<S> {
      options: PipeOptions = PipeOptions::default(),
  }
}

impl<S: Clone + PartialEq> Pipe<S> {
    pub fn new(options: PipeOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((PIPE_WIDTH, el.options.height, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pipe");

        for (i, bg) in theme::PIPE_SEGMENTS.iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(el.options.height)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(i as isize)
                .y(0);
            el.add(seg);
        }


        el
    }
}

impl<S: Clone + PartialEq> Default for Pipe<S> {
    fn default() -> Self {
        Self::new(PipeOptions::default())
    }
}
