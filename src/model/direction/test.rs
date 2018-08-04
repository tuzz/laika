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
