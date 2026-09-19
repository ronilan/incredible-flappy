use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

pub const DEAD_BIRD_WIDTH: usize = 2;
pub const DEAD_BIRD_HEIGHT: usize = 4;

#[derive(Clone, Debug)]
pub struct DeadBirdOptions {
    pub background: Option<u8>,
}

impl Default for DeadBirdOptions {
    fn default() -> Self {
        Self { background: None }
    }
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
            .fill(Some('x'))
            .color(Some(Color::Ansi(16)))
            .background(Some(Color::Ansi(231)))
            .x(1)
            .y(2);
        el.add(eye);

        // Tail.
        let tail = Rectangle::<S>::new();
        tail.width(1)
            .height(2)
            .fill(Some('|'))
            .color(Some(Color::Ansi(16)))
            .background(Some(Color::Ansi(160)))
            .x(0)
            .y(2);
        el.add(tail);

        // Body.
        let body = Rectangle::<S>::new();
        body.width(1)
            .height(2)
            .fill(Some(' '))
            .color(Some(Color::Ansi(16)))
            .background(Some(Color::Ansi(220)))
            .x(1)
            .y(0);
        el.add(body);

        // Head.
        let head = Rectangle::<S>::new();
        head.width(1)
            .height(2)
            .fill(Some('v'))
            .color(Some(Color::Ansi(231)))
            .background(Some(Color::Ansi(214)))
            .x(0)
            .y(0);
        el.add(head);


        el
    }
}
