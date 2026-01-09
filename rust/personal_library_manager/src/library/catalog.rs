use super::book::Book;
use std::collections::HashMap;

pub struct Catalog {
    books: HashMap<String, Book>,
}

impl Catalog {
    pub fn new() -> Catalog {
        Catalog {
            books: HashMap::new(),
        }
    }
    pub fn add_book(&mut self, book: Book) {
        let title = book.title().clone();
        self.books.insert(title, book);
    }
    pub fn remove_book(&mut self, title: &str) -> Result<(), String> {
        match self.books.remove(title) {
            Some(_) => Ok(()),
            None => Err("couldn't find book".to_string()),
        }
    }
    pub fn list_books(&self) {
        for book in self.books.values() {
            book.print_info();
        }
    }
}
