use super::*;

type Subject = Galaxy;

fn assert_between<T: PartialOrd>(lower: T, upper: T, value: T) {
    assert!(value >= lower);
    assert!(value <= upper);
}

const ITERATIONS: usize = 1_000;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_empty_galaxy() {
        let galaxy = Subject::new(vec![], vec![], 0.02);

        assert_eq!(galaxy.planets.len(), 0);
        assert_eq!(galaxy.sputniks.len(), 0);
        assert_eq!(galaxy.margin, 0.02);
    }
}

mod random {
    use super::*;

    fn setup() -> Option<Subject> {
        let planets = 3..=5;
        let radii = 0.02..=0.03;
        let zones = 1.5..=2.5;
        let sputniks = 7..=10;
        let areas = 0.01..=0.02;
        let margins = 0.02..=0.03;

        Subject::random(planets, radii, zones, sputniks, areas, margins)
    }

    #[test]
    fn it_generates_a_random_galaxy_with_planet_and_sputnik_parameters() {
        for _ in 0..ITERATIONS {
            let galaxy = setup().unwrap();

            assert_between(3, 5, galaxy.planets.len());
            assert_between(7, 10, galaxy.sputniks.len());
            assert_between(0.02, 0.03, galaxy.margin);

            for planet in galaxy.planets {
                assert_between(0.02, 0.05, planet.mass.radius);
                assert_between(0.05, 0.105, planet.zone.radius);
            }

            for sputnik in galaxy.sputniks {
                assert_between(0.01, 0.02, sputnik.hull.area());
            }
        }
    }

    #[test]
    fn it_sets_ordinals_for_planets_starting_from_one() {
        let galaxy = setup().unwrap();

        for (ordinal, planet) in galaxy.planets.iter().enumerate() {
            assert_eq!(planet.ordinal, ordinal + 1);
        }
    }

    #[test]
    fn it_positions_planets_away_from_other_planets() {
        for _ in 0..ITERATIONS {
            let galaxy = setup().unwrap();

            let zone1 = galaxy.planets[0].zone;
            let zone2 = galaxy.planets[1].zone;

            assert!(!zone1.intersects(zone2))
        }
    }

    #[test]
    fn it_positions_sputniks_away_from_planets() {
        for _ in 0..ITERATIONS {
            let galaxy = setup().unwrap();

            let zone = galaxy.planets[0].zone;
            let hull = galaxy.sputniks[0].hull;

            assert!(!zone.intersects(hull.bounding_circle()))
        }
    }

    #[test]
    fn it_positions_planets_away_from_the_margin() {
        for _ in 0..ITERATIONS {
            let galaxy = setup().unwrap();
            let zone = galaxy.planets[0].zone;

            let p1 = Point::new(zone.center.x, galaxy.margin);
            let p2 = Point::new(zone.center.x, 1.0 - galaxy.margin);
            let p3 = Point::new(galaxy.margin, zone.center.y);
            let p4 = Point::new(1.0 - galaxy.margin, zone.center.y);

            assert!(!zone.contains(p1));
            assert!(!zone.contains(p2));
            assert!(!zone.contains(p3));
            assert!(!zone.contains(p4));
        }
    }

    #[test]
    fn it_positions_sputniks_away_from_the_margin() {
        for _ in 0..ITERATIONS {
            let galaxy = setup().unwrap();
            let circle = galaxy.sputniks[0].hull.bounding_circle();

            let p1 = Point::new(circle.center.x, galaxy.margin);
            let p2 = Point::new(circle.center.x, 1.0 - galaxy.margin);
            let p3 = Point::new(galaxy.margin, circle.center.y);
            let p4 = Point::new(1.0 - galaxy.margin, circle.center.y);

            assert!(!circle.contains(p1));
            assert!(!circle.contains(p2));
            assert!(!circle.contains(p3));
            assert!(!circle.contains(p4));
        }
    }

    #[test]
    fn it_returns_none_if_it_fails_to_build_a_galaxy_with_the_parameters() {
        let option = Subject::random(100..=100, 0.05..=0.05, 0.0..=0.0, 0..=0, 0.0..=0.0, 0.0..=0.0);
        assert_eq!(option, None);
    }
}

mod add_planet {
    use super::*;

    #[test]
    fn it_adds_the_planet_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![], 0.02);
        let location = Point::new(0.5, 0.5);
        let planet = Planet::new(location, 0.1, 2.0, 3);

        let copy = galaxy.add_planet(planet);

        assert_eq!(copy.planets.len(), 1);
        assert_eq!(galaxy.planets.len(), 0);
    }
}

mod add_sputnik {
    use super::*;

    #[test]
    fn it_adds_the_sputnik_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![], 0.02);
        let heading = Direction::new(270.0);
        let location = Point::new(0.1, 0.2);
        let sputnik = Sputnik::new(heading, location, 0.01);

        let copy = galaxy.add_sputnik(sputnik);

        assert_eq!(copy.sputniks.len(), 1);
        assert_eq!(galaxy.sputniks.len(), 0);
    }
}

mod random_location {
    use super::*;

    #[test]
    fn it_picks_locations_from_the_distribution() {
        let galaxy = Subject::new(vec![], vec![], 0.02);
        let distribution = Random::new(0.0..=1.0);

        for _ in 0..ITERATIONS {
            let point = galaxy.random_location(&distribution, 0.0).unwrap();

            assert_between(0.0, 1.0, point.x);
            assert_between(0.0, 1.0, point.y);
        }
    }

    #[test]
    fn it_picks_locations_outside_planet_zones() {
        let planet = Planet::new(Point::new(0.5, 0.5), 0.1, 2.0, 3);
        let zone = planet.zone.clone();

        let galaxy = Subject::new(vec![], vec![], 0.02).add_planet(planet);
        let distribution = Random::new(0.0..=1.0);

        for _ in 0..ITERATIONS {
            let point = galaxy.random_location(&distribution, 0.0).unwrap();
            assert!(!zone.contains(point));
        }
    }

    #[test]
    fn it_picks_locations_some_distance_away_from_planet_zones() {
        let planet = Planet::new(Point::new(0.5, 0.5), 0.1, 2.0, 3);
        let zone = planet.zone.clone();

        let galaxy = Subject::new(vec![], vec![], 0.02).add_planet(planet);
        let distribution = Random::new(0.0..=1.0);

        for _ in 0..ITERATIONS {
            let distance = 0.01;

            let point = galaxy.random_location(&distribution, distance);
            let circle = Circle::new(point.unwrap(), distance);

            assert!(!zone.intersects(circle));
        }
    }

    #[test]
    fn it_returns_none_if_it_cant_find_a_point() {
        let planet = Planet::new(Point::new(0.5, 0.5), 0.1, 2.0, 3);
        let galaxy = Subject::new(vec![], vec![], 0.02).add_planet(planet);
        let distribution = Random::new(0.0..=1.0);
        let location = galaxy.random_location(&distribution, 1.0);

        assert_eq!(location, None);
    }
}

mod default {
    use super::*;

    #[test]
    fn it_generates_a_random_galaxy() {
        let galaxy = Subject::default();
        assert_between(3, 5, galaxy.planets.len());
    }
}
