pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

#[cfg(test)]
mod tests {
    use crate::Post;

    #[test]
    fn new_post_has_empty_content() {
        let p = Post::new();
        assert_eq!("", p.content(), "New draft post should have no content");
    }
}