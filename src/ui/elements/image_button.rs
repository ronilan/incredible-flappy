use incredible::*;
use incredible_elements::Image;
use incredible_macros_decl::element;

use crate::ui::assets::decode_png;

pub const IMAGE_BUTTON_HEIGHT_CELLS: usize = 2;

const BASIC_PNG: &[u8] = include_bytes!("../../../assets/basic.png");
const GRAZE_PNG: &[u8] = include_bytes!("../../../assets/graze.png");
const SHOOT_PNG: &[u8] = include_bytes!("../../../assets/shoot.png");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageButtonKind {
    Basic,
    Graze,
    Shoot,
}

impl ImageButtonKind {
    fn png(self) -> &'static [u8] {
        match self {
            ImageButtonKind::Basic => BASIC_PNG,
            ImageButtonKind::Graze => GRAZE_PNG,
            ImageButtonKind::Shoot => SHOOT_PNG,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageButtonOptions {
    pub kind: ImageButtonKind,
    pub height: usize,
}

impl Default for ImageButtonOptions {
    fn default() -> Self {
        Self {
            kind: ImageButtonKind::Basic,
            height: IMAGE_BUTTON_HEIGHT_CELLS,
        }
    }
}

element! {
  pub struct ImageButton<S> {
      options: ImageButtonOptions = ImageButtonOptions::default(),
  }
}

impl<S: Clone + PartialEq> ImageButton<S> {
    pub fn new(options: ImageButtonOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        // Height fixed, width derived from the image aspect ratio.
        let img = Image::<S>::new();
        img.height(el.options.height)
            .data(decode_png(el.options.kind.png()))
            .x(el.get_x())
            .y(el.get_y())
            .capture(Capture::all())
            .fused(true);
        let (w, h) = (img.visual.look.width(), img.visual.look.height());
        el.look(Look::from((w, h, ' '))).handle("image_button");
        el.add(img);

        el
    }
}

impl<S: Clone + PartialEq> Default for ImageButton<S> {
    fn default() -> Self {
        Self::new(ImageButtonOptions::default())
    }
}
