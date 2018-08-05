use super::*;

type Subject<T> = Random<T>;

mod sample_one {
    use super::*;

    #[test]
    fn it_draws_a_single_sample_from_the_distribution() {
        let sample = Subject::sample_one(0..=1);
        assert!(sample == 0 || sample == 1);
    }
}

mod sample {
    use super::*;

    #[test]
    fn it_samples_uniformly_from_the_distribution() {
        let random = Subject::new(0..=1);
        let mut sum = 0;

        for _ in 0..10_000 {
            sum += random.sample()
        }

        assert!(sum > 4_800);
        assert!(sum < 5_200);
    }

    #[test]
    fn it_can_sample_floats() {
        let random = Subject::new(0.0..=1.0);
        let mut sum = 0.0;

        for _ in 0..10_000 {
            sum += random.sample()
        }

        assert!(sum > 4_800.0);
        assert!(sum < 5_200.0);
    }
}
