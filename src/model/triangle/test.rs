use super::*;

type Subject = Triangle;

mod new {
    use super::*;

    #[test]
    fn it_sets_a_b_and_c() {
        let a = Point::new(0.5, 0.5);
        let b = Point::new(0.2, 0.2);
        let c = Point::new(0.8, 0.2);

        let triangle = Subject::new(a, b, c);

        assert_eq!(triangle.a, a);
        assert_eq!(triangle.b, b);
        assert_eq!(triangle.c, c);
    }
}

mod area {
    use super::*;

    #[test]
    fn it_calculates_the_area() {
        let a = Point::new(0.5, 0.5);
        let b = Point::new(0.2, 0.2);
        let c = Point::new(0.8, 0.2);

        let triangle = Subject::new(a, b, c);

        assert_approx_eq!(triangle.area(), 0.09);
    }

    #[test]
    fn it_is_positive() {
        let a = Point::new(0.5, 0.5);
        let b = Point::new(0.8, 0.2);
        let c = Point::new(0.2, 0.2);

        let triangle = Subject::new(a, b, c);

        assert_approx_eq!(triangle.area(), 0.09);
    }

    #[test]
    fn it_calculates_the_centroid() {
        let a = Point::new(0.5, 0.5);
        let b = Point::new(0.2, 0.2);
        let c = Point::new(0.8, 0.2);

        let triangle = Subject::new(a, b, c);
        let centroid = triangle.centroid();

        assert_approx_eq!(centroid.x, 0.5);
        assert_approx_eq!(centroid.y, 0.3);
    }
}

mod bounding_circle {
    use super::*;

    #[test]
    fn it_returns_the_smallest_circle_that_contains_the_triangle() {
        let a = Point::new(0.5, 0.5);
        let b = Point::new(0.2, 0.2);
        let c = Point::new(0.8, 0.2);

        let triangle = Subject::new(a, b, c);
        let circle = triangle.bounding_circle();
        let center = circle.center;

        assert_eq!(center.x, 0.5);
        assert_eq!(center.y, 0.3);

        let expected = b.distance(center);
        assert_eq!(circle.radius, expected);
    }
}
