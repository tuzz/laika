use std::ops::{Add,Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Direction {
    degrees: f32,
}

impl Direction {
    pub fn new(degrees: f32) -> Self {
        Self { degrees: Self::normalize(degrees) }
    }

    fn normalize(degrees: f32) -> f32 {
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
