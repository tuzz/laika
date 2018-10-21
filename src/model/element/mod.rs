use ::util::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub name: String,
    pub color: Color,
}

impl Element {
    pub fn new(name: String, red: f32, green: f32, blue: f32) -> Self {
        Self { name, color: Color { red, green, blue } }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::iron(), Self::copper()]
    }

    pub fn iron() -> Self {
        Self::new("Iron".to_string(), 1.0, 0.0, 0.0)
    }

    pub fn copper() -> Self {
        Self::new("Copper".to_string(), 0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod test;
