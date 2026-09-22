use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const FLYING_BIRD_WIDTH: usize = 7;
pub const FLYING_BIRD_HEIGHT: usize = 3;

#[derive(Clone, Debug)]
pub struct FlyingBirdOptions {
    pub background: Option<u8>,
}

impl Default for FlyingBirdOptions {
    fn default() -> Self {
        Self { background: None }
    }
}

element! {
  pub struct FlyingBird<S> {
      options: FlyingBirdOptions = FlyingBirdOptions::default(),
  }
}

impl<S: Clone + PartialEq> FlyingBird<S> {
    pub fn new(options: FlyingBirdOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((FLYING_BIRD_WIDTH, FLYING_BIRD_HEIGHT, ' ')))
            .handle("flying_bird");
        if let Some(bg) = el.options.background {
            el.background(Some(Color::Ansi(bg)));
        }

        // Eye.
        let eye = Rectangle::<S>::new();
        eye.width(1)
            .height(1)
            .fill(Some('.'))
            .color(Some(Color::Ansi(theme::BIRD_DARK)))
            .background(Some(Color::Ansi(theme::BIRD_LIGHT)))
            .x(3)
            .y(0);
        el.add(eye);

        // Wing.
        let wing = Rectangle::<S>::new();
        wing.width(2)
            .height(1)
            .fill(Some('─'))
            .color(Some(Color::Ansi(theme::BIRD_DARK)))
            .background(Some(Color::Ansi(theme::BIRD_WING_BACKGROUND)))
            .x(3)
            .y(1);
        el.add(wing);

        // Body.
        let body = Rectangle::<S>::new();
        body.width(3)
            .height(1)
            .fill(Some(' '))
            .background(Some(Color::Ansi(theme::BIRD_BODY_BACKGROUND)))
            .x(0)
            .y(0);
        el.add(body);

        // Belly patch.
        let belly = Rectangle::<S>::new();
        belly
            .width(1)
            .height(1)
            .fill(Some(' '))
            .background(Some(Color::Ansi(theme::BIRD_BELLY_BACKGROUND)))
            .x(2)
            .y(1);
        el.add(belly);

        // Beak.
        let beak = Rectangle::<S>::new();
        beak.width(2)
            .height(1)
            .fill(Some('>'))
            .color(Some(Color::Ansi(theme::BIRD_LIGHT)))
            .background(Some(Color::Ansi(theme::BIRD_BELLY_BACKGROUND)))
            .x(0)
            .y(1);
        el.add(beak);


        el
    }
}

impl<S: Clone + PartialEq> Default for FlyingBird<S> {
    fn default() -> Self {
        Self::new(FlyingBirdOptions::default())
    }
}
