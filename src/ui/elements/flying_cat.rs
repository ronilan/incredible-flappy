use incredible::*;
use incredible_elements::{Image, ImageData};
use incredible_macros_decl::element;

pub const FLYING_CAT_WIDTH_CELLS: usize = 10;
pub const FLYING_CAT_HEIGHT_CELLS: usize = 4;
const FLYING_CAT_UP_PNG: &[u8] = include_bytes!("../../../assets/flying_cat_up.png");
const FLYING_CAT_DOWN_PNG: &[u8] = include_bytes!("../../../assets/flying_cat_down.png");

#[derive(Clone, Debug)]
pub struct FlyingCatOptions {
    pub width_cells: usize,
    pub height_cells: usize,
}

impl Default for FlyingCatOptions {
    fn default() -> Self {
        Self {
            width_cells: FLYING_CAT_WIDTH_CELLS,
            height_cells: FLYING_CAT_HEIGHT_CELLS,
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

        let up = Image::<S>::new();
        up.width(el.options.width_cells);
        up.height(el.options.height_cells);
        up.data(decode_png(FLYING_CAT_UP_PNG));
        up.handle("flying_cat_up");

        let down = Image::<S>::new();
        down.width(el.options.width_cells);
        down.height(el.options.height_cells);
        down.data(decode_png(FLYING_CAT_DOWN_PNG));
        down.handle("flying_cat_down");
        down.showed(false);

        el.look(Look::from((
            el.options.width_cells,
            el.options.height_cells,
            ' ',
        )))
        .handle("flying_cat");
        el.add(up);
        el.add(down);

        el.decorate();

        el
    }

    /// Up image while rising, down image while falling.
    pub fn set_rising(&self, rising: bool) -> &Self {
        for img in self.elements.cot::<Image<S>>() {
            let handle = img.get_handle();
            if handle == "flying_cat_up" {
                img.showed(rising);
            } else if handle == "flying_cat_down" {
                img.showed(!rising);
            }
        }
        self
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
