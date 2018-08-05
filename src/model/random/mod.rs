use rand::{thread_rng,ThreadRng};
use rand::distributions::{Distribution,Uniform};
use rand::distributions::uniform::SampleUniform;

use std::cell::RefCell;

struct Random<T> where T: SampleUniform {
    distribution: Uniform<T>,
    generator: RefCell<ThreadRng>,
}

impl<T> Random<T> where T: SampleUniform {
    pub fn new(lower: T, upper: T) -> Self {
        let distribution = Uniform::new_inclusive(lower, upper);
        let generator = RefCell::new(thread_rng());

        Random { distribution, generator }
    }

    pub fn sample(&self) -> T {
        let generator = &mut *self.generator.borrow_mut();
        self.distribution.sample(generator)
    }
}

#[cfg(test)]
mod test;
