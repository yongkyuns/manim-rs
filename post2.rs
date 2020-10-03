pub enum State {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: State,
    content: String,
}

impl Post {
    fn new() -> Self {
        Self {
            state: State::Draft,
            content: String::new(),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

impl Editorial for Post {
    fn request_review(&mut self) {
        if let State::Draft = self.state {
            self.state = State::PendingReview;
        }
    }
    fn approve(&mut self) {
        if let State::PendingReview = self.state {
            self.state = State::Published;
        }
    }
    fn content(&self) -> &str {
        if let State::Published = self.state {
            &self.content
        } else {
            ""
        }
    }
}

trait Editorial {
    fn request_review(&mut self);
    fn approve(&mut self);
    fn content(&self) -> &str;
}

fn main() {
    let mut post = Post::new();

    post.add_text("Some content");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Some content", post.content());
}
