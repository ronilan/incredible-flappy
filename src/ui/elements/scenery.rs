use incredible::*;
use incredible_macros_decl::element;

use super::bushes::Bushes;
use super::floor::Floor;
use super::pavement::Pavement;

pub const SCENERY_WIDTH: usize = 80;
pub const SCENERY_HEIGHT: usize = 5;

#[derive(Clone, Debug, Default)]
pub struct SceneryOptions;

element! {
  pub struct Scenery<S> {
      options: SceneryOptions = SceneryOptions::default(),
  }
}

impl<S: Clone + PartialEq> Scenery<S> {
    pub fn new(options: SceneryOptions) -> Self {
        let mut el = Self::blank();
        el.options = options;

        el.look(Look::from((SCENERY_WIDTH, SCENERY_HEIGHT, ' ')))
            .handle("scenery");

        // Bottom to top: Floor, Pavement, Bushes.
        let floor = Floor::<S>::default();
        floor.x(40).y(2);
        el.add(floor);

        let pavement = Pavement::<S>::default();
        pavement.x(0).y(4);
        el.add(pavement);

        let bushes = Bushes::<S>::default();
        bushes.x(40).y(2);
        el.add(bushes);

        el.decorate();

        el
    }
}

impl<S: Clone + PartialEq> Default for Scenery<S> {
    fn default() -> Self {
        Self::new(SceneryOptions::default())
    }
}
