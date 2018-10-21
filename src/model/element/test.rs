use super::*;

type Subject = Element;

mod new {
    use super::*;

    #[test]
    fn it_has_a_name_and_color() {
        let iron = Subject::new("Iron".to_string(), 1.0, 0.1, 0.0);

        assert_eq!(iron.name, "Iron");
        assert_eq!(iron.color.red, 1.0);
        assert_eq!(iron.color.green, 0.1);
        assert_eq!(iron.color.blue, 0.0);
    }
}
