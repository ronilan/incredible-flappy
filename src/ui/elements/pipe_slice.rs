use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const PIPE_SLICE_WIDTH: usize = 6;
pub const PIPE_SLICE_SEGMENT_BACKGROUNDS: [u8; 6] = [28, 34, 34, 40, 46, 40];

#[derive(Clone, Debug)]
pub struct PipeSliceOptions {
    pub background: u8,
}

impl Default for PipeSliceOptions {
    fn default() -> Self {
        Self { background: 34 }
    }
}

element! {
  pub struct PipeSlice<S> {
      options: PipeSliceOptions = PipeSliceOptions::default(),
  }
}

impl<S: Clone + PartialEq> PipeSlice<S> {
    pub fn new(options: PipeSliceOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((PIPE_SLICE_WIDTH, 1, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pipe_slice");

        for (i, bg) in PIPE_SLICE_SEGMENT_BACKGROUNDS.iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(1)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(el.get_x() + i as isize)
                .y(el.get_y());
            seg.decorate();
            el.add(seg);
        }

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for PipeSlice<S> {
    fn default() -> Self {
        Self::new(PipeSliceOptions::default())
    }
}
