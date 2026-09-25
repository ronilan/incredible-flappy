use incredible::*;
use incredible_helpers_styling::*;

use crate::ui::state::{Phase, SelectedGame, State};
use crate::ui::elements::{ImageButton, ImageButtonKind, ImageButtonOptions};

fn image_button(kind: ImageButtonKind, handle: &str, game: SelectedGame) -> ImageButton<State> {
    let btn = ImageButton::<State>::new(ImageButtonOptions {
        kind,
        ..Default::default()
    });
    btn.handle(handle)
        .showed(false)
        .pointer(Some(PointerShape::Pointer))
        .focused(false);
    // Hovering steers splash focus; on_state paints the hint from it.
    btn.on_change(move |_el, state: &mut State, event| {
        if state.phase == Phase::Splash && event.changes.contains(&Change::Hovered(true)) {
            state.focus = Some(game);
        }
    });
    btn
}

/// Builds the kitty-mode image twins of the game buttons.
/// Unpositioned; the composer lays out the row.
pub(crate) fn build_image_buttons() -> [ImageButton<State>; 3] {
    [
        image_button(ImageButtonKind::Basic, "game_basic_image", SelectedGame::Classic),
        image_button(ImageButtonKind::Graze, "game_graze_image", SelectedGame::Busy),
        image_button(ImageButtonKind::Shoot, "game_shoot_image", SelectedGame::Invaders),
    ]
}
