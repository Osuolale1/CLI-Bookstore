use crate::inventory::Book;

pub trait Notification {
    fn notify(&self) -> String;
    fn amount(&self, quantity: u64) -> String;
}

pub trait Purchasable {
    fn purchasable(&self) -> bool;
}
pub trait Summary {
    fn summary(&self, book_name: String) -> String;
    fn all_book(&self) -> String;
}

impl Purchasable for Book {
    fn purchasable(&self) -> bool {
        self.to_be_sold
    }
}

impl Summary for Book {
    fn summary(&self, book_name: String) -> String {
        if self.title.to_lowercase() == book_name.to_lowercase() {
            format!(
                "{} by {} talking about {}. This is a Great Book in the {:?} field.",
                self.title.to_uppercase(),
                self.author.to_uppercase(),
                self.description,
                self.field
            )
        } else {
            format!("{} NOT FOUND", book_name,)
        }
    }

    fn all_book(&self) -> String {
        format!(
            "{:?} by {:?} talking about {:?}. This is a Great Book in the {:?} field.",
            self.title.to_uppercase(),
            self.author.to_uppercase(),
            self.description,
            self.field
        )
    }
}

impl Notification for Book {
    fn notify(&self) -> String {
        if self.stock_quantity == 0 {
            format!(
                "\n {} by {} has finished,  {} copies left.",
                self.title.to_uppercase(),
                self.author.to_uppercase(),
                self.stock_quantity,
            )
        } else if self.stock_quantity <= 10 {
            format!(
                "\n {} by {} is about to finished, There's currently {} copies available.",
                self.title.to_uppercase(),
                self.author.to_uppercase(),
                self.stock_quantity,
            )
        } else {
            format!(
                "{} copies {} left",
                self.stock_quantity,
                self.title.to_uppercase(),
            )
        }
    }

    fn amount(&self, quantity: u64) -> String {
        format!(
            "\n Dear Prospective Reader, you're about to spend ${} on {} by {}",
            self.price * quantity as f64,
            self.title.to_uppercase(),
            self.author.to_uppercase(),
        )
    }
}
