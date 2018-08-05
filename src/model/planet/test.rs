use super::*;

type Subject = Planet;

fn setup() -> (Point, Subject) {
    let location = Point::new(0.5, 0.5);
    let planet = Subject::new(location, 0.1, 2.0, 3);

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
    fn it_sets_zone_from_location_radius_and_zone_height() {
        let (location, planet) = setup();
        let zone = planet.zone;

        assert_eq!(zone.center, location);
        assert_approx_eq!(zone.radius, 0.3);
    }

    #[test]
    fn it_sets_ordinal() {
        let (_, planet) = setup();
        assert_eq!(planet.ordinal, 3);
    }
}

mod zone_height {
    use super::*;

    #[test]
    fn it_returns_a_height_relative_to_the_radius_of_the_planet() {
        assert_approx_eq!(Subject::zone_height(0.1, 2.0), 0.3);
        assert_approx_eq!(Subject::zone_height(0.1, 3.0), 0.4);
        assert_approx_eq!(Subject::zone_height(0.2, 3.0), 0.8);
    }
}
