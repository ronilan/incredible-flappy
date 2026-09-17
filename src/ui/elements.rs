use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_helpers_styling::*;

use crate::ui::app::State;

pub const PIPE_WIDTH: usize = 6;
pub const PIPE_HEIGHT: usize = 10;
pub const PIPE_BACKGROUND: u8 = 34;
pub const PIPE_SEGMENT_BACKGROUNDS: [u8; 6] = [38, 44, 44, 50, 56, 62];

pub const BUSHING_WIDTH: usize = 8;
pub const BUSHING_HEIGHT: usize = 1;
pub const BUSHING_BACKGROUND: u8 = 34;
pub const BUSHING_SEGMENT_BACKGROUNDS: [u8; 8] = [38, 44, 44, 50, 50, 50, 56, 62];

pub const PAVEMENT_WIDTH: usize = 80;
pub const PAVEMENT_HEIGHT: usize = 1;

pub const BUILDINGS_COLOR: u8 = 244;
pub const BUILDINGS_STR: &str = "_     ___       |^^^|  \n __| |___|:::|    __|:::|  \n|oo|.|* *|:::|   |''|:::|  \n|oo|.|** |:::|   |''|:::|  \n|_o|_|[]_|_|_|   |_'|___| ";

pub const FLYING_BIRD_WIDTH: usize = 5;
pub const FLYING_BIRD_HEIGHT: usize = 2;

pub const DEAD_BIRD_WIDTH: usize = 2;
pub const DEAD_BIRD_HEIGHT: usize = 4;

pub fn bush_fill(index: usize) -> char {
    if index % 3 != 0 {
        '.'
    } else if index % 4 != 0 {
        '`'
    } else {
        '^'
    }
}

pub fn bush_height() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as usize)
        .unwrap_or(1);
    // 1 or 2.
    nanos % 2 + 1
}

fn hand_drawn<S: Clone + PartialEq>(el: &impl ElementTrait<S>) {
    el.decoratable(false);
}

pub fn build_pipe(x: isize, y: isize) -> Rectangle<State> {
    let pipe = Rectangle::<State>::new();
    pipe.width(PIPE_WIDTH)
        .height(PIPE_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::ansi(PIPE_BACKGROUND)))
        .handle("pipe")
        .x(x)
        .y(y);
    for (i, bg) in PIPE_SEGMENT_BACKGROUNDS.iter().enumerate() {
        let seg = Rectangle::<State>::new();
        seg.width(1)
            .height(PIPE_HEIGHT)
            .fill(Some(' '))
            .background(Some(Color::ansi(*bg)))
            .x(x + i as isize)
            .y(y);
        seg.decoratable(false);
        pipe.add(seg);
    }
    hand_drawn(&pipe);
    pipe
}

pub fn build_bushing(x: isize, y: isize) -> Rectangle<State> {
    let bushing = Rectangle::<State>::new();
    bushing
        .width(BUSHING_WIDTH)
        .height(BUSHING_HEIGHT)
        .fill(Some(' '))
        .background(Some(Color::ansi(BUSHING_BACKGROUND)))
        .handle("bushing")
        .x(x)
        .y(y);
    for (i, bg) in BUSHING_SEGMENT_BACKGROUNDS.iter().enumerate() {
        let seg = Rectangle::<State>::new();
        seg.width(1)
            .height(1)
            .fill(Some(' '))
            .background(Some(Color::ansi(*bg)))
            .x(x + i as isize)
            .y(y);
        seg.decoratable(false);
        bushing.add(seg);
    }
    hand_drawn(&bushing);
    bushing
}

pub fn build_pavement(x: isize, y: isize) -> Text<State> {
    let pavement = Text::<State>::default();
    pavement
        .text(&" ".repeat(PAVEMENT_WIDTH))
        .wrap_at(PAVEMENT_WIDTH)
        .background(Some(Color::ansi(34)))
        .handle("pavement")
        .x(x)
        .y(y);
    // Look manipulation: alternate background every block between 34 and 40.
    {
        let blocks = pavement.visual.look.blocks();
        if let Some(row) = blocks.first() {
            for (i, block) in row.iter().enumerate().take(PAVEMENT_WIDTH) {
                let bg = if i % 2 == 0 { 34 } else { 40 };
                block.decor.background.set(Some(Color::ansi(bg)));
            }
        }
    }
    hand_drawn(&pavement);
    debug_assert_eq!(pavement.visual.look.height(), PAVEMENT_HEIGHT);
    pavement
}

pub fn build_buildings(x: isize, y: isize) -> Text<State> {
    let buildings = Text::<State>::default();
    buildings
        .text(BUILDINGS_STR)
        .color(Some(Color::ansi(BUILDINGS_COLOR)))
        .handle("buildings")
        .x(x)
        .y(y);
    // Hand-drawn per-block color so the look carries 244 even when not decoratable.
    {
        let blocks = buildings.visual.look.blocks();
        for row in blocks.iter() {
            for block in row.iter() {
                block.decor.color.set(Some(Color::ansi(BUILDINGS_COLOR)));
            }
        }
    }
    hand_drawn(&buildings);
    buildings
}

