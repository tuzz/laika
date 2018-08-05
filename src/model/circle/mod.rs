use super::point::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn contains(&self, point: Point) -> bool {
        self.center.distance(point) < self.radius
    }

    pub fn intersects(&self, other: Self) -> bool {
        self.center.distance(other.center) < self.radius + other.radius
    }
}

#[cfg(test)]
mod test;
