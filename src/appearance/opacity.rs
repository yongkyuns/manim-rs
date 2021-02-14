#[derive(Debug, PartialEq)]
pub struct Opacity {
    is_visible: bool,
    alpha: f32,
}

impl Opacity {
    pub fn new(is_visible: bool) -> Self {
        let mut alpha = 0.0;
        if is_visible {
            alpha = 1.0;
        }
        Self { is_visible, alpha }
    }
}

impl GetOpacity for Opacity {
    fn opacity(&self) -> f32 {
        self.alpha
    }
    fn is_visible(&self) -> bool {
        self.is_visible
    }
}

impl SetOpacity for Opacity {
    fn opacity_mut(&mut self) -> &mut Opacity {
        self
    }
}

pub trait GetOpacity {
    fn opacity(&self) -> f32;
    fn alpha(&self) -> f32 {
        self.opacity()
    }
    fn is_visible(&self) -> bool;
}

pub trait SetOpacity: GetOpacity {
    fn opacity_mut(&mut self) -> &mut Opacity;
    fn show(&mut self) {
        self.opacity_mut().is_visible = true;
        self.opacity_mut().alpha = 1.0;
    }
    fn hide(&mut self) {
        self.opacity_mut().is_visible = false;
        self.opacity_mut().alpha = 0.0;
    }
    fn set_opacity(&mut self, alpha: f32) {
        if alpha > 0.0 {
            self.opacity_mut().is_visible = true;
            self.opacity_mut().alpha = alpha;
        } else {
            self.hide();
        }
    }
    fn set_alpha(&mut self, alpha: f32) {
        self.set_opacity(alpha);
    }
}