pub fn build_flying_bird(x: isize, y: isize) -> Rectangle<State> {
    let bird = Rectangle::<State>::new();
    bird.width(FLYING_BIRD_WIDTH)
        .height(FLYING_BIRD_HEIGHT)
        .fill(Some(' '))
        .handle("flying_bird")
        .x(x)
        .y(y);
    hand_drawn(&bird);
    bird
}

pub fn build_dead_bird(x: isize, y: isize) -> Rectangle<State> {
    let bird = Rectangle::<State>::new();
    bird.width(DEAD_BIRD_WIDTH)
        .height(DEAD_BIRD_HEIGHT)
        .fill(Some(' '))
        .handle("dead_bird")
        .x(x)
        .y(y);
    hand_drawn(&bird);
    bird
}

pub fn build_bush(x: isize, y: isize, index: usize) -> Rectangle<State> {
    let bush = Rectangle::<State>::new();
    bush.width(2)
        .height(bush_height())
        .fill(Some(bush_fill(index)))
        .handle("bush")
        .x(x)
        .y(y);
    hand_drawn(&bush);
    bush
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_spec() {
        Globals::draw_enabled(false);
        let pipe = build_pipe(0, 0);
        assert_eq!(pipe.get_width(), PIPE_WIDTH);
        assert_eq!(pipe.get_height(), PIPE_HEIGHT);
        assert_eq!(pipe.get_background(), Some(Color::ansi(PIPE_BACKGROUND)));
        assert!(!pipe.get_decoratable());
        let segs = pipe.elements.cot::<Rectangle<State>>();
        assert_eq!(segs.len(), 6);
        for (i, seg) in segs.iter().enumerate() {
            assert_eq!(seg.get_width(), 1);
            assert_eq!(seg.get_height(), PIPE_HEIGHT);
            assert_eq!(
                seg.get_background(),
                Some(Color::ansi(PIPE_SEGMENT_BACKGROUNDS[i]))
            );
            assert!(!seg.get_decoratable());
        }
    }

    #[test]
    fn bushing_spec() {
        Globals::draw_enabled(false);
        let bushing = build_bushing(0, 0);
        assert_eq!(bushing.get_width(), BUSHING_WIDTH);
        assert_eq!(bushing.get_height(), BUSHING_HEIGHT);
        assert_eq!(
            bushing.get_background(),
            Some(Color::ansi(BUSHING_BACKGROUND))
        );
        assert!(!bushing.get_decoratable());
        let segs = bushing.elements.cot::<Rectangle<State>>();
        assert_eq!(segs.len(), 8);
        for (i, seg) in segs.iter().enumerate() {
            assert_eq!(seg.get_width(), 1);
            assert_eq!(seg.get_height(), 1);
            assert_eq!(
                seg.get_background(),
                Some(Color::ansi(BUSHING_SEGMENT_BACKGROUNDS[i]))
            );
            assert!(!seg.get_decoratable());
        }
    }

    #[test]
    fn pavement_spec() {
        Globals::draw_enabled(false);
        let pavement = build_pavement(0, 0);
        assert_eq!(pavement.visual.look.width(), PAVEMENT_WIDTH);
        assert_eq!(pavement.visual.look.height(), 1);
        assert_eq!(PAVEMENT_HEIGHT, 1);
        assert!(!pavement.get_decoratable());
        let blocks = pavement.visual.look.blocks();
        let row = &blocks[0];
        assert!(row.len() >= PAVEMENT_WIDTH);
        for (i, block) in row.iter().enumerate().take(PAVEMENT_WIDTH) {
            let expected = if i % 2 == 0 { 34 } else { 40 };
            assert_eq!(
                block.decor.background.get(),
                Some(Color::ansi(expected)),
                "pavement block {}",
                i
            );
        }
    }

    #[test]
    fn buildings_spec() {
        Globals::draw_enabled(false);
        let buildings = build_buildings(0, 0);
        assert_eq!(buildings.get_color(), Some(Color::ansi(BUILDINGS_COLOR)));
        assert!(!buildings.get_decoratable());
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
        for row in blocks.iter() {
            for block in row.iter() {
                if block.content.get().is_some() {
                    assert_eq!(
                        block.decor.color.get(),
                        Some(Color::ansi(BUILDINGS_COLOR))
                    );
                }
            }
        }
    }

    #[test]
    fn birds_spec() {
        Globals::draw_enabled(false);
        let flying = build_flying_bird(0, 0);
        assert_eq!(flying.get_width(), FLYING_BIRD_WIDTH);
        assert_eq!(flying.get_height(), FLYING_BIRD_HEIGHT);
        assert!(!flying.get_decoratable());
        let dead = build_dead_bird(0, 0);
        assert_eq!(dead.get_width(), DEAD_BIRD_WIDTH);
        assert_eq!(dead.get_height(), DEAD_BIRD_HEIGHT);
        assert!(!dead.get_decoratable());
    }

    #[test]
    fn bush_spec() {
        Globals::draw_enabled(false);
        for index in 0..12 {
            let bush = build_bush(0, 0, index);
            assert_eq!(bush.get_width(), 2);
            let h = bush.get_height();
            assert!((1..=2).contains(&h), "bush height {}", h);
            assert_eq!(bush.get_fill(), Some(bush_fill(index)));
            assert!(!bush.get_decoratable());
        }
        assert_eq!(bush_fill(1), '.');
        assert_eq!(bush_fill(3), '`');
        assert_eq!(bush_fill(0), '^');
    }
}
