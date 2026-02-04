# CLI Bookstore — v1
Simple Command-Line Bookstore Management System (Rust)

## Overview
Bookstore is a CLI-based bookstore management system built in Rust. It demonstrates core inventory and checkout mechanisms:

- A modular architecture with separate concerns
- Trait-based extensibility for book operations
- Stock management with purchase validation
- Search functionality with input normalization

This version (v1) focuses on basic CRUD operations and forms a foundation for future expansions (database persistence, user accounts, reporting, etc).

## Features
- Add books with full metadata
- View individual book details
- Update book pricing
- View all inventory
- Search by title or author
- Purchase with stock validation
- Low stock warnings
- Purchasable/non-purchasable book support

## Data Model

### Book
| Field            | Type         | Description                          |
|------------------|--------------|--------------------------------------|
| title            | String       | Book title                           |
| author           | String       | Author name                          |
| price            | f64          | Price in dollars                     |
| stock_quantity   | u64          | Available copies                     |
| description      | String       | Book summary                         |
| field            | Field        | Category (Science, Art, Fiction...)  |
| availability     | Availability | Stock status enum                    |
| to_be_sold       | bool         | Whether book is purchasable          |

### Field (Enum)
```
Science | Art | Technology | Fiction | Religion | Politics | Finance | Law | Medicine | Others
```

### Availability (Enum)
```
Available | Unavailable | OutOfStock | LowStock
```

## Traits

### Summary
```rust
fn summary(&self, book_name: String) -> String;
fn all_book(&self) -> String;
```

### Notification
```rust
fn notify(&self) -> String;
fn amount(&self, quantity: u64) -> String;
```

### Purchasable
```rust
fn purchasable(&self) -> bool;
```

## Menu Operations

| Option | Action              | Description                              |
|--------|---------------------|------------------------------------------|
| 1      | ADD BOOK            | Add a new book to inventory              |
| 2      | ABOUT BOOK          | View details of a specific book          |
| 3      | UPDATE BOOK PRICE   | Modify price of existing book            |
| 4      | VIEW ALL BOOKS      | List entire inventory                    |
| 5      | SEARCH BY TITLE     | Find book by title (case-insensitive)    |
| 6      | SEARCH BY AUTHOR    | Find book by author (space-normalized)   |
| 7      | PURCHASE BOOK       | Buy copies with stock validation         |
| 0      | EXIT                | Close application                        |

## Security Model
This bookstore uses input validation:

- Price and quantity parsing with error handling
- Case-insensitive searches
- Whitespace normalization for author searches
- Purchase confirmation before stock deduction
- Non-purchasable book protection

## Architecture

### Module Structure
```
src/
├── main.rs        # CLI loop & user interaction
├── lib.rs         # Module exports
├── inventory.rs   # Book, Bookstore structs & methods
├── checkout.rs    # Traits: Summary, Notification, Purchasable
└── utils.rs       # Helper functions (get_input, printers)
```

### Data Flow
```
User Input → main.rs → Bookstore methods → Trait implementations → Output
```

## Testing
Run the application with:
```bash
cargo run
```

Build the project:
```bash
cargo build
```

Release build:
```bash
cargo build --release
```

## Toolchain Requirements

| Tool   | Version |
|--------|---------|
| Rust   | 1.70+   |
| Cargo  | 1.70+   |
| Edition| 2021    |

Ensure you have the latest stable Rust:
```bash
rustup default stable
rustup update
```

## Future Roadmap

### Planned v2 — Data Persistence
- File-based storage (JSON/CSV)
- Load inventory on startup
- Save changes automatically
- Backup/restore functionality

### Planned v3 — Enhanced Search
- HashMap keyed by book ID for O(1) lookups
- Fuzzy search support
- Filter by field/availability
- Sort options (price, title, stock)

### Planned v4 — Lifetime Optimization
- Borrowed title descriptions
- Reduced cloning overhead
- Memory-efficient iterations

### Planned v5 — Advanced Features
- User accounts & authentication
- Purchase history tracking
- Sales reporting
- Inventory alerts
- REST API interface

## User Stories Completed

### Stage 1
- [x] Add books with title, author, price, and stock quantity
- [x] View all books in the inventory

### Stage 2
- [x] Update stock levels when books are purchased
- [x] Search for books by title or author

### Stage 3
- [x] Process purchases and reduce inventory
- [x] See low stock warnings

## Status
**Version:** v1 — CLI Bookstore Management
**Maturity:** Educational / Learning Project

## Built With
- Rust
- Standard Library (std::io)

## Project Layout
```
task7/
├── Cargo.toml      # Package manifest
├── Cargo.lock      # Dependency lock
├── README.md       # This file
└── src/
    ├── main.rs     # Entry point & CLI
    ├── lib.rs      # Library root
    ├── inventory.rs# Data structures
    ├── checkout.rs # Trait definitions
    └── utils.rs    # Utilities
```

## License
MIT
