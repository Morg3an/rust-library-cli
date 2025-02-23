#[derive(Debug, Clone)]
pub struct Book {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}

impl Book {
    pub fn new(id: usize, title: &str, author: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            author: author.to_string(),
            is_borrowed: false,
        }
    }
}