use super::*;

type Subject = Galaxy;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_empty_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);

        assert_eq!(galaxy.planets.len(), 0);
        assert_eq!(galaxy.sputniks.len(), 0);
    }
}

mod add_planet {
    use super::*;

    #[test]
    fn it_adds_the_planet_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);
        let location = Point::new(0.5, 0.5);
        let planet = Planet::new(location, 0.1, 3);

        let copy = galaxy.add_planet(planet);

        assert_eq!(copy.planets.len(), 1);
        assert_eq!(galaxy.planets.len(), 0);
    }
}

mod add_sputnik {
    use super::*;

    #[test]
    fn it_adds_the_sputnik_to_the_galaxy() {
        let galaxy = Subject::new(vec![], vec![]);
        let heading = Direction::new(270.0);
        let location = Point::new(0.1, 0.2);
        let sputnik = Sputnik::new(heading, location, 0.01);

        let copy = galaxy.add_sputnik(sputnik);

        assert_eq!(copy.sputniks.len(), 1);
        assert_eq!(galaxy.sputniks.len(), 0);
    }
}
