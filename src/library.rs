use crate::book::Book;

pub struct Library {
    books: Vec<Book>,
    next_id: usize,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_book(&mut self, title: &str, author: &str) {
        let book = Book::new(self.next_id, title, author);
        self.books.push(book);
        println!("Book added successfully!");
        self.next_id += 1;
    }

    pub fn list_books(&self) {
        if self.books.is_empty() {
            println!("The library is empty!");
            return;
        }
        println!("\nAvailable Books:");
        for book in &self.books {
            println!(
                "[{}] {} by {} - {}",
                book.id,
                book.title,
                book.author,
                if book.is_borrowed { "Borrowed" } else { "Available" }
            );
        }
    }

    pub fn borrow_book(&mut self, id: usize) {
        if let Some(book) = self.books.iter_mut().find(|b| b.id == id) {
            if book.is_borrowed {
                println!("This book is already borrowed!");
            } else {
                book.is_borrowed = true;
                println!("You have borrowed: {}", book.title);
            }
        } else {
            println!("Book ID not found!");
        }
    }

    pub fn return_book(&mut self, id: usize) {
        if let Some(book) = self.books.iter_mut().find(|b| b.id == id) {
            if book.is_borrowed {
                book.is_borrowed = false;
                println!("You have returned: {}", book.title);
            } else {
                println!("This book was not borrowed!");
            }
        } else {
            println!("Book ID not found!");
        }
    }

    pub fn remove_book(&mut self, id: usize) {
        if let Some(index) = self.books.iter().position(|b| b.id == id) {
            self.books.remove(index);
            println!("Book removed from the library!");
        } else {
            println!("Book ID not found!");
        }
    }
}