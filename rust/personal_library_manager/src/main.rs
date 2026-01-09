mod library;
use library::{
    book::{Book, Genre},
    catalog::Catalog,
};

fn main() {
    println!("Hello, world!");
    let books = vec![
        Book::new("Crime and Punishment", "Fyodor Dostoevsky", 1866, Genre::Crime),
        Book::new("1984", "George Orwell", 1949, Genre::Dystopian),
        Book::new("Blood Meridian", "Cormac McCarthy", 1985, Genre::Epic),
    ];
    let mut catalog = Catalog::new();
    for book in books.into_iter() {
        catalog.add_book(book);
    }
    catalog.list_books();
}
