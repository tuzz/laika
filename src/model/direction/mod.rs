use std::ops::{Add,Sub};

pub struct Direction {
    degrees: f64,
}

impl Direction {
    pub fn new(degrees: f64) -> Self {
        Self { degrees: Self::normalize(degrees) }
    }

    fn normalize(degrees: f64) -> f64 {
        let m = degrees % 360.0;

        if m < 0.0 {
            m + 360.0
        } else {
            m
        }
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.degrees + other.degrees)
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.degrees - other.degrees)
    }
}

#[cfg(test)]
mod test;
