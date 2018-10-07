pub struct Matrix {
    pub array: [f32; 9],
}

impl Matrix {
    pub fn new(array: [f32; 9]) -> Self {
        Self { array }
    }

    pub fn identity() -> Self {
        Self::new([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    pub fn translation(tx: f32, ty: f32) -> Self {
        Self::new([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            tx,  ty,  1.0,
        ])
    }

    pub fn rotation(radians: f32) -> Self {
        let sin = radians.sin();
        let cos = radians.cos();

        Self::new([
            cos, -sin, 0.0,
            sin,  cos, 0.0,
            0.0,  0.0, 1.0,
        ])
    }

    pub fn scalar(sx: f32, sy: f32) -> Self {
        Self::new([
            sx,  0.0, 0.0,
            0.0, sy,  0.0,
            0.0, 0.0, 1.0,
        ])
    }

    pub fn translate(&self, tx: f32, ty: f32) -> Self {
        self.multiply(&Self::translation(tx, ty))
    }

    pub fn rotate(&self, radians: f32) -> Self {
        self.multiply(&Self::rotation(radians))
    }

    pub fn scale(&self, sx: f32, sy: f32) -> Self {
        self.multiply(&Self::scalar(sx, sy))
    }

    // Project 0 -> 1 to WebGL clipspace, with y=0 at the top.
    pub fn project(&self) -> Self {
        self.translate(-0.5, -0.5).scale(2.0, -2.0)
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let a = self.array;
        let b = other.array;

        Self::new([
            // First row:
            a[0] * b[0]    +    a[1] * b[3]    +    a[2] * b[6],
            a[0] * b[1]    +    a[1] * b[4]    +    a[2] * b[7],
            a[0] * b[2]    +    a[1] * b[5]    +    a[2] * b[8],

            // Second row:
            a[3] * b[0]    +    a[4] * b[3]    +    a[5] * b[6],
            a[3] * b[1]    +    a[4] * b[4]    +    a[5] * b[7],
            a[3] * b[2]    +    a[4] * b[5]    +    a[5] * b[8],

            // Third row:
            a[6] * b[0]    +    a[7] * b[3]    +    a[8] * b[6],
            a[6] * b[1]    +    a[7] * b[4]    +    a[8] * b[7],
            a[6] * b[2]    +    a[7] * b[5]    +    a[8] * b[8],
        ])
    }
}

#[cfg(test)]
mod test;
