mod book;
mod library;

use library::Library;
use std::io::{self, Write};

fn main() {
    let mut library = Library::new();

    loop {
        println!("\nLibrary Management System");
        println!("1. Add a book");
        println!("2. List books");
        println!("3. Borrow a book");
        println!("4. Return a book");
        println!("5. Remove a book");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let (title, author) = get_book_details();
                library.add_book(&title, &author);
            }
            "2" => {
                library.list_books();
            }
            "3" => {
                let id = get_book_id();
                library.borrow_book(id);
            }
            "4" => {
                let id = get_book_id();
                library.return_book(id);
            }
            "5" => {
                let id = get_book_id();
                library.remove_book(id);
            }
            "6" => {
                println!("Exiting the library system. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter a valid option."),
        }
    }
}

fn get_book_details() -> (String, String) {
    print!("Enter book title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    
    print!("Enter book author: ");
    io::stdout().flush().unwrap();
    let mut author = String::new();
    io::stdin().read_line(&mut author).unwrap();
    
    (title.trim().to_string(), author.trim().to_string())
}

fn get_book_id() -> usize {
    print!("Enter book ID: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    id.trim().parse().unwrap_or(0)
}