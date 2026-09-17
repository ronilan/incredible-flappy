use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;
use incredible_macros_decl::element;
use rand::rng;
use rand::Rng;

// -----------------------------
// Shared spec constants
// -----------------------------

pub const PIPE_WIDTH: usize = 6;
pub const PIPE_HEIGHT: usize = 10;
pub const PIPE_SEGMENT_BACKGROUNDS: [u8; 6] = [28, 34, 34, 40, 46, 40];

pub const BUSHING_WIDTH: usize = 8;
pub const BUSHING_HEIGHT: usize = 1;
pub const BUSHING_SEGMENT_BACKGROUNDS: [u8; 8] = [28, 34, 34, 40, 40, 40, 46, 40];

pub const PAVEMENT_WIDTH: usize = 80;
pub const PAVEMENT_HEIGHT: usize = 1;

pub const BUILDINGS_COLOR: u8 = 244;
pub const BUILDINGS_STR: &str = "    _     ___       |^^^|  \n __| |___|:::|    __|:::|  \n|oo|.|* *|:::|   |''|:::|  \n|oo|.|** |:::|   |''|:::|  \n|_o|_|[]_|_|_|   |_'|___| ";

pub const FLYING_BIRD_WIDTH: usize = 5;
pub const FLYING_BIRD_HEIGHT: usize = 2;

pub const DEAD_BIRD_WIDTH: usize = 2;
pub const DEAD_BIRD_HEIGHT: usize = 4;

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

// -----------------------------
// Pipe
// -----------------------------

#[derive(Clone, Debug)]
pub struct PipeOptions {
    pub background: u8,
}

impl Default for PipeOptions {
    fn default() -> Self {
        Self { background: 34 }
    }
}

element! {
  pub struct Pipe<S> {
      options: PipeOptions = PipeOptions::default(),
  }
}

