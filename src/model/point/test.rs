use super::*;

type Subject = Point;

mod new {
    use super::*;

    #[test]
    fn it_sets_x_and_y() {
        let point = Subject::new(0.1, 0.2);

        assert_eq!(point.x, 0.1);
        assert_eq!(point.y, 0.2);
    }

    #[test]
    fn it_wraps_around_the_unit_square() {
        let point = Subject::new(-0.1, 1.2);

        assert_approx_eq!(point.x, 0.9);
        assert_approx_eq!(point.y, 0.2);
    }

    #[test]
    fn it_wraps_around_multiple_times() {
        let point = Subject::new(-1.1, 3.2);

        assert_approx_eq!(point.x, 0.9);
        assert_approx_eq!(point.y, 0.2);
    }

    #[test]
    fn it_normalises_to_zero_at_the_boundary() {
        let point = Subject::new(1.0, -1.0);

        assert_approx_eq!(point.x, 0.0);
        assert_approx_eq!(point.y, 0.0);
    }
}
