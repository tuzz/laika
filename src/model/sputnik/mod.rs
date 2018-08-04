use super::direction::Direction;
use super::point::Point;

pub struct Sputnik {
    location: Point,
    travelling: Direction,
    heading: Direction,
}

impl Sputnik {
    pub fn new(heading: Direction, location: Point) -> Self {
        Sputnik { heading, travelling: heading, location }
    }
}

#[cfg(test)]
mod test;
