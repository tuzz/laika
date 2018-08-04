use super::*;

type Subject = Direction;

mod new {
    use super::*;

    #[test]
    fn it_sets_degrees() {
        let direction = Subject::new(270.0);
        assert_eq!(direction.degrees, 270.0);
    }

    #[test]
    fn it_wraps_around_the_circle() {
        let direction = Subject::new(361.0);
        assert_approx_eq!(direction.degrees, 1.0);

        let direction = Subject::new(-1.0);
        assert_approx_eq!(direction.degrees, 359.0);
    }

    #[test]
    fn it_wraps_around_multiple_times() {
        let direction = Subject::new(721.0);
        assert_approx_eq!(direction.degrees, 1.0);

        let direction = Subject::new(-721.0);
        assert_approx_eq!(direction.degrees, 359.0);
    }

    #[test]
    fn it_normalises_to_zero_at_the_boundary() {
        let direction = Subject::new(360.0);
        assert_approx_eq!(direction.degrees, 0.0);
    }
}

mod add {
    use super::*;

    #[test]
    fn it_adds_two_directions() {
        let d1 = Subject::new(30.0);
        let d2 = Subject::new(40.0);

        let direction = d1 + d2;
        assert_approx_eq!(direction.degrees, 70.0);
    }

    #[test]
    fn it_wraps_around_the_circle() {
        let d1 = Subject::new(181.0);
        let d2 = Subject::new(182.0);

        let direction = d1 + d2;
        assert_approx_eq!(direction.degrees, 3.0);
    }
}

mod sub {
    use super::*;

    #[test]
    fn it_subtracts_one_direction_from_another() {
        let d1 = Subject::new(50.0);
        let d2 = Subject::new(40.0);

        let direction = d1 - d2;
        assert_approx_eq!(direction.degrees, 10.0);
    }

    #[test]
    fn it_wraps_around_the_circle() {
        let d1 = Subject::new(10.0);
        let d2 = Subject::new(40.0);

        let direction = d1 - d2;
        assert_approx_eq!(direction.degrees, 330.0);
    }
}
