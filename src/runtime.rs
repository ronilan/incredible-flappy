use crate::{
    platform,
    ui::{app, settings},
};

pub fn run() -> incredible::tui::DeferredValue<app::State> {
    platform::init();

    let mut state = app::State::default();
    settings::apply_to_state(&settings::load(), &mut state);
    app::build().run(state)
}
