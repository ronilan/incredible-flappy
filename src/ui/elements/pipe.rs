use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const PIPE_WIDTH: usize = 6;
pub const PIPE_HEIGHT: usize = 10;
pub const PIPE_SEGMENT_BACKGROUNDS: [u8; 6] = [28, 34, 34, 40, 46, 40];

#[derive(Clone, Debug)]
pub struct PipeOptions {
    pub background: u8,
}

impl Default for PipeOptions {
    fn default() -> Self {
        Self { background: 34 }
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

        el.look(Look::from((PIPE_WIDTH, PIPE_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pipe");

        for (i, bg) in PIPE_SEGMENT_BACKGROUNDS.iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(PIPE_HEIGHT)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(i as isize)
                .y(0);
            seg.decorate();
            el.add(seg);
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
