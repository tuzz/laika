use ::util::{Circle, Direction, Point, Random};

use super::planet::Planet;
use super::sputnik::Sputnik;

use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct Galaxy {
    pub planets: Vec<Planet>,
    pub sputniks: Vec<Sputnik>,
    margin: f32,
}

struct Rands {
    planets: Random<usize>,
    radii: Random<f32>,
    zones: Random<f32>,
    sputniks: Random<usize>,
    areas: Random<f32>,
    margins: Random<f32>,
    locations: Random<f32>,
    directions: Random<f32>,
}

type R<T> = RangeInclusive<T>;

const MAX_ATTEMPTS: usize = 1000;

impl Galaxy {
    fn new(planets: Vec<Planet>, sputniks: Vec<Sputnik>, margin: f32) -> Self {
        Galaxy { planets, sputniks, margin }
    }

    fn random(planets: R<usize>, radii: R<f32>, zones: R<f32>, sputniks: R<usize>, areas: R<f32>, margins: R<f32>) -> Option<Self> {
        let rands = Rands {
            planets: Random::new(planets),
            radii: Random::new(radii),
            zones: Random::new(zones),
            sputniks: Random::new(sputniks),
            areas: Random::new(areas),
            margins: Random::new(margins),
            locations: Random::new(0.0..=1.0),
            directions: Random::new(0.0..=360.0),
        };

        let mut galaxy = Self::new(vec![], vec![], rands.margins.sample());

        for ordinal in 1..=rands.planets.sample() {
            galaxy = galaxy.add_random_planet(&rands, ordinal)?;
        }

        for _ in 0..rands.sputniks.sample() {
            galaxy = galaxy.add_random_sputnik(&rands)?;
        }

        Some(galaxy)
    }

    fn add_planet(&self, planet: Planet) -> Self {
        let planets = Self::extend(&self.planets, planet);
        Self::new(planets, self.sputniks.clone(), self.margin)
    }

    fn add_sputnik(&self, sputnik: Sputnik) -> Self {
        let sputniks = Self::extend(&self.sputniks, sputnik);
        Self::new(self.planets.clone(), sputniks, self.margin)
    }

    fn add_random_planet(&self, rands: &Rands, ordinal: usize) -> Option<Self> {
        let radius = rands.radii.sample();
        let zone = rands.zones.sample();
        let height = Planet::zone_height(radius, zone);
        let location = self.random_location(&rands.locations, height)?;

        let planet = Planet::new(location, radius, zone, ordinal);
        Some(self.add_planet(planet))
    }

    fn add_random_sputnik(&self, rands: &Rands) -> Option<Self> {
        let heading = Direction::new(rands.directions.sample());
        let area = rands.areas.sample();
        let radius = Sputnik::hull_radius(area);
        let location = self.random_location(&rands.locations, radius)?;

        let sputnik = Sputnik::new(heading, location, area);
        Some(self.add_sputnik(sputnik))
    }

    fn random_location(&self, rand: &Random<f32>, clearance: f32) -> Option<Point> {
        for _ in 0..MAX_ATTEMPTS {
            let point = Point::new(rand.sample(), rand.sample());
            let circle = Circle::new(point, clearance);

            if self.away_from_planets(circle) && self.away_from_margin(circle) {
                return Some(point);
            }
        }

        None
    }

    fn away_from_planets(&self, circle: Circle) -> bool {
        !self.planets.iter().any(|p| p.zone.intersects(circle))
    }

    fn away_from_margin(&self, circle: Circle) -> bool {
        let c = circle;
        let (x, y) = (c.center.x, c.center.y);
        let margin = self.margin;

        let p1 = Point::new(margin, y);
        let p2 = Point::new(1.0 - margin, y);
        let p3 = Point::new(x, margin);
        let p4 = Point::new(x, 1.0 - margin);

        !c.contains(p1) && !c.contains(p2) && !c.contains(p3) && !c.contains(p4)
    }

    fn extend<T: Clone>(vec: &Vec<T>, element: T) -> Vec<T> {
        [&vec[..], &[element]].concat()
    }
}

impl Default for Galaxy {
    fn default() -> Self {
        for _ in 0..MAX_ATTEMPTS {
            let planets = 3..=5;
            let radii = 0.02..=0.03;
            let zones = 1.5..=2.5;
            let sputniks = 7..=10;
            let areas = 0.01..=0.02;
            let margins = 0.02..=0.03;

            let option = Self::random(planets, radii, zones, sputniks, areas, margins);

            if let Some(galaxy) = option {
                return galaxy;
            }
        }

        panic!("failed to generate random galaxy");
    }
}

#[cfg(test)]
mod test;
