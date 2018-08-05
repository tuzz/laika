use super::direction::Direction;
use super::planet::Planet;
use super::point::Point;
use super::sputnik::Sputnik;

struct Galaxy {
    planets: Vec<Planet>,
    sputniks: Vec<Sputnik>,
}

impl Galaxy {
    fn new(planets: Vec<Planet>, sputniks: Vec<Sputnik>) -> Self {
        Galaxy { planets, sputniks }
    }

    fn add_planet(&self, planet: Planet) -> Self {
        let planets = Self::extend(&self.planets, planet);
        Self::new(planets, self.sputniks.clone())
    }

    fn add_sputnik(&self, sputnik: Sputnik) -> Self {
        let sputniks = Self::extend(&self.sputniks, sputnik);
        Self::new(self.planets.clone(), sputniks)
    }

    fn extend<T: Clone>(vec: &Vec<T>, element: T) -> Vec<T> {
        [&vec[..], &[element]].concat()
    }
}

#[cfg(test)]
mod test;
