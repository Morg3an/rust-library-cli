# 📚 Rust Library CLI

A **Rust-based CLI Library Management System** that allows users to add, list, borrow, return, and remove books through an interactive terminal interface.

## 🚀 Features
- **Add Books** 📖
- **List Books** 📚
- **Borrow & Return Books** 🔄
- **Remove Books** 🗑️
- **Interactive CLI Menu** 🎛️
- **Memory-Safe Borrowing** ✅

## 📂 Project Structure
```
rust-library-cli/
│── src/
│   │── main.rs        // Entry point with CLI logic
│   │── library.rs     // Library struct with CRUD functions
│   │── book.rs        // Book struct definition
│── Cargo.toml         // Dependencies
│── README.md          // Project documentation
```

## 🛠️ Installation
1. **Clone the repository:**
```sh
git clone https://github.com/Morg3an/rust-library-cli.git
cd rust-library-cli
```
2. **Build the project:**
```sh
cargo build
```
3. **Run the CLI application:**
```sh
cargo run
```

## 📖 Usage Example
```
Library Management System
1️. Add a book
2️. List books
3️. Borrow a book
4️. Return a book
5️. Remove a book
6️. Exit
Enter your choice: 1

Enter book title: Rust in Action
Enter book author: Tim McNamara
Book added successfully!

Library Management System
1️. Add a book
2️. List books
3️. Borrow a book
4️. Return a book
5️. Remove a book
6️. Exit
Enter your choice: 2

Available Books:
[1] Rust in Action by Tim McNamara - Available
```

## 📜 License
This project is licensed under the MIT License.

## 🤝 Contributing
Contributions are welcome! Feel free to fork this repository, make changes, and submit a pull request.

## 📬 Contact
For any issues or suggestions, create an issue or reach out via [GitHub](https://github.com/Morg3an).