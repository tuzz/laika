use super::*;

fn setup() -> (Direction, Point, Sputnik) {
    let heading = Direction::new(270.0);
    let location = Point::new(0.1, 0.2);
    let sputnik = Sputnik::new(heading, location);

    (heading, location, sputnik)
}

mod new {
    use super::*;

    #[test]
    fn it_sets_heading_and_location() {
        let (heading, location, sputnik) = setup();

        assert_eq!(sputnik.heading, heading);
        assert_eq!(sputnik.location, location);
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
