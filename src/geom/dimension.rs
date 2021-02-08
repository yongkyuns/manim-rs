#[derive(Debug, PartialEq)]
pub struct Dimension {
    width: f32,
    height: f32,
}

impl Dimension {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl GetDimension for Dimension {
    fn dimension(&self) -> &Dimension {
        self
    }
}

impl SetDimension for Dimension {
    fn dimension_mut(&mut self) -> &mut Dimension {
        self
    }
}

pub fn dimension(width: f32, height: f32) -> Dimension {
    Dimension::new(width, height)
}

pub trait GetDimension {
    fn dimension(&self) -> &Dimension;
    fn width(&self) -> f32 {
        self.dimension().width
    }
    fn height(&self) -> f32 {
        self.dimension().height
    }
}

pub trait SetDimension {
    fn dimension_mut(&mut self) -> &mut Dimension;
    fn set_width(&mut self, width: f32) {
        self.dimension_mut().width = width;
    }
    fn set_height(&mut self, height: f32) {
        self.dimension_mut().height = height;
    }
}
