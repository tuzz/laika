mod galaxy;
mod planet;
mod sputnik;
mod thruster;

use self::galaxy::Galaxy;

pub struct Model {
    pub galaxy: Galaxy,
}

impl Model {
    pub fn new() -> Self {
        Self { galaxy: Galaxy::default() }
    }

    pub fn advance(&mut self, _delta: f64, _elapsed: f64) {
        // TODO: implement physics
    }
}
