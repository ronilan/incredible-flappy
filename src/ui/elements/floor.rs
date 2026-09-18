use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const FLOOR_WIDTH: usize = 40;
pub const FLOOR_HEIGHT: usize = 2;

#[derive(Clone, Debug)]
pub struct FloorOptions {
    pub background: u8,
}

impl Default for FloorOptions {
    fn default() -> Self {
        Self { background: 187 }
    }
}

element! {
  pub struct Floor<S> {
      options: FloorOptions = FloorOptions::default(),
  }
}

impl<S: Clone + PartialEq> Floor<S> {
    pub fn new(options: FloorOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((FLOOR_WIDTH, FLOOR_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("floor");

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Floor<S> {
    fn default() -> Self {
        Self::new(FloorOptions::default())
    }
}
