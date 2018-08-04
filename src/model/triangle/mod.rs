use super::point::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
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
}

#[cfg(test)]
mod test;
