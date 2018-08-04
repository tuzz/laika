use super::point::Point;
use super::circle::Circle;

struct Planet {
    mass: Circle,
    ordinal: usize,
}

impl Planet {
    fn new(location: Point, radius: f64, ordinal: usize) -> Self {
        let mass = Circle::new(location, radius);

        Planet { mass, ordinal }
    }

    fn orbital_zone(&self) -> Circle {
        Circle::new(self.mass.center, self.mass.radius * 3.0)
    }
}

#[cfg(test)]
mod test;
