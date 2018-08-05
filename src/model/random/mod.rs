use rand::{thread_rng,ThreadRng};
use rand::distributions::{Distribution,Uniform};
use rand::distributions::uniform::SampleUniform;

use std::cell::RefCell;
use std::ops::RangeInclusive;

pub struct Random<T> where T: SampleUniform {
    distribution: Uniform<T>,
    generator: RefCell<ThreadRng>,
}

impl<T> Random<T> where T: SampleUniform {
    pub fn new(range: RangeInclusive<T>) -> Self {
        let (lower, upper) = range.into_inner();
        let distribution = Uniform::new_inclusive(lower, upper);
        let generator = RefCell::new(thread_rng());

        Random { distribution, generator }
    }

    pub fn sample_one(range: RangeInclusive<T>) -> T {
        Self::new(range).sample()
    }

    pub fn sample(&self) -> T {
        let generator = &mut *self.generator.borrow_mut();
        self.distribution.sample(generator)
    }
}

#[cfg(test)]
mod test;
