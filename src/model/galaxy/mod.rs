use super::circle::Circle;
use super::direction::Direction;
use super::planet::Planet;
use super::point::Point;
use super::random::Random;
use super::sputnik::Sputnik;

use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
struct Galaxy {
    planets: Vec<Planet>,
    sputniks: Vec<Sputnik>,
    margin: f64,
}

type R<T> = RangeInclusive<T>;
type RandF = Random<f64>;

const MAX_ATTEMPTS: usize = 100;

impl Galaxy {
    fn new(planets: Vec<Planet>, sputniks: Vec<Sputnik>, margin: f64) -> Self {
        Galaxy { planets, sputniks, margin }
    }

    fn random(planets: R<usize>, radii: R<f64>, sputniks: R<usize>, areas: R<f64>, margins: R<f64>) -> Option<Self> {
        let margin = Random::sample_one(margins);
        let mut galaxy = Self::new(vec![], vec![], margin);

        let radii_rand = Random::new(radii);
        let areas_rand = Random::new(areas);
        let locations_rand = Random::new(margin..=1.0 - margin);
        let directions_rand = Random::new(0.0..=360.0);

        for ordinal in 1..=Random::sample_one(planets) {
            galaxy = galaxy.add_random_planet(&radii_rand, &locations_rand, ordinal)?;
        }

        for _ in 0..Random::sample_one(sputniks) {
            galaxy = galaxy.add_random_sputnik(&areas_rand, &locations_rand, &directions_rand)?;
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

    fn add_random_planet(&self, radii: &RandF, locations: &RandF, ordinal: usize) -> Option<Self> {
        let radius = radii.sample();
        let location = self.random_location(locations, radius)?;
        let planet = Planet::new(location, radius, ordinal);

        Some(self.add_planet(planet))
    }

    fn add_random_sputnik(&self, areas: &RandF, locations: &RandF, directions: &RandF) -> Option<Self> {
        let heading = Direction::new(directions.sample());
        let area = areas.sample();
        let location = self.random_location(locations, area)?;
        let sputnik = Sputnik::new(heading, location, area);

        Some(self.add_sputnik(sputnik))
    }

    fn random_location(&self, rand: &RandF, clearance: f64) -> Option<Point> {
        for _ in 0..MAX_ATTEMPTS {
            let point = Point::new(rand.sample(), rand.sample());
            let circle = Circle::new(point, clearance);

            if self.away_from_planets(circle) {
                return Some(point);
            }
        }

        None
    }

    fn away_from_planets(&self, circle: Circle) -> bool {
        !self.planets.iter().any(|p| p.orbital_zone().intersects(circle))
    }

    fn extend<T: Clone>(vec: &Vec<T>, element: T) -> Vec<T> {
        [&vec[..], &[element]].concat()
    }
}

#[cfg(test)]
mod test;
