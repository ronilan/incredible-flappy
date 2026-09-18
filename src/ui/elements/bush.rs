use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;
use rand::Rng;
use rand::rng;

/// Fill character for a bush by column index:
/// `(index % 3) ? '.' : ((index % 4) ? '`' : '^')`.
pub fn bush_fill(index: usize) -> char {
    if index % 3 != 0 {
        '.'
    } else if index % 4 != 0 {
        '`'
    } else {
        '^'
    }
}

#[derive(Clone, Debug)]
pub struct BushOptions {
    pub index: usize,
    pub background: u8,
    pub color: u8,
}

impl Default for BushOptions {
    fn default() -> Self {
        Self {
            index: 0,
            background: 156,
            color: 40,
        }
    }
}

element! {
  pub struct Bush<S> {
      options: BushOptions = BushOptions::default(),
  }
}

impl<S: Clone + PartialEq> Bush<S> {
    pub fn new(options: BushOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        // Height setter: Math.floor(Math.random() * 2) + 1.
        let height = rng().random_range(1..=2);

        el.look(Look::from((2, height, bush_fill(el.options.index))))
            .background(Some(Color::Ansi(el.options.background)))
            .color(Some(Color::Ansi(el.options.color)))
            .handle("bush");

        el.decorate();

        el
    }
}
