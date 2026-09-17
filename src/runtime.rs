use crate::{platform, ui::app};

pub fn run() -> incredible::tui::DeferredValue<app::State> {
    platform::init();

    app::build().run(app::State::default())
}
