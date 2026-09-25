use crate::ui::state::State;
use crate::ui::elements::Scenery;

/// Builds the ground tile. Unpositioned; the composer places it.
pub(crate) fn build_scenery() -> Scenery<State> {
    Scenery::<State>::default()
}
