use ::util::Color;

struct Element {
    name: String,
    color: Color,
}

impl Element {
    fn new(name: String, red: f32, green: f32, blue: f32) -> Self {
        Self { name, color: Color { red, green, blue } }
    }

    fn all() -> Vec<Self> {
        vec![Self::iron(), Self::copper()]
    }

    fn iron() -> Self {
        Self::new("Iron".to_string(), 1.0, 0.0, 0.0)
    }

    fn copper() -> Self {
        Self::new("Copper".to_string(), 0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod test;
