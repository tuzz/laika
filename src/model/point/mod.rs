use std::ops::{Add,Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        let x1 = Self::normalize(x);
        let y1 = Self::normalize(y);

        Self { x: x1, y: y1 }
    }

    fn normalize(x: f64) -> f64 {
        let m = x % 1.0;

        if m < 0.0 {
            m + 1.0
        } else {
            m
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;

        Self::new(x, y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;

        Self::new(x, y)
    }
}

#[cfg(test)]
mod test;
