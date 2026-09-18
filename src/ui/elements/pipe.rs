use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use super::pipe_slice::PipeSlice;

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
            background: 34,
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

        for row in 0..el.options.height {
            let slice = PipeSlice::<S>::default();
            slice.x(el.get_x()).y(el.get_y() + row as isize);
            el.add(slice);
        }

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Pipe<S> {
    fn default() -> Self {
        Self::new(PipeOptions::default())
    }
}
