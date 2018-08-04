use super::point::Point;

struct Planet {
    location: Point,
    radius: f64,
    ordinal: usize,
}

impl Planet {
    fn new(location: Point, radius: f64, ordinal: usize) -> Self {
        Planet { location, radius, ordinal }
    }

    fn collided(&self, point: Point) -> bool {
        let x = self.location.x - point.x;
        let y = self.location.y - point.y;

        let x2 = x.powf(2.0);
        let y2 = y.powf(2.0);
        let r2 = self.radius.powf(2.0);

        println!("{}, {}", r2, x2 + y2);
        x2 + y2 < r2
    }
}

#[cfg(test)]
mod test;
