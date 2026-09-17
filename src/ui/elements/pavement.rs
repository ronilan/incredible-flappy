use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const PAVEMENT_WIDTH: usize = 80;
pub const PAVEMENT_HEIGHT: usize = 1;

#[derive(Clone, Debug)]
pub struct PavementOptions {
    pub background: u8,
}

impl Default for PavementOptions {
    fn default() -> Self {
        Self { background: 34 }
    }
}

element! {
  pub struct Pavement<S> {
      options: PavementOptions = PavementOptions::default(),
  }
}

impl<S: Clone + PartialEq> Pavement<S> {
    pub fn new(options: PavementOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((PAVEMENT_WIDTH, PAVEMENT_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pavement");

        for i in 0..PAVEMENT_WIDTH {
            let bg = if i % 2 == 0 {
                el.options.background
            } else {
                el.options.background + 6
            };
            let block = Rectangle::<S>::new();
            block
                .width(1)
                .height(1)
                .fill(Some(' '))
                .background(Some(Color::Ansi(bg)))
                .x(i as isize)
                .y(0);
            block.decorate();
            el.add(block);
        }

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Pavement<S> {
    fn default() -> Self {
        Self::new(PavementOptions::default())
    }
}
