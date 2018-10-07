use super::*;

type Subject = Matrix;

fn subject() -> Subject {
    Subject::new([
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ])
}

mod identity {
    use super::*;

    #[test]
    fn it_builds_the_identity_matrix() {
        let matrix = Subject::identity();
        let expected = &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];

        assert_eq!(&matrix.array, expected);
    }
}

mod translation {
    use super::*;

    #[test]
    fn it_builds_a_translation_matrix() {
        let matrix = Subject::translation(1.2, 3.4);
        let expected = &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            1.2, 3.4, 1.0,
        ];

        assert_eq!(&matrix.array, expected);
    }
}

mod rotation {
    use super::*;

    #[test]
    fn it_builds_a_rotation_matrix() {
        let matrix = Subject::rotation(1.2);

        let sin = 0.9320391;
        let cos = 0.3623577;

        let expected = &[
            cos, -sin, 0.0,
            sin,  cos, 0.0,
            0.0,  0.0, 1.0,
        ];

        assert_eq!(&matrix.array, expected);
    }
}

mod scalar {
    use super::*;

    #[test]
    fn it_builds_a_scalar_matrix() {
        let matrix = Subject::scalar(1.2, 3.4);

        let expected = &[
            1.2, 0.0, 0.0,
            0.0, 3.4, 0.0,
            0.0, 0.0, 1.0,
        ];

        assert_eq!(&matrix.array, expected);
    }
}

mod multiply {
    use super::*;

    #[test]
    fn it_multiplies_the_matrix_by_another() {
        let another = Subject::new([
            0.1, 0.2, 0.3,
            0.4, 0.5, 0.6,
            0.7, 0.8, 0.9,
        ]);

        let result = subject().multiply(&another);
        let expected = &[
            // First row:
            1.0 * 0.1    +    2.0 * 0.4    +    3.0 * 0.7,
            1.0 * 0.2    +    2.0 * 0.5    +    3.0 * 0.8,
            1.0 * 0.3    +    2.0 * 0.6    +    3.0 * 0.9,

            // Second row:
            4.0 * 0.1    +    5.0 * 0.4    +    6.0 * 0.7,
            4.0 * 0.2    +    5.0 * 0.5    +    6.0 * 0.8,
            4.0 * 0.3    +    5.0 * 0.6    +    6.0 * 0.9,

            // Third row:
            7.0 * 0.1    +    8.0 * 0.4    +    9.0 * 0.7,
            7.0 * 0.2    +    8.0 * 0.5    +    9.0 * 0.8,
            7.0 * 0.3    +    8.0 * 0.6    +    9.0 * 0.9,
        ];

        assert_eq!(&result.array, expected);
    }
}
