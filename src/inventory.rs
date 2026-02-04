use crate::{checkout::Purchasable, utils::*};

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: f64,
    pub stock_quantity: u64,
    pub description: String,
    pub field: Field,
    pub availability: Availability,
    pub to_be_sold: bool,
}
#[derive(Debug, Clone)]
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
            println!("\n No Book found!");
            return;
        }

        if let Some(book) = self
            .bookstore
            .iter_mut()
            .find(|b| b.title.to_uppercase() == book_name.to_uppercase())
        {
            book.price = new_price;
            println!("\n Price updated successfully!");
        } else {
            println!("\n '{}' Book Not Found!", book_name);
        }
    }

    pub fn buy_book(&mut self, book_name: String, quantity: u64) {
        if self.bookstore.is_empty() {
            println!("\n No Book found!");
            return;
        }

        if let Some(book) = self
            .bookstore
            .iter_mut()
            .find(|b| b.title.to_uppercase() == book_name.to_uppercase())
        {
            if quantity == 0 {
                println!("\n Cant' Buy 0 Book");
                return;
            }

            if !book.purchasable() {
                println!("\n Dear Reader, {}, is not meant for sell, you can read the E-format on our Site. \n Thanks for your understanding.", book.title);
                return;
            }

            if quantity > book.stock_quantity {
                println!(
                    "\n Insufficient Copies, We only have {} copies left.",
                    book.stock_quantity
                );
                return;
            }
            amount(book, quantity);
            println!("\n DO YOU WANT TO CONTINUE WITH YOUR PURCAHSE, YES or NO TO CONTINUE");
            let _choice = match get_input().to_lowercase().as_str() {
                "yes" => {
                    book.stock_quantity -= quantity;
                    println!("\n PURCHASED SUCCESSFULLY");
                    notify(book);
                }
                "no" => {
                    println!("\n You're No longer buying.");
                    return;
                }
                _ => {
                    println!("\n Invalid Input");
                    return;
                }
            };
        } else {
            println!("Book '{}' Not Found!", book_name);
        }
    }

    pub fn search_book_by_title(&self, book_name: String) {
        if let Some(book) = self
            .bookstore
            .iter()
            .find(|s| s.title.to_lowercase() == book_name.to_lowercase())
        {
            println!("Found Book: {:?}", book);
        } else {
            println!("Book Not found");
        }
    }

    pub fn search_book_by_author(&self, author_name: String) {
        let name: String = author_name.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(book) = self
            .bookstore
            .iter()
            .find(|b| b.author.to_lowercase() == name.to_lowercase())
        {
            println!("Found Book: {:?}", book);
        } else {
            println!("Book Not found");
        }
    }
}
