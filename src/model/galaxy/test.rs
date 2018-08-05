use super::*;

type Subject = Galaxy;

fn assert_between<T: PartialOrd>(lower: T, upper: T, value: T) {
    assert!(value >= lower);
    assert!(value <= upper);
}

mod new {
    use super::*;

    #[test]
    fn it_builds_an_empty_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);

        assert_eq!(galaxy.planets.len(), 0);
        assert_eq!(galaxy.sputniks.len(), 0);
    }
}

mod random {
    use super::*;

    #[test]
    fn it_generates_a_random_galaxy_with_planet_and_sputnik_parameters() {
        for _ in 0..20 {
            let planets = 3..=5;
            let radii = 0.05..=0.1;
            let sputniks = 7..=10;
            let areas = 0.01..=0.02;

            let galaxy = Subject::random(planets, radii, sputniks, areas);

            assert_between(3, 5, galaxy.planets.len());
            assert_between(7, 10, galaxy.sputniks.len());

            for planet in galaxy.planets {
                assert_between(0.05, 0.1, planet.mass.radius);
            }

            for sputnik in galaxy.sputniks {
                assert_between(0.01, 0.02, sputnik.hull.area());
            }
        }
    }

    #[test]
    fn it_sets_ordinals_for_planets_starting_from_one() {
        let galaxy = Subject::random(5..=5, 0.1..=0.1, 0..=0, 0.1..=0.1);

        for (ordinal, planet) in galaxy.planets.iter().enumerate() {
            assert_eq!(planet.ordinal, ordinal + 1);
        }
    }
}

mod add_planet {
    use super::*;

    #[test]
    fn it_adds_the_planet_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);
        let location = Point::new(0.5, 0.5);
        let planet = Planet::new(location, 0.1, 3);

        let copy = galaxy.add_planet(planet);

        assert_eq!(copy.planets.len(), 1);
        assert_eq!(galaxy.planets.len(), 0);
    }
}

mod add_sputnik {
    use super::*;

    #[test]
    fn it_adds_the_sputnik_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);
        let heading = Direction::new(270.0);
        let location = Point::new(0.1, 0.2);
        let sputnik = Sputnik::new(heading, location, 0.01);

        let copy = galaxy.add_sputnik(sputnik);

        assert_eq!(copy.sputniks.len(), 1);
        assert_eq!(galaxy.sputniks.len(), 0);
    }
}
