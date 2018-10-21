use super::*;

type Subject = Composition;

fn assert_between<T: PartialOrd>(lower: T, upper: T, value: T) {
    assert!(value >= lower);
    assert!(value <= upper);
}

mod from_distribution {
    use super::*;

    #[test]
    fn it_randomly_chooses_elements_from_the_distribution() {
        let distribution = vec![(Element::iron(), 0.3), (Element::copper(), 0.7)];
        let composition = Subject::from_distribution(1_000, distribution);

        let iron_count = composition.elements.iter()
            .filter(|&e| *e == Element::iron()).count();

        let copper_count = composition.elements.iter()
            .filter(|&e| *e == Element::copper()).count();

        assert_between(250, 350, iron_count);
        assert_between(650, 750, copper_count);
    }
}

mod diminishing_distribution {
    use super::*;

    #[test]
    fn it_contains_one_of_every_element() {
        let elements = Element::all();
        let distribution = Subject::diminishing_distribution();

        let actual: Vec<Element> = distribution.into_iter().map(|t| t.0).collect();
        assert_eq!(actual.len(), elements.len());

        for e in Element::all() {
            assert!(actual.contains(&e));
        }
    }

    #[test]
    fn it_sums_to_one() {
        let distribution = Subject::diminishing_distribution();
        let sum = distribution.iter().fold(0.0, |acc, t| acc + t.1);

        assert_approx_eq!(sum, 1.0);
    }
}
