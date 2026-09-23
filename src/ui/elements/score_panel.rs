use std::cell::Cell;

use incredible::*;
use incredible_elements::{Frame, FrameKind, FrameStyle, Image, Label, LabelOptions};
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::assets::decode_png;
use crate::ui::theme;

const SCORE_FRAME_PNG: &[u8] = include_bytes!("../../../assets/score_frame.png");

pub const SCORE_PANEL_WIDTH: usize = 16;
pub const SCORE_PANEL_HEIGHT: usize = 5;
/// Resting row, game-relative, for the slid-in score panel.
pub const SCORE_PANEL_REST_Y: isize = 8;
/// Parked row, game-relative, below the 24-row screen for the slide-in.
pub const SCORE_PANEL_PARK_Y: isize = 24;

const MEDALS: [&str; 3] = ["🥇", "🥈", "🥉"];

#[derive(Clone, Debug, Default)]
pub struct ScorePanelState {
    pub score: Cell<u32>,
    pub best: Cell<u32>,
    pub medal: Cell<Option<usize>>,
}

element! {
  pub struct ScorePanel<S> {
      internal_state: ScorePanelState = ScorePanelState::default(),
  }
}

impl<S: Clone + PartialEq> ScorePanel<S> {
    pub fn new() -> Self {
        let el = Self::blank();

        el.look(Look::from((
            SCORE_PANEL_WIDTH,
            SCORE_PANEL_HEIGHT,
            ' ',
        )))
        .background(Some(theme::score_panel_background()))
        .handle("score_panel");

        let frame_style = FrameStyle::default();
        frame_style.base.kind.set(Some(FrameKind::Double));
        let frame = Frame::<S>::new();
        frame
            .width(SCORE_PANEL_WIDTH)
            .height(SCORE_PANEL_HEIGHT)
            .frame_style(frame_style)
            .color(Some(Color::Ansi(theme::SCORE_PANEL_TEXT)))
            .x(el.get_x())
            .y(el.get_y())
            .capture(Capture::all())
            .fused(true);
        frame.handle("score_panel_frame");
        el.add(frame);

        // Kitty-mode image frame over the same box.
        let image = Image::<S>::new();
        image
            .width(SCORE_PANEL_WIDTH)
            .height(SCORE_PANEL_HEIGHT)
            .data(decode_png(SCORE_FRAME_PNG))
            .x(el.get_x())
            .y(el.get_y())
            .capture(Capture::all())
            .fused(true);
        image.handle("score_panel_image");
        image.showed(false);
        el.add(image);

        el.refresh();

        el
    }

    /// Rebuilds the score, best and medal rows.
    pub fn refresh(&self) {
        let _ = self.elements.saot::<Label<S>>();
        let rows = [
            format!("Score: {}", self.internal_state.score.get()),
            format!("Best: {}", self.internal_state.best.get()),
            match self.internal_state.medal.get() {
                Some(rank) => format!("Medal: {}", MEDALS[rank.min(2)]),
                None => "Medal: 💩".to_string(),
            },
        ];
        for (i, row) in rows.iter().enumerate() {
            let label = Label::<S>::new(LabelOptions::default());
            let (r, g, b) = theme::SCORE_PANEL_LABEL_BACKGROUND;
            // Must set background when above image as there is nothing but
            // the terminal background to "transparent into".
            label
                .text(row.as_str())
                .color(Some(Color::Ansi(theme::SCORE_PANEL_TEXT)))
                .background(Some(Color::Rgba(Rgba::new(r, g, b, 255))))
                .x(2)
                .y(1 + i as isize)
                .capture(Capture::all().set_style(false))
                .fused(true);
            self.add(label);
        }
        self.draw();
    }
}

impl<S: Clone + PartialEq> ScorePanel<S> {
    /// Sets the run result: this score, the recorded best, and the
    /// medal rank (0 gold, 1 silver, 2 bronze), if any.
    pub fn set_result(&self, score: u32, best: u32, medal: Option<usize>) -> &Self {
        self.internal_state.score.set(score);
        self.internal_state.best.set(best);
        self.internal_state.medal.set(medal);
        self.refresh();
        self
    }
}

impl<S: Clone + PartialEq> Default for ScorePanel<S> {
    fn default() -> Self {
        Self::new()
    }
}
