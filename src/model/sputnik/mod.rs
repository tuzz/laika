use super::direction::Direction;
use super::point::Point;
use super::thruster::Thruster;
use super::triangle::Triangle;

#[derive(Clone, Debug, PartialEq)]
pub struct Sputnik {
    pub hull: Triangle,
    pub travelling: Direction,
    pub heading: Direction,
    pub thruster: Option<Thruster>,
}

impl Sputnik {
    pub fn new(heading: Direction, location: Point, area: f64) -> Self {
        let hull = Self::build_hull(location, area);

        Self {
            hull,
            heading,
            travelling: heading,
            thruster: None,
        }
    }

    pub fn left_thruster(&self) -> Self {
        Self { thruster: Some(Thruster::Left), .. *self }
    }

    pub fn right_thruster(&self) -> Self {
        Self { thruster: Some(Thruster::Right), .. *self }
    }

    pub fn no_thruster(&self) -> Self {
        Self { thruster: None, .. *self }
    }

    pub fn hull_radius(area: f64) -> f64 {
        let arbitrary_point = Point::new(0.5, 0.5);
        let hull = Self::build_hull(arbitrary_point, area);

        hull.bounding_circle().radius
    }

    fn build_hull(location: Point, area: f64) -> Triangle {
        let x = (area / 2.0).sqrt();
        let y = x / 1.5;

        let front = location + Point::new(0.0, y * 2.0);
        let back_left = location + Point::new(-x, -y);
        let back_right = location + Point::new(x, -y);

        Triangle::new(front, back_left, back_right)
    }
}

#[cfg(test)]
mod test;
