pub struct Book {
    title: String,
    author: String,
    year: i32,
    genre: Genre,
}

#[derive(Debug)]
pub enum Genre {
    Horror,
    Romance,
    Dystopian,
    Drama,
    Comedy,
    Education,
    History,
    Philosophy,
    NonFiction,
    SciFi,
    Fantasy,
    Mystery,
    Thriller,
    Crime,
    Epic,
    Other,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i32, genre: Genre) -> Book {
        let title = title.to_string();
        let author = author.to_string();
        Book { title, author, year, genre }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn genre(&self) -> &Genre {
        &self.genre
    }

    pub fn print_info(&self) {
        println!("Title: {}  |  Author: {}  |  Year: {}  |  Genre: {}", self.title, self.author, self.year, self.genre);
    }
}

use std::fmt;
impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}