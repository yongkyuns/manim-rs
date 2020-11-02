use nannou::lyon::path::builder::FlatPathBuilder;
use nannou::lyon::path::iterator::PathIterator;
use nannou::lyon::path::Path;
use nannou::lyon::path::PathEvent;
pub trait MeasureLength {
    fn approximate_length(&self, tolerance: f32) -> f32;
}

impl MeasureLength for Path {
    fn approximate_length(&self, tolerance: f32) -> f32 {
        let mut length = 0.0;
        for e in self.iter().flattened(tolerance) {
            match e {
                PathEvent::Line { from, to } => {
                    length += (to - from).length();
                }
                // PathEvent::End {
                //     last,
                //     first,
                //     close: _,
                // } => {
                //     length += (last - first).length();
                // }
                _ => {}
            }
        }
        length
    }
}

pub trait GetPartial: MeasureLength {
    fn upto(&self, ratio: f32, tolerance: f32) -> Path;
}

impl GetPartial for Path {
    fn upto(&self, ratio: f32, tolerance: f32) -> Path {
        if ratio >= 1.0 {
            self.clone()
        } else {
            let ratio = ratio.max(0.0);
            let full_length = self.approximate_length(tolerance);
            let stop_at = ratio * full_length;

            let mut builder = Path::builder();
            let mut length = 0.0;

            for e in self.iter().flattened(tolerance) {
                if length > stop_at {
                    break;
                }
                match e {
                    PathEvent::Begin { at } => {
                        FlatPathBuilder::move_to(&mut builder, at);
                    }
                    PathEvent::Line { from, to } => {
                        let seg_length = (to - from).length();
                        let new_length = length + seg_length;
                        if new_length > stop_at {
                            let seg_ratio = 1.0 - (new_length - stop_at) / seg_length;
                            FlatPathBuilder::line_to(&mut builder, from.lerp(to, seg_ratio));
                            break;
                        } else {
                            length = new_length;
                            FlatPathBuilder::line_to(&mut builder, to);
                        }
                    }
                    PathEvent::End {
                        last: _,
                        first: _,
                        close: _,
                    } => {
                        FlatPathBuilder::close(&mut builder);
                    }
                    _ => (),
                }
            }
            builder.build()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use nannou::geom::rect::Rect;
    use nannou::lyon::path::builder::PathBuilder;
    use nannou::prelude::*;
    #[test]
    fn partial_path() {
        let win_rect = Rect::from_w_h(640.0, 480.0);
        let text = text("Hello").font_size(128).left_justify().build(win_rect);
        let mut builder = Path::builder();
        for e in text.path_events() {
            builder.path_event(e);
        }
        builder.close();
        let path = builder.build();
        let partial_path = path.upto(0.5, 0.01);

        println!("length = {}", partial_path.approximate_length(0.01));
    }
}
