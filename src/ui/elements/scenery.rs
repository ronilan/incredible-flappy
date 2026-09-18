use incredible::*;
use incredible_macros_decl::element;

use super::bushes::Bushes;
use super::buildings::Buildings;
use super::floor::Floor;
use super::pavement::Pavement;

pub const SCENERY_WIDTH: usize = 40;
pub const SCENERY_HEIGHT: usize = 9;

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
        floor.x(0).y(6);
        el.add(floor);

        let pavement = Pavement::<S>::default();
        pavement.x(0).y(5);
        el.add(pavement);

        let buildings = Buildings::<S>::default();
        buildings.x(2).y(0);
        el.add(buildings);
        
        let bushes = Bushes::<S>::default();
        bushes.x(0).y(3);
        el.add(bushes);

        

        el
    }
}

impl<S: Clone + PartialEq> Default for Scenery<S> {
    fn default() -> Self {
        Self::new(SceneryOptions::default())
    }
}
