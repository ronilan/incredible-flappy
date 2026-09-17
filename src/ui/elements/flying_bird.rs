use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const FLYING_BIRD_WIDTH: usize = 5;
pub const FLYING_BIRD_HEIGHT: usize = 2;

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
            .color(Some(Color::Ansi(16)))
            .background(Some(Color::Ansi(231)))
            .x(3)
            .y(0);
        eye.decorate();
        el.add(eye);

        // Wing.
        let wing = Rectangle::<S>::new();
        wing.width(2)
            .height(1)
            .fill(Some('─'))
            .color(Some(Color::Ansi(16)))
            .background(Some(Color::Ansi(160)))
            .x(3)
            .y(1);
        wing.decorate();
        el.add(wing);

        // Body.
        let body = Rectangle::<S>::new();
        body.width(3)
            .height(1)
            .fill(Some(' '))
            .background(Some(Color::Ansi(220)))
            .x(0)
            .y(0);
        body.decorate();
        el.add(body);

        // Belly patch.
        let belly = Rectangle::<S>::new();
        belly
            .width(1)
            .height(1)
            .fill(Some(' '))
            .background(Some(Color::Ansi(214)))
            .x(2)
            .y(1);
        belly.decorate();
        el.add(belly);

        // Beak.
        let beak = Rectangle::<S>::new();
        beak.width(2)
            .height(1)
            .fill(Some('>'))
            .color(Some(Color::Ansi(231)))
            .background(Some(Color::Ansi(214)))
            .x(0)
            .y(1);
        beak.decorate();
        el.add(beak);

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for FlyingBird<S> {
    fn default() -> Self {
        Self::new(FlyingBirdOptions::default())
    }
}
