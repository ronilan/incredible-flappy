use incredible::*;
use incredible_elements::Text;

use crate::ui::app::State;

/// Builds the splash screen title.
pub(crate) fn build() -> Text<State> {
    let title = Text::<State>::default();
    title.text("SPLASH").handle("splash_title").x(2).y(2);

    title
}
