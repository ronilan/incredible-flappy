use incredible::*;
use incredible_elements::{Image, ImageData};
use incredible_macros_decl::element;

pub const DEAD_CAT_WIDTH_CELLS: usize = 10;
pub const DEAD_CAT_HEIGHT_CELLS: usize = 4;
const DEAD_CAT_PNG: &[u8] = include_bytes!("../../../assets/dead_cat.png");

#[derive(Clone, Debug)]
pub struct DeadCatOptions {
    pub width_cells: usize,
    pub height_cells: usize,
}

impl Default for DeadCatOptions {
    fn default() -> Self {
        Self {
            width_cells: DEAD_CAT_WIDTH_CELLS,
            height_cells: DEAD_CAT_HEIGHT_CELLS,
        }
    }
}

element! {
  pub struct DeadCat<S> {
      options: DeadCatOptions = DeadCatOptions::default(),
  }
}

impl<S: Clone + PartialEq> DeadCat<S> {
    pub fn new(options: DeadCatOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let img = Image::<S>::new();
        img.width(el.options.width_cells);
        img.height(el.options.height_cells);
        img.data(decode_png(DEAD_CAT_PNG));
        img.handle("dead_cat_image");

        el.look(Look::from((
            img.visual.look.width(),
            img.visual.look.height(),
            '.',
        )))
        .handle("dead_cat");
        el.add(img);


        el
    }
}

impl<S: Clone + PartialEq> Default for DeadCat<S> {
    fn default() -> Self {
        Self::new(DeadCatOptions::default())
    }
}

fn decode_png(bytes: &[u8]) -> ImageData {
    let img = image::load_from_memory(bytes).expect("cat asset decodes");
    let rgba = img.to_rgba8();
    let (width_px, height_px) = (rgba.width(), rgba.height());
    ImageData {
        bytes: rgba.into_raw(),
        width_px,
        height_px,
    }
}
