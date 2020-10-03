use nannou;
pub trait Draw {
    fn draw(&self, draw: nannou::Draw);
}
