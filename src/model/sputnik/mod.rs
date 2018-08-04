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
        Sputnik {
            heading,
            travelling: heading,
            location,
            thruster: None,
        }
    }
}

#[cfg(test)]
mod test;
