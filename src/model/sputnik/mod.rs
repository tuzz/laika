use super::direction::Direction;
use super::point::Point;
use super::thruster::Thruster;
use super::triangle::Triangle;

#[derive(Clone, Debug, PartialEq)]
pub struct Sputnik {
    hull: Triangle,
    location: Point,
    travelling: Direction,
    heading: Direction,
    thruster: Option<Thruster>,
}

impl Sputnik {
    pub fn new(heading: Direction, location: Point, area: f64) -> Self {
        let hull = Self::build_hull(area);

        Self {
            hull,
            heading,
            travelling: heading,
            location,
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

    fn build_hull(area: f64) -> Triangle {
        let x = (area / 2.0).sqrt();
        let y = x / 1.5;

        let center = Point::new(0.5, 0.5);

        let front = center + Point::new(0.0, y * 2.0);
        let back_left = center + Point::new(-x, -y);
        let back_right = center + Point::new(x, -y);

        Triangle::new(front, back_left, back_right)
    }
}

#[cfg(test)]
mod test;
