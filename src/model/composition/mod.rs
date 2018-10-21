use super::element::Element;

use ::util::Random;

#[derive(Clone, Debug, PartialEq)]
pub struct Composition {
    pub elements: Vec<Element>,
}

impl Composition {
    pub fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }

    pub fn random(number_of_elements: usize) -> Self {
        let distribution = Self::diminishing_distribution();
        Self::from_distribution(number_of_elements, distribution)
    }

    fn from_distribution(number_of_elements: usize, distribution: Vec<(Element, f32)>) -> Self {
        let mut elements = vec![];
        let random = Random::new(0.0..=1.0);

        'outer: for _ in 0..number_of_elements {
            let sample = random.sample();
            let mut total = 0.0;

            for (element, probability) in distribution.iter() {
                total += probability;

                if sample <= total {
                    elements.push(element.clone());
                    continue 'outer;
                }
            }
        }

        Self::new(elements)
    }

    fn diminishing_distribution() -> Vec<(Element, f32)> {
        let mut distribution = vec![];

        let mut elements = Element::all();
        let mut limit = 1.0;

        for remaining in (0..elements.len()).rev() {
            let index = Random::new(0..=remaining).sample();
            let element = elements.remove(index);

            let abundance;
            if remaining == 0 {
                abundance = limit
            } else {
                abundance = Random::new(0.0..=limit).sample();
                limit -= abundance;
            }

            distribution.push((element, abundance));
        }

        distribution
    }
}

#[cfg(test)]
mod test;
