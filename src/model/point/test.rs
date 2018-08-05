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

mod distance {
    use super::*;

    #[test]
    fn it_returns_the_distance_to_other_point() {
        let p1 = Subject::new(0.1, 0.3);
        let p2 = Subject::new(0.4, 0.7);

        assert_approx_eq!(p1.distance(p2), 0.5);
    }
}

mod add {
    use super::*;

    #[test]
    fn it_adds_two_points() {
        let p1 = Subject::new(0.2, 0.3);
        let p2 = Subject::new(0.4, 0.5);

        let point = p1 + p2;

        assert_approx_eq!(point.x, 0.6);
        assert_approx_eq!(point.y, 0.8);
    }

    #[test]
    fn it_wraps_around_the_unit_square() {
        let p1 = Subject::new(0.6, 0.7);
        let p2 = Subject::new(0.8, 0.9);

        let point = p1 + p2;

        assert_approx_eq!(point.x, 0.4);
        assert_approx_eq!(point.y, 0.6);
    }
}

mod sub {
    use super::*;

    #[test]
    fn it_subtracts_one_point_from_another() {
        let p1 = Subject::new(0.6, 0.8);
        let p2 = Subject::new(0.4, 0.5);

        let point = p1 - p2;

        assert_approx_eq!(point.x, 0.2);
        assert_approx_eq!(point.y, 0.3);
    }

    #[test]
    fn it_wraps_around_the_unit_square() {
        let p1 = Subject::new(0.1, 0.3);
        let p2 = Subject::new(0.4, 0.5);

        let point = p1 - p2;

        assert_approx_eq!(point.x, 0.7);
        assert_approx_eq!(point.y, 0.8);
    }
}
