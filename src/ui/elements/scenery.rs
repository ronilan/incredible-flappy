use incredible::*;
use incredible_macros_decl::element;

use super::bushes::{BUSHES_WIDTH, Bushes, BushesOptions};
use super::buildings::Buildings;
use super::floor::{Floor, FloorOptions};
use super::pavement::{PAVEMENT_WIDTH, Pavement, PavementOptions};

pub const SCENERY_WIDTH: usize = 40;
pub const SCENERY_HEIGHT: usize = 9;

#[derive(Clone, Debug)]
pub struct SceneryOptions {
    pub pavement_background: u8,
    pub floor_background: u8,
    pub bush_background: u8,
    pub bush_color: u8,
}

impl Default for SceneryOptions {
    fn default() -> Self {
        Self {
            pavement_background: crate::ui::theme::PAVEMENT_BASE,
            floor_background: crate::ui::theme::FLOOR_BACKGROUND,
            bush_background: crate::ui::theme::BUSH_BACKGROUND,
            bush_color: crate::ui::theme::BUSH_COLOR,
        }
    }
}

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
        let floor = Floor::<S>::new(FloorOptions {
            background: el.options.floor_background,
        });
        floor.x(0).y(6);
        el.add(floor);

        let pavement = Pavement::<S>::new(PavementOptions {
            background: el.options.pavement_background,
            width: PAVEMENT_WIDTH,
        });
        pavement.x(0).y(5);
        el.add(pavement);

        let buildings = Buildings::<S>::default();
        buildings.x(2).y(0);
        el.add(buildings);
        
        let bushes = Bushes::<S>::new(BushesOptions {
            width: BUSHES_WIDTH,
            bush_background: el.options.bush_background,
            bush_color: el.options.bush_color,
        });
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
