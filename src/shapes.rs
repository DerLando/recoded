#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Shapes {
    Circle(piet::kurbo::Circle),
}

impl Default for Shapes {
    fn default() -> Self {
        Self::Circle(piet::kurbo::Circle::default())
    }
}

impl Shapes {
    pub fn get_shape(&self) -> &impl piet::kurbo::Shape {
        match self {
            Shapes::Circle(shape) => shape,
        }
    }
}
