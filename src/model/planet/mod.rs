use super::point::Point;
use super::circle::Circle;

#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
    pub mass: Circle,
    pub zone: Circle,
    pub ordinal: usize,
}

impl Planet {
    pub fn new(location: Point, radius: f64, zone_height: f64, ordinal: usize) -> Self {
        let mass = Circle::new(location, radius);
        let zone = Circle::new(location, Self::zone_height(radius, zone_height));

        Planet { mass, zone, ordinal }
    }

    pub fn zone_height(radius: f64, height: f64) -> f64 {
        radius * (height + 1.0)
    }
}

#[cfg(test)]
mod test;
