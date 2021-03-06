use std::ops::{Add,Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }

    pub fn wrap_around(&self) -> Self {
        let x = Self::normalize(self.x);
        let y = Self::normalize(self.y);

        Self::new(x, y)
    }

    fn normalize(x: f32) -> f32 {
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
