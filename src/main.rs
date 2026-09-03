use std::{env, process::ExitCode};

use rust_learning_lab::{
    algorithms, basics, collections, concurrency, errors, oop, ownership, patterns,
};

fn main() -> ExitCode {
    let lesson = env::args().nth(1).unwrap_or_else(|| "all".to_owned());

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
        "help" | "--help" | "-h" => print_help(),
        unknown => {
            eprintln!("Unknown lesson: {unknown}\n");
            print_help();
            return ExitCode::from(2);
        }
    }

    ExitCode::SUCCESS
}

fn run_all() {
    println!("RUST LEARNING LAB");
    println!("=================\n");
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

fn print_help() {
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
