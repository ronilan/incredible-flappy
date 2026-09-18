use incredible::*;
use incredible_elements::Rectangle;

use super::ui;
use crate::ui::app::State;

/// Builds the whole splash screen.
pub(crate) fn build() -> Rectangle<State> {
    let splash = ui::screen::build();
    splash.add(ui::title::build());
    splash.add(ui::hint::build());

    splash
}
