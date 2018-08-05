use super::direction::Direction;
use super::planet::Planet;
use super::point::Point;
use super::random::Random;
use super::sputnik::Sputnik;

use std::ops::RangeInclusive;

struct Galaxy {
    planets: Vec<Planet>,
    sputniks: Vec<Sputnik>,
}

type R<T> = RangeInclusive<T>;
type RandF = Random<f64>;

impl Galaxy {
    fn new(planets: Vec<Planet>, sputniks: Vec<Sputnik>) -> Self {
        Galaxy { planets, sputniks }
    }

    fn random(planets: R<usize>, radii: R<f64>, sputniks: R<usize>, areas: R<f64>) -> Self {
        let mut galaxy = Self::new(vec![], vec![]);

        let radii_rand = Random::new(radii);
        let areas_rand = Random::new(areas);
        let locations_rand = Random::new(0.0..=1.0);
        let directions_rand = Random::new(0.0..=360.0);

        for ordinal in 1..=Random::sample_one(planets) {
            galaxy = galaxy.add_random_planet(&radii_rand, &locations_rand, ordinal);
        }

        for _ in 0..Random::sample_one(sputniks) {
            galaxy = galaxy.add_random_sputnik(&areas_rand, &locations_rand, &directions_rand);
        }

        galaxy
    }

    fn add_planet(&self, planet: Planet) -> Self {
        let planets = Self::extend(&self.planets, planet);
        Self::new(planets, self.sputniks.clone())
    }

    fn add_sputnik(&self, sputnik: Sputnik) -> Self {
        let sputniks = Self::extend(&self.sputniks, sputnik);
        Self::new(self.planets.clone(), sputniks)
    }

    fn add_random_planet(&self, radii: &RandF, locations: &RandF, ordinal: usize) -> Self {
        let radius = radii.sample();
        let location = Self::random_location(locations, radius);
        let planet = Planet::new(location, radius, ordinal);

        self.add_planet(planet)
    }

    fn add_random_sputnik(&self, areas: &RandF, locations: &RandF, directions: &RandF) -> Self {
        let heading = Self::random_direction(directions);
        let area = areas.sample();
        let location = Self::random_location(locations, area);
        let sputnik = Sputnik::new(heading, location, area);

        self.add_sputnik(sputnik)
    }

    fn random_location(rand: &RandF, _todo: f64) -> Point {
        Point::new(rand.sample(), rand.sample())
    }

    fn random_direction(rand: &RandF) -> Direction {
        Direction::new(rand.sample())
    }

    fn extend<T: Clone>(vec: &Vec<T>, element: T) -> Vec<T> {
        [&vec[..], &[element]].concat()
    }
}

#[cfg(test)]
mod test;
