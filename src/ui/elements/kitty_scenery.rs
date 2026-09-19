use incredible::*;
use incredible_elements::{Image, ImageData};
use incredible_macros_decl::element;

pub const KITTY_SCENERY_WIDTH_CELLS: usize = 40;
const KITTY_SCENERY_PNG: &[u8] = include_bytes!("../../../assets/sceneray.png");

#[derive(Clone, Debug)]
pub struct KittySceneryOptions {
    pub width_cells: usize,
}

impl Default for KittySceneryOptions {
    fn default() -> Self {
        Self {
            width_cells: KITTY_SCENERY_WIDTH_CELLS,
        }
    }
}

element! {
  pub struct KittyScenery<S> {
      options: KittySceneryOptions = KittySceneryOptions::default(),
  }
}

impl<S: Clone + PartialEq> KittyScenery<S> {
    pub fn new(options: KittySceneryOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        let img = Image::<S>::new();
        img.width(el.options.width_cells);
        img.data(decode_png(KITTY_SCENERY_PNG));
        img.handle("kitty_scenery_image");

        el.look(Look::from((
            img.visual.look.width(),
            img.visual.look.height(),
            ' ',
        )))
        .handle("kitty_scenery");
        el.add(img);

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for KittyScenery<S> {
    fn default() -> Self {
        Self::new(KittySceneryOptions::default())
    }
}

fn decode_png(bytes: &[u8]) -> ImageData {
    let img = image::load_from_memory(bytes).expect("scenery asset decodes");
    let rgba = img.to_rgba8();
    let (width_px, height_px) = (rgba.width(), rgba.height());
    ImageData {
        bytes: rgba.into_raw(),
        width_px,
        height_px,
    }
}
