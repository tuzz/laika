use super::*;

type Subject = Sputnik;

mod new {
    use super::*;

    #[test]
    fn it_sets_heading_and_location() {
        let heading = Direction::new(270.0);
        let location = Point::new(0.1, 0.2);

        let sputnik = Subject::new(heading, location);

        assert_eq!(sputnik.heading, heading);
        assert_eq!(sputnik.location, location);
    }

    #[test]
    fn it_sets_travelling_from_heading() {
        let heading = Direction::new(270.0);
        let location = Point::new(0.1, 0.2);

        let sputnik = Subject::new(heading, location);

        assert_eq!(sputnik.travelling, heading);
    }
}
