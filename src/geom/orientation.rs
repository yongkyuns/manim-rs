pub trait SetOrientation: GetOrientation {
    fn orientation_mut(&mut self) -> &mut f32;
    fn rotate_by(&mut self, angle: f32) {
        *self.orientation_mut() += angle;
    }
    fn rotate_to(&mut self, angle: f32) {
        *self.orientation_mut() = angle;
    }
    fn look_at(&mut self, angle: f32) {
        *self.orientation_mut() = angle;
    }
}
pub trait GetOrientation {
    fn orientation(&self) -> f32;
}
