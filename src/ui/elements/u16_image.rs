use std::cell::Cell;

use incredible::*;
use incredible_elements::Image;
use incredible_macros_decl::element;

use crate::ui::assets::decode_png;

pub const DIGIT_WIDTH_CELLS: usize = 3;
pub const DIGIT_HEIGHT_CELLS: usize = 3;

const DIGIT_PNGS: [&[u8]; 10] = [
    include_bytes!("../../../assets/0.png"),
    include_bytes!("../../../assets/1.png"),
    include_bytes!("../../../assets/2.png"),
    include_bytes!("../../../assets/3.png"),
    include_bytes!("../../../assets/4.png"),
    include_bytes!("../../../assets/5.png"),
    include_bytes!("../../../assets/6.png"),
    include_bytes!("../../../assets/7.png"),
    include_bytes!("../../../assets/8.png"),
    include_bytes!("../../../assets/9.png"),
];

#[derive(Clone, Debug)]
pub struct U16ImageOptions {
    pub digit_width: usize,
    pub digit_height: usize,
}

impl Default for U16ImageOptions {
    fn default() -> Self {
        Self {
            digit_width: DIGIT_WIDTH_CELLS,
            digit_height: DIGIT_HEIGHT_CELLS,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct U16ImageState {
    pub value: Cell<u16>,
}

element! {
  pub struct U16Image<S> {
      options: U16ImageOptions = U16ImageOptions::default(),
      internal_state: U16ImageState = U16ImageState::default(),
  }
}

impl<S: Clone + PartialEq> U16Image<S> {
    pub fn new() -> Self {
        let el = Self::blank();

        el.internal_on_change(|el, _, _event| {
            el.draw();
        });
        el.refresh();

        el
    }

    /// Rebuilds the digit images for the current value.
    pub fn refresh(&self) {
        let _ = self.elements.saot::<Image<S>>();

        let mut cursor: usize = 0;
        for ch in self.internal_state.value.get().to_string().chars() {
            // Narrow 1 renders one cell slimmer than the standard width.
            let w = if ch == '1' {
                self.options.digit_width.saturating_sub(1)
            } else {
                self.options.digit_width
            };
            let img = Image::<S>::new();
            img.width(w)
                .height(self.options.digit_height)
                .data(decode_png(DIGIT_PNGS[ch as usize - '0' as usize]))
                .x(cursor as isize)
                .y(0)
                .capture(Capture::all())
                .fused(true);
            cursor += img.visual.look.width();
            self.add(img);
        }
        self.look(Look::from((cursor, self.options.digit_height, ' ')));
        self.draw();
    }
}

impl<S: Clone + PartialEq> U16Image<S> {
    /// Sets the displayed value.
    pub fn value(&self, value: u16) -> &Self {
        self.internal_state.value.set(value);
        self.refresh();
        self
    }

    /// Gets the displayed value.
    pub fn get_value(&self) -> u16 {
        self.internal_state.value.get()
    }
}

impl<S: Clone + PartialEq> Default for U16Image<S> {
    fn default() -> Self {
        Self::new()
    }
}
