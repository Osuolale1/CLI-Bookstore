// Project 7: CLI Bookstore
//
// Summary:
//   Create a command line bookstore management system that handles. yes
//   inventory and checkout operations. This project introduces modular. yes
//   organization, traits, and lifetime management for book data.
//
// User stories:
// * Stage 1:
//   - I want to add books with title, author, price, and stock quantity. yes
//   - I want to view all books in the inventory. yes
// * Stage 2:
//   - I want to update stock levels when books are purchased. partially yes
//   - I want to search for books by title or author. 🫠
// * Stage 3:
//   - I want to process purchases and reduce inventory. yes
//   - I want to see low stock warnings. yes
//
// Tips:
// * Organize code into modules: inventory, checkout, and utils. yes
// * Create a Purchasable trait for items that can be bought. 
// * Use lifetimes on borrowed title descriptions to avoid cloning. 🙂‍↔️
// * A HashMap keyed by book ID will make lookups easier. 🥲
// * Each module should be in its own function for isolation. yes 

use std::io::stdin;
use task7::inventory::*;
use task7::utils::*;

fn main() {
    // println!("Hello, world!");
    let mut new_bookstore = Bookstore::new();
    loop {
        println!("\n=== BOOKSTORE ===");
        println!("1. ADD BOOK ");
        println!("2. ABOUT BOOK ");
        println!("3. UPDATE BOOK PRICE ");
        println!("4. VIEW ALL BOOKS ");
        println!("5. SEARCH  ");
        println!("6. PURCHASE BOOK ");
        println!("0. EXIT");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid Input");

        let user_input: u8 = match input.trim().parse() {
            Ok(user_input) => user_input,
            Err(_) => {
                println!("Invalid Input");
                continue;
            }
        };

        match user_input {
            1 => {
                println!("\n ENTER THE BOOK NAME");
                let book_name = get_input().to_uppercase();

                println!("\n ENTER THE BOOK AUTHOR");
                let book_author = get_input().to_uppercase();

                println!("\n ENTER THE BOOK PRICE");
                let book_price: f64 = match get_input().parse() {
                    Ok(book_price) => book_price,
                    Err(_) => {
                        println!("Invalid Price");
                        continue;
                    }
                };

                println!("\n ENTER THE BOOK QUANTITY");
                let book_stock_quantity: u64 = match get_input().parse() {
                    Ok(quantity) => quantity,
                    Err(_) => {
                        println!("Invalid Quantity");
                        continue;
                    }
                };

                println!("\n ENTER THE BOOK DESCRIPTION");
                let book_description = get_input().to_lowercase();

                println!("\n ENTER THE BOOK FIELD");
                let book_field = match get_input().to_uppercase().as_str() {
                    "SCIENCE" => Field::Science,
                    "ART" => Field::Art,
                    "TECHNOLOGY" => Field::Technology,
                    "FICTION" => Field::Fiction,
                    "RELIGION" => Field::Religion,
                    "POLITICS" => Field::Politics,
                    "FINANCE" => Field::Finance,
                    "LAW" => Field::Law,
                    "MEDICINE" => Field::Medicine,
                    "OTHERS" => Field::Others,
                    _ => {
                        println!(
                            "Invalid Field, \n You Can Enter Others if Field Doesn't Suit Any Of The Listed Ones"
                        );
                        continue;
                    }
                };

                println!("\n ENTER THE BOOK AVAILABILITY");
                let book_availability = match get_input().to_uppercase().as_str() {
                    "AVAILABLE" => Availability::Available,
                    "UNAVAILABLE" => Availability::Unavailable,
                    "OUTOFSTOCK" => Availability::OutOfStock,
                    "LOWSTOCK" => Availability::LowStock,
                    _ => {
                        println!("Invalid Input");
                        continue;
                    }
                };

                let book = Book {
                    title: book_name,
                    author: book_author,
                    price: book_price,
                    stock_quantity: book_stock_quantity,
                    description: book_description,
                    field: book_field,
                    availability: book_availability,
                };

                new_bookstore.add_book(book);
                println!("\n BOOK ADDED SUCCESFULLY ");
            }
            2 => {
                println!("\n ENTER THE BOOK YOU WANT TO VIEW");
                let book_name = get_input();

                if let Some(book) = new_bookstore
                    .bookstore
                    .iter()
                    .find(|b| b.title.to_uppercase() == book_name.to_uppercase())
                {
                    about_book(book, book_name);
                } else {
                    println!("Book not found!");
                }
            }
            3 => {
                println!("\n ENTER THE BOOK YOU WANT TO UPDATE IT PRICE ");
                let book_name = get_input();

                println!("\n ENTER THE NEW PRICE ");
                let price = get_input();

                let price: f64 = match price.parse() {
                    Ok(price) => price,
                    Err(_) => {
                        println!("Invalid PRICE");
                        continue;
                    }
                };

                new_bookstore.update(book_name, price);
            }
            4 => {
                for book in &new_bookstore.bookstore {
                    print_all_book(book);
                }
            }
            5 => {}
            6 => {
                println!("\n ENTER THE TITLE OF THE BOOK YOU WANT TO PURCHASE ");
                let book_name = get_input();

                println!("\n ENTER THE QUANTITY");
                let quantity = get_input();

                let user_quantity: u64 = match quantity.parse() {
                    Ok(user_quantity) => user_quantity,
                    Err(_) => {
                        println!("Invalid Input");
                        continue;
                    }
                };

                new_bookstore.buy_book(book_name, user_quantity);
            }
            0 => {
                println!("EXITING...");
                break;
            }
            _ => {
                println!("\n SELECT AN OPTION BETWEEN 0 - 6 ")
            }
        }
    }
}
