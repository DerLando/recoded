#[derive(serde::Serialize, serde::Deserialize)]
pub enum Shapes {
    Circle(piet::kurbo::Circle),
}

impl Shapes {
    pub fn get_shape(&self) -> &impl piet::kurbo::Shape {
        match self {
            Shapes::Circle(shape) => shape,
        }
    }
}
