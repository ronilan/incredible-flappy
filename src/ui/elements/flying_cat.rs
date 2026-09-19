use incredible::*;
use incredible_elements::{Image, ImageData};
use incredible_macros_decl::element;

pub const FLYING_CAT_WIDTH_CELLS: usize = 3;
const FLYING_CAT_PNG: &[u8] = include_bytes!("../../../assets/FlyingCat.PNG");

#[derive(Clone, Debug)]
pub struct FlyingCatOptions {
    pub width_cells: usize,
}

impl Default for FlyingCatOptions {
    fn default() -> Self {
        Self {
            width_cells: FLYING_CAT_WIDTH_CELLS,
        }
    }
}

element! {
  pub struct FlyingCat<S> {
      options: FlyingCatOptions = FlyingCatOptions::default(),
  }
}

impl<S: Clone + PartialEq> FlyingCat<S> {
    pub fn new(options: FlyingCatOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let img = Image::<S>::new();
        img.width(el.options.width_cells);
        img.data(decode_png(FLYING_CAT_PNG));
        img.handle("flying_cat_image");

        el.look(Look::from((
            img.visual.look.width(),
            img.visual.look.height(),
            ' ',
        )))
        .handle("flying_cat");
        el.add(img);

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for FlyingCat<S> {
    fn default() -> Self {
        Self::new(FlyingCatOptions::default())
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
