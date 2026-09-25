use incredible::{Platform, PlatformOutput};

use crate::{
    platform,
    settings,
    ui::{app, state},
};

pub fn run() -> incredible::tui::DeferredValue<state::State> {
    platform::init();

    let mut state = state::State::default();
    settings::apply_to_state(&settings::load(), &mut state);
    let app = app::build();
    // Kitty needs images; force it off where unsupported. The file
    // keeps its value for capable terminals.
    if !Platform::output_provider().images() {
        state.kitty = false;
    }
    app.run(state)
}
