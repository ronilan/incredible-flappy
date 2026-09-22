use incredible::*;

use crate::ui::app::State;
use crate::ui::elements::{ImageButton, ImageButtonKind, ImageButtonOptions};

fn image_button(kind: ImageButtonKind, handle: &str) -> ImageButton<State> {
    let btn = ImageButton::<State>::new(ImageButtonOptions {
        kind,
        ..Default::default()
    });
    btn.handle(handle);
    btn.showed(false);
    btn
}

/// Builds the kitty-mode image twins of the game buttons.
/// Unpositioned; the composer lays out the row.
pub(crate) fn build_image_buttons() -> [ImageButton<State>; 3] {
    [
        image_button(ImageButtonKind::Basic, "game_basic_image"),
        image_button(ImageButtonKind::Graze, "game_graze_image"),
        image_button(ImageButtonKind::Shoot, "game_shoot_image"),
    ]
}
