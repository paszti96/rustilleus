//! # Rust Learning Lab library
//!
//! A Rust package is called a **crate**. This file is the root of the library
//! crate, while `main.rs` is the root of the executable program. Keeping most
//! code in a library makes it easy to call from both the program and the tests.
//!
//! Each `pub mod` line below tells Rust to load a module from a file with the
//! same name. For example, `pub mod basics;` loads `src/basics.rs`. `pub` means
//! "public", so `main.rs` and other crates may use that module.
//!
//! Run the binary for printed examples, read the source beside the output, and
//! run `cargo test` to see checks that Rust can execute automatically.

// These declarations form the table of contents for our lessons. They are kept
// alphabetically by topic except that related lessons remain easy to scan.
pub mod algorithms;
pub mod basics;
pub mod collections;
pub mod concurrency;
pub mod errors;
pub mod oop;
pub mod ownership;
pub mod patterns;
