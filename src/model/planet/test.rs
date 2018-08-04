use super::*;

fn setup() -> (Point, Planet) {
    let location = Point::new(0.5, 0.5);
    let planet = Planet::new(location, 0.1, 3);

    (location, planet)
}

mod new {
    use super::*;

    #[test]
    fn it_sets_location_radius_and_ordinal() {
        let (location, planet) = setup();

        assert_eq!(planet.location, location);
        assert_eq!(planet.radius, 0.1);
        assert_eq!(planet.ordinal, 3);
    }
}

mod collided {
    use super::*;

    #[test]
    fn it_returns_true_if_inside_of_radius() {
        let (_, planet) = setup();
        let point = Point::new(0.57, 0.57);

        assert!(planet.collided(point));
    }

    #[test]
    fn it_returns_false_if_outside_of_radius() {
        let (_, planet) = setup();
        let point = Point::new(0.58, 0.58);

        assert!(!planet.collided(point));
    }
}
