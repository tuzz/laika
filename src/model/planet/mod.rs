use ::util::{Circle, Point};

use super::composition::Composition;

#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
    pub mass: Circle,
    pub zone: Circle,
    pub composition: Composition,
    pub ordinal: usize,
}

pub const NUMBER_OF_ELEMENTS: usize = 100;

impl Planet {
    pub fn new(location: Point, radius: f32, zone_height: f32, ordinal: usize) -> Self {
        let mass = Circle::new(location, radius);
        let zone = Circle::new(location, Self::zone_height(radius, zone_height));
        let composition = Composition::random(NUMBER_OF_ELEMENTS);

        Planet { mass, zone, composition, ordinal }
    }

    pub fn zone_height(radius: f32, height: f32) -> f32 {
        radius * (height + 1.0)
    }
}

#[cfg(test)]
mod test;
