use nannou::geom::Point2;
use nannou::geom::Rect;
// use nannou::lyon::algorithms::path::math::Point;
use nannou::lyon::algorithms::path::Path;
use nannou::lyon::path::builder::PathBuilder;
use nannou::text::text;
// use nannou::lyon::path::builder::PathBuilder;
#[allow(dead_code)]
pub enum State {
    Transitioning,
    Complete,
}

pub struct VMObject {
    pub path: Path,
    pub points: Vec<Point2>,
    pub state: State,
}
impl VMObject {
    pub fn new() -> Self {
        let path = Path::new();
        let state = State::Complete;
        let points = Vec::new();
        Self {
            path,
            state,
            points,
        }
    }
}

pub struct TextMObject {
    pub inner: VMObject,
    pub text: String,
}

impl TextMObject {
    pub fn new(text: &str, rect: Rect) -> Self {
        let mut inner = VMObject::new();
        inner.path = build_path_from_text(text, rect);
        let text = String::from(text);
        Self { text, inner }
    }
}
fn build_path_from_text(input: &str, rect: Rect) -> Path {
    let text = text(input).font_size(128).left_justify().build(rect);
    let mut builder = Path::builder();
    for e in text.path_events() {
        builder.path_event(e);
    }
    builder.close();
    builder.build()
}
