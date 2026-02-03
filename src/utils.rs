use crate::checkout::*;
use std::io::stdin;

////////////// HELPER FUNCTIONS ///////////////
pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid Input");

    let user_input = input.trim();

    user_input.to_string()
}

pub fn about_book(main: &impl Summary, book_name: String) {
    let new = main.summary(book_name);
    println!("{}", new)
}

pub fn amount(main: &impl Notification, quantity: u64) {
    let new = main.amount(quantity);
    println!("{}", new)
}

pub fn notify(main: &impl Notification) {
    let new = main.notify();
    println!("{}", new)
}

pub fn print_all_book(main: &impl Summary) {
    let new = main.all_book();
    println!("{}", new)
}
