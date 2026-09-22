use crate::ui::app::State;
use crate::ui::elements::Scenery;

/// Builds the two ground tiles. Unpositioned; the composer places them.
pub(crate) fn build_scenery() -> [Scenery<State>; 2] {
    [Scenery::<State>::default(), Scenery::<State>::default()]
}
