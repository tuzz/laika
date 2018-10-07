mod circle;
mod direction;
mod galaxy;
mod planet;
mod point;
mod random;
mod sputnik;
mod thruster;
mod triangle;

use self::galaxy::Galaxy;

pub struct Model {
    _galaxy: Galaxy,
}

impl Model {
    pub fn new() -> Self {
        Self { _galaxy: Galaxy::default() }
    }

    pub fn advance(&mut self, _delta: f64, _elapsed: f64) {
        // TODO: implement physics
    }
}
