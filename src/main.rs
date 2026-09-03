// `use` brings names into the current file so we can write `env::args()` instead
// of the longer `std::env::args()`. `std` is Rust's standard library.
use std::{env, process::ExitCode};

// This imports the lesson modules from our library crate. Curly braces let one
// `use` statement import several names from the same place.
use rust_learning_lab::{
    algorithms, basics, collections, concurrency, errors, oop, ownership, patterns,
};

/// The operating system starts the program by calling `main`.
///
/// Returning `ExitCode` lets us report success (code 0) or a command-line error
/// (code 2) to the terminal. Programs and scripts use these numbers to decide
/// whether the previous command worked.
fn main() -> ExitCode {
    // `env::args()` produces an iterator over command-line words. The program's
    // own path is item 0, so `.nth(1)` asks for the first user argument.
    //
    // The result is `Option<String>`: `Some(text)` when an argument exists or
    // `None` when it does not. `unwrap_or_else` safely creates "all" for `None`.
    let lesson = env::args().nth(1).unwrap_or_else(|| "all".to_owned());

    // `match` compares one value with several patterns. Rust checks that every
    // possible value is handled. Each `=>` points from a pattern to its action.
    // `as_str()` lets us compare the owned `String` with short string slices.
    match lesson.as_str() {
        "all" => run_all(),
        "basics" => basics::run(),
        "ownership" => ownership::run(),
        "oop" => oop::run(),
        "collections" => collections::run(),
        "algorithms" => algorithms::run(),
        "patterns" => patterns::run(),
        "errors" => errors::run(),
        "concurrency" => concurrency::run(),
        // The `|` means "or", so all three spellings display the same help.
        "help" | "--help" | "-h" => print_help(),
        // Any text not matched above is captured in a new variable named
        // `unknown`. This final arm makes the match exhaustive.
        unknown => {
            // `eprintln!` writes errors to the terminal's error stream. The `!`
            // tells us that this is a macro, not an ordinary function.
            eprintln!("Unknown lesson: {unknown}\n");
            print_help();
            return ExitCode::from(2);
        }
    }

    // Reaching this point means the requested lesson ran without an error.
    // This final expression has no semicolon, so it becomes the return value.
    ExitCode::SUCCESS
}

/// Runs every lesson in a sensible order.
///
/// A function without a written return type returns the unit value `()`, which
/// is Rust's way of saying "there is no useful result to return."
fn run_all() {
    // `println!` prints text followed by a newline. `\n` adds another newline.
    println!("RUST LEARNING LAB");
    println!("=================\n");

    // The `::` operator selects an item inside a module. Every lesson exposes a
    // function called `run`, but their module names keep those functions apart.
    basics::run();
    ownership::run();
    oop::run();
    collections::run();
    algorithms::run();
    patterns::run();
    errors::run();
    concurrency::run();
    println!("Tour complete. Try `cargo test` next.");
}

/// Prints command-line instructions without running a lesson.
fn print_help() {
    // A backslash at the end of each source line continues one long string while
    // keeping the code readable. The spaces are part of the displayed layout.
    println!(
        "Rust Learning Lab\n\n\
         Usage:\n\
           cargo run                 Run the complete tour\n\
           cargo run -- <lesson>     Run one lesson\n\n\
         Lessons:\n\
           basics       Variables, types, functions, and control flow\n\
           ownership    Ownership, borrowing, slices, and lifetimes\n\
           oop          Structs, methods, traits, composition, polymorphism\n\
           collections  Vec, maps, sets, queues, and heaps\n\
           algorithms   Search, sort, graph traversal, dynamic programming\n\
           patterns     Builder, Strategy, newtype, and enum state\n\
           errors       Option, Result, custom errors, and the ? operator\n\
           concurrency  Threads, channels, Arc, and Mutex\n\
           all          Run every lesson (the default)\n"
    );
}
