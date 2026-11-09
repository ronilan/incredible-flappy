use incredible::*;
use incredible_elements::{FillKind, Rectangle};
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

pub const DEAD_BIRD_WIDTH: usize = 2;
pub const DEAD_BIRD_HEIGHT: usize = 5;

#[derive(Clone, Debug)]
#[derive(Default)]
pub struct DeadBirdOptions {
    pub background: Option<u8>,
}


element! {
  pub struct DeadBird<S> {
      options: DeadBirdOptions = DeadBirdOptions::default(),
  }
}

impl<S: Clone + PartialEq> Default for DeadBird<S> {
    fn default() -> Self {
        Self::new(DeadBirdOptions::default())
    }
}

impl<S: Clone + PartialEq> DeadBird<S> {
    pub fn new(options: DeadBirdOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((DEAD_BIRD_WIDTH, DEAD_BIRD_HEIGHT, ' ')))
            .handle("dead_bird");
        if let Some(bg) = el.options.background {
            el.background(Some(Color::Ansi(bg)));
        }

        // Eye.
        let eye = Rectangle::<S>::new();
        eye.width(1)
            .height(1)
            .fill(Some(FillKind::Custom('x')))
            .color(Some(Color::Ansi(theme::BIRD_DARK)))
            .background(Some(Color::Ansi(theme::BIRD_LIGHT)))
            .x(1)
            .y(2);
        el.add(eye);

        // Tail.
        let tail = Rectangle::<S>::new();
        tail.width(1)
            .height(2)
            .fill(Some(FillKind::Custom('|')))
            .color(Some(Color::Ansi(theme::BIRD_DARK)))
            .background(Some(Color::Ansi(theme::BIRD_WING_BACKGROUND)))
            .x(0)
            .y(2);
        el.add(tail);

        // Body.
        let body = Rectangle::<S>::new();
        body.width(1)
            .height(2)
            .fill(Some(FillKind::Custom(' ')))
            .color(Some(Color::Ansi(theme::BIRD_DARK)))
            .background(Some(Color::Ansi(theme::BIRD_BODY_BACKGROUND)))
            .x(1)
            .y(0);
        el.add(body);

        // Head.
        let head = Rectangle::<S>::new();
        head.width(1)
            .height(2)
            .fill(Some(FillKind::Custom('v')))
            .color(Some(Color::Ansi(theme::BIRD_LIGHT)))
            .background(Some(Color::Ansi(theme::BIRD_BELLY_BACKGROUND)))
            .x(0)
            .y(0);
        el.add(head);


        el
    }
}
