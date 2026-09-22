use incredible::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const BULLET_SPEED: isize = 2;

#[derive(Clone, Debug)]
pub struct BulletOptions {
    pub color: u8,
}

impl Default for BulletOptions {
    fn default() -> Self {
        Self {
            color: theme::BULLET_COLOR,
        }
    }
}

element! {
  pub struct Bullet<S> {
      options: BulletOptions = BulletOptions::default(),
  }
}

impl<S: Clone + PartialEq> Bullet<S> {
    pub fn new(options: BulletOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let look = Look::from("▄");
        for row in look.blocks().iter() {
            for block in row.iter() {
                block.decor.color.set(Some(Color::Ansi(el.options.color)));
            }
        }
        el.look(look).handle("bullet");

        el
    }
}

impl<S: Clone + PartialEq> Default for Bullet<S> {
    fn default() -> Self {
        Self::new(BulletOptions::default())
    }
}
