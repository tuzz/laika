pub struct Direction {
    degrees: f64,
}

impl Direction {
    pub fn new(degrees: f64) -> Self {
        Direction { degrees: Self::normalize(degrees) }
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

#[cfg(test)]
mod test;
