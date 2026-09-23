use incredible::{Platform, PlatformOutput};

use crate::{
    platform,
    settings,
    ui::app,
};

pub fn run() -> incredible::tui::DeferredValue<app::State> {
    platform::init();

    let mut state = app::State::default();
    settings::apply_to_state(&settings::load(), &mut state);
    let app = app::build();
    // Kitty needs images; force it off where unsupported. The file
    // keeps its value for capable terminals.
    if !Platform::output_provider().images() {
        state.kitty = false;
    }
    app.run(state)
}
