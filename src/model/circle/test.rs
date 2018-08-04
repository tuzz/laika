use super::*;

fn setup() -> (Point, Circle) {
    let center = Point::new(0.5, 0.5);
    let radius = 0.1;

    let circle = Circle::new(center, radius);

    (center, circle)
}

mod new {
    use super::*;

    #[test]
    fn it_sets_center_and_radius() {
        let (center, circle) = setup();

        assert_eq!(circle.center, center);
        assert_eq!(circle.radius, 0.1);
    }
}

mod contains {
    use super::*;

    #[test]
    fn it_returns_true_if_circle_contains_point() {
        let (center, circle) = setup();
        let point = center + Point::new(0.07, 0.07);

        assert!(circle.contains(point));
    }

    #[test]
    fn it_returns_false_if_circle_does_not_contain_point() {
        let (center, circle) = setup();
        let point = center + Point::new(0.08, 0.08);

        assert!(!circle.contains(point));
    }
}
