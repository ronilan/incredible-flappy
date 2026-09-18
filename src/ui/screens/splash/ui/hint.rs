use incredible::*;
use incredible_elements::Text;

use crate::ui::app::State;

/// Builds the splash screen hint.
pub(crate) fn build() -> Text<State> {
    let hint = Text::<State>::default();
    hint.text("Enter -> Game-Ready | Esc -> Splash")
        .handle("splash_hint")
        .x(2)
        .y(4);

    hint
}
