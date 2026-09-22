use incredible::*;
use incredible_elements::Image;

use crate::ui::app::State;
use crate::ui::assets::decode_png;

/// Builds the kitty-mode fluffy title at two thirds of the given
/// title width. Unpositioned; the composer places it.
pub(crate) fn build_fluffy(title_w: usize) -> Image<State> {
    let fluffy = Image::<State>::new();
    fluffy.width(title_w * 2 / 3);
    fluffy.data(decode_png(include_bytes!("../../../../assets/fluffy.png")));
    fluffy.handle("fluffy_title");
    fluffy.showed(false);
    fluffy
}
