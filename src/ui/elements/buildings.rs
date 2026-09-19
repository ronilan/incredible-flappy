use incredible::*;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const BUILDINGS_COLOR: u8 = 244;
pub const BUILDINGS_STR: &str = "    _     ___       |^^^|     \n __| |___|:::|    __|:::|     \n|oo|.|* *|:::|   |''|:::|     \n|oo|.|** |:::|   |''|:::|     \n|_o|_|[]_|_|_|   |_'|___|     ";

#[derive(Clone, Debug)]
pub struct BuildingsOptions {
    pub color: u8,
}

impl Default for BuildingsOptions {
    fn default() -> Self {
        Self {
            color: BUILDINGS_COLOR,
        }
    }
}

element! {
  pub struct Buildings<S> {
      options: BuildingsOptions = BuildingsOptions::default(),
  }
}

impl<S: Clone + PartialEq> Buildings<S> {
    pub fn new(options: BuildingsOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from(BUILDINGS_STR))
            .color(Some(Color::Ansi(el.options.color)))
            .handle("buildings");


        el
    }
}

impl<S: Clone + PartialEq> Default for Buildings<S> {
    fn default() -> Self {
        Self::new(BuildingsOptions::default())
    }
}