impl<S: Clone + PartialEq> Pipe<S> {
    pub fn new(options: PipeOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((PIPE_WIDTH, PIPE_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("pipe");

        for (i, bg) in PIPE_SEGMENT_BACKGROUNDS.iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(PIPE_HEIGHT)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(i as isize)
                .y(0);
            seg.decorate();
            el.add(seg);
        }

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Pipe<S> {
    fn default() -> Self {
        Self::new(PipeOptions::default())
    }
}

// -----------------------------
// Bushing
// -----------------------------

#[derive(Clone, Debug)]
pub struct BushingOptions {
    pub background: u8,
}

impl Default for BushingOptions {
    fn default() -> Self {
        Self { background: 34 }
    }
}

element! {
  pub struct Bushing<S> {
      options: BushingOptions = BushingOptions::default(),
  }
}

impl<S: Clone + PartialEq> Bushing<S> {
    pub fn new(options: BushingOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((BUSHING_WIDTH, BUSHING_HEIGHT, ' ')))
            .background(Some(Color::Ansi(el.options.background)))
            .handle("bushing");

        for (i, bg) in BUSHING_SEGMENT_BACKGROUNDS.iter().enumerate() {
            let seg = Rectangle::<S>::new();
            seg.width(1)
                .height(1)
                .fill(Some(' '))
                .background(Some(Color::Ansi(*bg)))
                .x(i as isize)
                .y(0);
            seg.decorate();
            el.add(seg);
        }

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Bushing<S> {
    fn default() -> Self {
        Self::new(BushingOptions::default())
    }
}

// -----------------------------
// Pavement
// -----------------------------

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

// -----------------------------
// Buildings
// -----------------------------

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

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Buildings<S> {
    fn default() -> Self {
        Self::new(BuildingsOptions::default())
    }
}

// -----------------------------
// FlyingBird
// -----------------------------

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

// -----------------------------
// DeadBird
// -----------------------------

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
        eye.decorate();
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
        tail.decorate();
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
        body.decorate();
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
        head.decorate();
        el.add(head);

        el.decorate();

        el
    }
}

// -----------------------------
// Bush
// -----------------------------

#[derive(Clone, Debug)]
pub struct BushOptions {
    pub index: usize,
}

impl Default for BushOptions {
    fn default() -> Self {
        Self { index: 0 }
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
            .handle("bush");

        el.decorate();

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::app::State;

    #[test]
    fn pipe_spec() {
        Globals::draw_enabled(false);
        let pipe = Pipe::<State>::new(PipeOptions::default());
        assert_eq!(pipe.visual.look.width(), PIPE_WIDTH);
        assert_eq!(pipe.visual.look.height(), PIPE_HEIGHT);
        assert_eq!(pipe.get_background(), Some(Color::Ansi(34)));
        let segs = pipe.elements.cot::<Rectangle<State>>();
        assert_eq!(segs.len(), 6);
        for (i, seg) in segs.iter().enumerate() {
            assert_eq!(seg.get_width(), 1);
            assert_eq!(seg.get_height(), PIPE_HEIGHT);
            assert_eq!(
                seg.get_background(),
                Some(Color::Ansi(PIPE_SEGMENT_BACKGROUNDS[i]))
            );
        }
    }

    #[test]
    fn bushing_spec() {
        Globals::draw_enabled(false);
        let bushing = Bushing::<State>::new(BushingOptions::default());
        assert_eq!(bushing.visual.look.width(), BUSHING_WIDTH);
        assert_eq!(bushing.visual.look.height(), BUSHING_HEIGHT);
        assert_eq!(bushing.get_background(), Some(Color::Ansi(34)));
        let segs = bushing.elements.cot::<Rectangle<State>>();
        assert_eq!(segs.len(), 8);
        for (i, seg) in segs.iter().enumerate() {
            assert_eq!(seg.get_width(), 1);
            assert_eq!(seg.get_height(), 1);
            assert_eq!(
                seg.get_background(),
                Some(Color::Ansi(BUSHING_SEGMENT_BACKGROUNDS[i]))
            );
        }
    }

    #[test]
    fn pavement_spec() {
        Globals::draw_enabled(false);
        let pavement = Pavement::<State>::new(PavementOptions::default());
        assert_eq!(pavement.visual.look.width(), PAVEMENT_WIDTH);
        assert_eq!(pavement.visual.look.height(), PAVEMENT_HEIGHT);
        let blocks = pavement.elements.cot::<Rectangle<State>>();
        assert_eq!(blocks.len(), PAVEMENT_WIDTH);
        for (i, block) in blocks.iter().enumerate() {
            let expected = if i % 2 == 0 { 34 } else { 40 };
            assert_eq!(
                block.get_background(),
                Some(Color::Ansi(expected)),
                "pavement block {}",
                i
            );
        }
    }

    #[test]
    fn buildings_spec() {
        Globals::draw_enabled(false);
        let buildings = Buildings::<State>::new(BuildingsOptions::default());
        assert_eq!(buildings.get_color(), Some(Color::Ansi(BUILDINGS_COLOR)));
        let blocks = buildings.visual.look.blocks();
        assert_eq!(blocks.len(), 5);
        let text: String = blocks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| b.content.get().map(|c| c.as_str()).unwrap_or_default())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(text.contains("|^^^|"));
        assert!(text.contains("___"));
    }

    #[test]
    fn birds_spec() {
        Globals::draw_enabled(false);
        let flying = FlyingBird::<State>::new(FlyingBirdOptions::default());
        assert_eq!(flying.visual.look.width(), FLYING_BIRD_WIDTH);
        assert_eq!(flying.visual.look.height(), FLYING_BIRD_HEIGHT);
        assert_eq!(flying.elements.cot::<Rectangle<State>>().len(), 5);
        let dead = DeadBird::<State>::new(DeadBirdOptions::default());
        assert_eq!(dead.visual.look.width(), DEAD_BIRD_WIDTH);
        assert_eq!(dead.visual.look.height(), DEAD_BIRD_HEIGHT);
    }

    #[test]
    fn flying_bird_renders() {        Globals::draw_enabled(false);
        let bird = FlyingBird::<State>::new(FlyingBirdOptions::default());
        let flat = flatten(&bird as &dyn ElementTrait<State>);
        assert_eq!((flat.width(), flat.height()), (5, 2));
        let blocks = flat.blocks();
        let cell = |x: usize, y: usize| {
            blocks[y][x]
                .content
                .get()
                .map(|c| c.as_str())
                .unwrap_or_default()
        };
        let bg = |x: usize, y: usize| blocks[y][x].decor.background.get();
        // Eye.
        assert_eq!(cell(3, 0), ".");
        assert_eq!(bg(3, 0), Some(Color::Ansi(231)));
        // Body.
        assert_eq!(bg(0, 0), Some(Color::Ansi(220)));
        assert_eq!(bg(2, 0), Some(Color::Ansi(220)));
        // Beak.
        assert_eq!(cell(0, 1), ">");
        assert_eq!(cell(1, 1), ">");
        assert_eq!(bg(0, 1), Some(Color::Ansi(214)));
        // Belly patch.
        assert_eq!(bg(2, 1), Some(Color::Ansi(214)));
        // Wing.
        assert_eq!(cell(3, 1), "─");
        assert_eq!(cell(4, 1), "─");
        assert_eq!(bg(3, 1), Some(Color::Ansi(160)));
    }

    #[test]
    fn dead_bird_renders() {
        Globals::draw_enabled(false);
        let bird = DeadBird::<State>::new(DeadBirdOptions::default());
        assert_eq!(bird.elements.cot::<Rectangle<State>>().len(), 4);
        let flat = flatten(&bird as &dyn ElementTrait<State>);
        assert_eq!((flat.width(), flat.height()), (2, 4));
        let blocks = flat.blocks();
        let cell = |x: usize, y: usize| {
            blocks[y][x]
                .content
                .get()
                .map(|c| c.as_str())
                .unwrap_or_default()
        };
        let bg = |x: usize, y: usize| blocks[y][x].decor.background.get();
        // Head.
        assert_eq!(cell(0, 0), "v");
        assert_eq!(cell(0, 1), "v");
        assert_eq!(bg(0, 0), Some(Color::Ansi(214)));
        // Body.
        assert_eq!(bg(1, 0), Some(Color::Ansi(220)));
        assert_eq!(bg(1, 1), Some(Color::Ansi(220)));
        // Tail + eye.
        assert_eq!(cell(0, 2), "|");
        assert_eq!(cell(0, 3), "|");
        assert_eq!(bg(0, 2), Some(Color::Ansi(160)));
        assert_eq!(cell(1, 2), "x");
        assert_eq!(bg(1, 2), Some(Color::Ansi(231)));
    }

    #[test]
    fn pipe_renders_segments_with_backgrounds() {
        Globals::draw_enabled(false);
        let pipe = Pipe::<State>::new(PipeOptions::default());
        let flat = flatten(&pipe as &dyn ElementTrait<State>);
        assert_eq!((flat.width(), flat.height()), (PIPE_WIDTH, PIPE_HEIGHT));
        let blocks = flat.blocks();
        for (x, expected) in PIPE_SEGMENT_BACKGROUNDS.iter().enumerate() {
            for row in blocks.iter() {
                assert_eq!(
                    row[x].decor.background.get(),
                    Some(Color::Ansi(*expected)),
                    "pipe column {}",
                    x
                );
            }
        }
    }

    #[test]
    fn pavement_renders_alternating_backgrounds() {
        Globals::draw_enabled(false);
        let pavement = Pavement::<State>::new(PavementOptions::default());
        let flat = flatten(&pavement as &dyn ElementTrait<State>);
        assert_eq!((flat.width(), flat.height()), (PAVEMENT_WIDTH, 1));
        let blocks = flat.blocks();
        for (i, block) in blocks[0].iter().enumerate() {
            let expected = if i % 2 == 0 { 34 } else { 40 };
            assert_eq!(
                block.decor.background.get(),
                Some(Color::Ansi(expected)),
                "pavement block {}",
                i
            );
        }
    }

    #[test]
    fn bush_spec() {        Globals::draw_enabled(false);
        assert_eq!(bush_fill(1), '.');
        assert_eq!(bush_fill(3), '`');
        assert_eq!(bush_fill(0), '^');
        for index in 0..12 {
            let bush = Bush::<State>::new(BushOptions { index });
            assert_eq!(bush.visual.look.width(), 2);
            let h = bush.visual.look.height();
            assert!((1..=2).contains(&h), "bush height {}", h);
            let first: String = bush.visual.look.blocks()[0]
                .iter()
                .map(|b| b.content.get().map(|c| c.as_str()).unwrap_or_default())
                .collect();
            assert!(
                first.chars().all(|c| c == bush_fill(index)),
                "bush fill {:?}",
                first
            );
        }
    }
}
