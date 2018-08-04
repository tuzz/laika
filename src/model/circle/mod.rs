use super::point::Point;

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn contains(&self, point: Point) -> bool {
        let x = self.center.x - point.x;
        let y = self.center.y - point.y;

        let x2 = x.powf(2.0);
        let y2 = y.powf(2.0);
        let r2 = self.radius.powf(2.0);

        x2 + y2 < r2
    }
}

#[cfg(test)]
mod test;
