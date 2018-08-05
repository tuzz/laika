use super::point::Point;
use super::circle::Circle;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    pub fn area(&self) -> f64 {
        let (a, b, c) = (self.a, self.b, self.c);

        let g = a.x * (b.y - c.y);
        let h = b.x * (c.y - a.y);
        let i = c.x * (a.y - b.y);

        (g + h + i) / 2.0
    }

    pub fn centroid(&self) -> Point {
        let (a, b, c) = (self.a, self.b, self.c);

        let x = (a.x + b.x + c.x) / 3.0;
        let y = (a.y + b.y + c.y) / 3.0;

        Point::new(x, y)
    }

    pub fn bounding_circle(&self) -> Circle {
        let points = &[self.a, self.b, self.c];
        let center = self.centroid();

        let distances = points.iter().map(|p| p.distance(center));
        let furthest = distances.max_by(|a, b| a.partial_cmp(b).unwrap());

        Circle::new(center, furthest.unwrap())
    }
}

#[cfg(test)]
mod test;
