use super::*;

fn setup() -> (Point, Planet) {
    let location = Point::new(0.5, 0.5);
    let planet = Planet::new(location, 0.1, 3);

    (location, planet)
}

mod new {
    use super::*;

    #[test]
    fn it_sets_mass_from_location_and_radius() {
        let (location, planet) = setup();
        let mass = planet.mass;

        assert_eq!(mass.center, location);
        assert_eq!(mass.radius, 0.1);
    }

    #[test]
    fn it_sets_ordinal() {
        let (_, planet) = setup();
        assert_eq!(planet.ordinal, 3);
    }
}

mod orbital_zone {
    use super::*;

    #[test]
    fn it_returns_a_circle_with_the_same_center() {
        let (location, planet) = setup();
        let circle = planet.orbital_zone();

        assert_eq!(circle.center, location);
    }

    #[test]
    fn it_returns_a_circle_with_three_times_the_radius() {
        let (_, planet) = setup();
        let circle = planet.orbital_zone();

        assert_approx_eq!(circle.radius, 0.3);
    }
}
