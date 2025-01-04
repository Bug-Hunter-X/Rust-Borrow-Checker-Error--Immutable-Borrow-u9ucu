# Rust Borrow Checker Error: Immutable Borrow

This repository demonstrates a common error encountered when working with references in Rust: attempting to modify a value through an immutable reference.  The Rust compiler's borrow checker prevents this to ensure data integrity and avoid data races.

The `bug.rs` file contains the code exhibiting the error.  The `bugSolution.rs` file shows how to correctly handle the situation using mutable references.

This example highlights the importance of understanding Rust's borrowing rules and how to use mutable references appropriately.