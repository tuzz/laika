use super::direction::Direction;
use super::point::Point;
use super::thruster::Thruster;

pub struct Sputnik {
    location: Point,
    travelling: Direction,
    heading: Direction,
    thruster: Option<Thruster>,
}

impl Sputnik {
    pub fn new(heading: Direction, location: Point) -> Self {
        Self {
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
}

#[cfg(test)]
mod test;
