use super::*;

type Subject = Sputnik;

fn setup() -> (Direction, Point, Subject) {
    let heading = Direction::new(270.0);
    let location = Point::new(0.1, 0.2);
    let sputnik = Subject::new(heading, location, 0.01);

    (heading, location, sputnik)
}

mod new {
    use super::*;

    #[test]
    fn it_sets_heading() {
        let (heading, _, sputnik) = setup();
        assert_eq!(sputnik.heading, heading);
    }

    #[test]
    fn it_sets_travelling_from_heading() {
        let (heading, _, sputnik) = setup();
        assert_eq!(sputnik.travelling, heading);
    }

    #[test]
    fn it_sets_thruster_to_none() {
        let (_, _, sputnik) = setup();
        assert_eq!(sputnik.thruster, None);
    }

    #[test]
    fn it_builds_a_hull_with_area_equal_to_hull_area() {
        let (_, _, sputnik) = setup();
        assert_approx_eq!(sputnik.hull.area(), 0.01);
    }

    #[test]
    fn it_builds_a_hull_centered_at_location() {
        let (_, location, sputnik) = setup();
        let centroid = sputnik.hull.centroid();

        assert_approx_eq!(centroid.x, location.x);
        assert_approx_eq!(centroid.y, location.y);
    }
}

mod left_thruster {
    use super::*;

    #[test]
    fn it_activates_the_left_thruster() {
        let (_, _, sputnik) = setup();
        let copy = sputnik.left_thruster();

        assert_eq!(copy.thruster, Some(Thruster::Left));
        assert_eq!(sputnik.thruster, None);
    }
}

mod right_thruster {
    use super::*;

    #[test]
    fn it_activates_the_right_thruster() {
        let (_, _, sputnik) = setup();
        let copy = sputnik.right_thruster();

        assert_eq!(copy.thruster, Some(Thruster::Right));
        assert_eq!(sputnik.thruster, None);
    }
}

mod no_thruster {
    use super::*;

    #[test]
    fn it_deactives_the_thrusters() {
        let (_, _, sputnik) = setup();
        let copy = sputnik.right_thruster().no_thruster();

        assert_eq!(copy.thruster, None);
        assert_eq!(sputnik.thruster, None);
    }
}

mod hull_radius {
    use super::*;

    #[test]
    fn it_returns_the_radius_of_a_hull_with_given_area() {
        let (_, _, sputnik) = setup();

        let expected = sputnik.hull.bounding_circle().radius;
        let actual = Subject::hull_radius(0.01);

        assert_approx_eq!(expected, actual);
    }
}
