use std::cell::RefCell;

use incredible::*;
use incredible_elements::{FrameKind, FrameStyle, FramedText};
use incredible_helpers_styling::*;
use incredible_macros_decl::element;

use crate::ui::theme;

/// Cell width for one boxed letter; spaces advance the same pitch.
pub const BOXED_CELL_WIDTH: usize = 3;

/// Boxed letters: framed text in the score theme (black box,
/// white text), sized to its content. For titles and labels.
#[derive(Clone, Debug)]
pub struct BoxedTextOptions {
    pub background: u8,
    pub foreground: u8,
}

impl Default for BoxedTextOptions {
    fn default() -> Self {
        Self {
            background: theme::SCORE_BACKGROUND,
            foreground: theme::SCORE_TEXT,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BoxedTextState {
    pub text: RefCell<String>,
}

element! {
  pub struct BoxedText<S> {
      options: BoxedTextOptions = BoxedTextOptions::default(),
      internal_state: BoxedTextState = BoxedTextState::default(),
  }
}

impl<S: Clone + PartialEq> BoxedText<S> {
    pub fn new(options: BoxedTextOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;
        el.refresh();
        el.handle("boxed_text");
        el
    }

    /// Rebuilds one double-framed cell per letter. Spaces advance
    /// the pitch without a box.
    pub fn refresh(&self) {
        let _ = self.elements.saot::<FramedText<S>>();

        let frame_style = FrameStyle::default();
        frame_style.base.kind.set(Some(FrameKind::Double));

        let mut cursor: usize = 0;
        let mut height: usize = 0;
        for ch in self.internal_state.text.borrow().chars() {
            if ch == ' ' {
                cursor += BOXED_CELL_WIDTH;
                continue;
            }
            let cell = FramedText::<S>::default();
            cell.text(ch.to_string().as_str())
                .width(BOXED_CELL_WIDTH)
                .frame_style(frame_style.clone())
                .background(Some(Color::Ansi(self.options.background)))
                .color(Some(Color::Ansi(self.options.foreground)))
                .x(cursor as isize)
                .y(0)
                .capture(Capture::all())
                .fused(true);
            cursor += cell.visual.look.width();
            height = height.max(cell.visual.look.height());
            self.add(cell);
        }
        self.look(Look::from((cursor, height, ' ')));
        self.draw();
    }
}

impl<S: Clone + PartialEq> BoxedText<S> {
    /// Sets the displayed text.
    pub fn text(&self, raw: &str) -> &Self {
        self.internal_state.text.replace(raw.to_string());
        self.refresh();
        self
    }
}

impl<S: Clone + PartialEq> Default for BoxedText<S> {
    fn default() -> Self {
        Self::new(BoxedTextOptions::default())
    }
}
