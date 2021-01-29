pub trait Visibility {
    fn visible_mut(&mut self) -> &mut bool;
    fn is_visible(&self) -> bool;
    fn show(&mut self) {
        *self.visible_mut() = true;
    }
    fn hide(&mut self) {
        *self.visible_mut() = false;
    }
}
