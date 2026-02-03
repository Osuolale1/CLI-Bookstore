use crate::utils::*;

#[derive(Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: f64,
    pub stock_quantity: u64,
    pub description: String,
    pub field: Field,
    pub availability: Availability,
}
#[derive(Clone)]
pub enum Availability {
    Available,
    Unavailable,
    OutOfStock,
    LowStock,
}

#[derive(Clone, Debug)]
pub enum Field {
    Science,
    Art,
    Technology,
    Fiction,
    Religion,
    Politics,
    Finance,
    Law,
    Medicine,
    Others,
}

#[derive(Clone)]
pub struct Bookstore {
    pub bookstore: Vec<Book>,
}

impl Bookstore {
    pub fn new() -> Self {
        Self {
            bookstore: Vec::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.bookstore.push(book);
    }
    pub fn update(&mut self, book_name: String, new_price: f64) {
        if self.bookstore.is_empty() {
            println!("No Book found!");
            return;
        }

        if let Some(book) = self
            .bookstore
            .iter_mut()
            .find(|b| b.title.to_uppercase() == book_name.to_uppercase())
        {
            book.price = new_price;
            println!("Price updated successfully!");
        } else {
            println!("Book '{}' not found!", book_name);
        }
    }

    pub fn buy_book(&mut self, book_name: String, quantity: u64) {
        if self.bookstore.is_empty() {
            println!("No Book found!");
            return;
        }

        if let Some(book) = self
            .bookstore
            .iter_mut()
            .find(|b| b.title.to_uppercase() == book_name.to_uppercase())
        {
            if quantity > book.stock_quantity {
                println!("Insufficient Copies");
                return;
            }
            amount(book, quantity);
            book.stock_quantity -= quantity;
            notify(book);
        } else {
            println!("Book '{}' not found!", book_name);
        }
    }
}
