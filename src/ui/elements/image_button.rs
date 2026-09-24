use incredible::*;
use incredible_elements::Image;
use incredible_macros_decl::element;

use crate::ui::assets::decode_png;

pub const IMAGE_BUTTON_HEIGHT_CELLS: usize = 2;

const BASIC_PNG: &[u8] = include_bytes!("../../../assets/basic.png");
const GRAZE_PNG: &[u8] = include_bytes!("../../../assets/graze.png");
const SHOOT_PNG: &[u8] = include_bytes!("../../../assets/shoot.png");
const BASIC_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/basic_hovered.png");
const GRAZE_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/graze_hovered.png");
const SHOOT_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/shoot_hovered.png");
const BASIC_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/basic_activated.png");
const GRAZE_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/graze_activated.png");
const SHOOT_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/shoot_activated.png");
const BASIC_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/basic_focused.png");
const GRAZE_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/graze_focused.png");
const SHOOT_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/shoot_focused.png");
const BACK_PNG: &[u8] = include_bytes!("../../../assets/back.png");
const BACK_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/back_hovered.png");
const BACK_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/back_focused.png");
const BACK_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/back_activated.png");
const PLAY_PNG: &[u8] = include_bytes!("../../../assets/play.png");
const PLAY_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/play_hovered.png");
const PLAY_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/play_focused.png");
const PLAY_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/play_activated.png");
const PAUSE_PNG: &[u8] = include_bytes!("../../../assets/pause.png");
const PAUSE_HOVERED_PNG: &[u8] = include_bytes!("../../../assets/pause_hovered.png");
const PAUSE_FOCUSED_PNG: &[u8] = include_bytes!("../../../assets/pause_focused.png");
const PAUSE_ACTIVATED_PNG: &[u8] = include_bytes!("../../../assets/pause_activated.png");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageButtonKind {
    Basic,
    Graze,
    Shoot,
    Back,
    Play,
    Pause,
}

impl ImageButtonKind {
    fn png(self) -> &'static [u8] {
        match self {
            ImageButtonKind::Basic => BASIC_PNG,
            ImageButtonKind::Graze => GRAZE_PNG,
            ImageButtonKind::Shoot => SHOOT_PNG,
            ImageButtonKind::Back => BACK_PNG,
            ImageButtonKind::Pause => PAUSE_PNG,
            ImageButtonKind::Play => PLAY_PNG,
        }
    }

    fn hovered_png(self) -> &'static [u8] {
        match self {
            ImageButtonKind::Basic => BASIC_HOVERED_PNG,
            ImageButtonKind::Graze => GRAZE_HOVERED_PNG,
            ImageButtonKind::Shoot => SHOOT_HOVERED_PNG,
            ImageButtonKind::Back => BACK_HOVERED_PNG,
            ImageButtonKind::Pause => PAUSE_HOVERED_PNG,
            ImageButtonKind::Play => PLAY_HOVERED_PNG,
        }
    }

    fn activated_png(self) -> &'static [u8] {
        match self {
            ImageButtonKind::Basic => BASIC_ACTIVATED_PNG,
            ImageButtonKind::Graze => GRAZE_ACTIVATED_PNG,
            ImageButtonKind::Shoot => SHOOT_ACTIVATED_PNG,
            ImageButtonKind::Back => BACK_ACTIVATED_PNG,
            ImageButtonKind::Pause => PAUSE_ACTIVATED_PNG,
            ImageButtonKind::Play => PLAY_ACTIVATED_PNG,
        }
    }

    fn focused_png(self) -> &'static [u8] {
        match self {
            ImageButtonKind::Basic => BASIC_FOCUSED_PNG,
            ImageButtonKind::Graze => GRAZE_FOCUSED_PNG,
            ImageButtonKind::Shoot => SHOOT_FOCUSED_PNG,
            ImageButtonKind::Back => BACK_FOCUSED_PNG,
            ImageButtonKind::Pause => PAUSE_FOCUSED_PNG,
            ImageButtonKind::Play => PLAY_FOCUSED_PNG,
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

impl<S: Clone + PartialEq + 'static> ImageButton<S> {
    pub fn new(options: ImageButtonOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        // All four states preloaded in variant order: normal,
        // focused, hovered, activated. Visibility rotates between them.
        let pngs = [
            el.options.kind.png(),
            el.options.kind.focused_png(),
            el.options.kind.hovered_png(),
            el.options.kind.activated_png(),
        ];
        let mut w = 0;
        let mut h = 0;
        for (i, png) in pngs.into_iter().enumerate() {
            let img = Image::<S>::new();
            img.height(el.options.height)
                .data(decode_png(png))
                .x(el.get_x())
                .y(el.get_y())
                .capture(Capture::all())
                .fused(true);
            if i == 0 {
                w = img.visual.look.width();
                h = img.visual.look.height();
            }
            el.add(img);
        }
        // Base layer on; decorate owns the rest from here.
        for (i, img) in el.elements.cot::<Image<S>>().into_iter().enumerate() {
            img.showed(i == 0);
        }
        el.look(Look::from((w, h, ' '))).handle("image_button");

        el.renderer.decorate.set(|el| {
            el.image_button_decorate();
        });

        el
    }

    /// Rotates visibility between the four preloaded states, following
    /// the framework status precedence (focused < hovered < activated).
    /// Authoritative every pass; only writes on actual flips.
    fn image_button_decorate(&self) {
        let status = self.status();
        let variant = if status.activated.get() {
            3
        } else if status.hovered.get() {
            2
        } else if status.focused.get() {
            1
        } else {
            0
        };
        for (i, img) in self.elements.cot::<Image<S>>().into_iter().enumerate() {
            if img.status().showed.get() != (i == variant as usize) {
                img.showed(i == variant as usize);
            }
        }
    }
}

impl<S: Clone + PartialEq + 'static> Default for ImageButton<S> {
    fn default() -> Self {
        Self::new(ImageButtonOptions::default())
    }
}
