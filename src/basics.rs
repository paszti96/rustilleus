//! # Lesson 1: Rust basics
//!
//! This lesson introduces the smallest building blocks of a Rust program:
//! variables store values, types describe what kind of values they are, functions
//! group reusable work, and control-flow tools choose what runs next.
//!
//! Rust is **statically typed**, meaning the compiler knows every value's type
//! before the program runs. Rust can often infer a type, but we may also write it
//! explicitly when that makes the code clearer.

/// Runs and prints all examples in the basics lesson.
pub fn run() {
    println!("1. BASICS");

    // `let` creates a variable. Variables are immutable (cannot be changed) by
    // default. This helps prevent accidental changes later in the program.
    let language = "Rust";
    let release_year = 2015;

    // Add `mut`, short for "mutable", when a variable really needs to change.
    // Rust uses `+=` as a short form of `lessons_completed = lessons_completed + 1`.
    let mut lessons_completed = 0;
    lessons_completed += 1;

    // Shadowing means declaring a new variable with the same name. The first
    // `spaces` is text; the second is a number. This differs from mutation because
    // the old variable is replaced and the new variable may have another type.
    let spaces = "   ";
    let spaces = spaces.len();

    // A scalar is one simple value. `i32` is a signed 32-bit whole number, `f64`
    // is a 64-bit decimal number, `bool` is true/false, and `char` is one Unicode
    // character. Type annotations follow a colon, as in `score: i32`.
    let score: i32 = 42;
    let ratio: f64 = 1.5;
    let is_fast: bool = true;
    let crab: char = '🦀';

    // A tuple groups a fixed number of values and can mix their types. Here both
    // values happen to be i32. "Destructuring" gives tuple parts the names x and y.
    let point: (i32, i32) = (3, 4);
    let (x, y) = point;

    // An array has a fixed length and one element type. A slice such as `[1..3]`
    // is a borrowed view into part of an array. The start is included and the end
    // is excluded, so this selects indexes 1 and 2: the values 20 and 30.
    let numbers = [10, 20, 30, 40];
    let middle: &[i32] = &numbers[1..3];

    println!("   {language} has been stable since {release_year}.");
    println!("   mutable progress: {lessons_completed}; shadowed data: {spaces}");
    println!("   scalars: {score}, {ratio}, {is_fast}, {crab}");
    println!(
        "   point ({x}, {y}) has length {:.1}",
        hypotenuse(x as f64, y as f64)
    );
    println!("   borrowed array slice: {middle:?}");

    // `if` is an expression, which means it can produce a value. Both branches
    // must produce the same type; here each branch produces a string slice.
    let parity = if score % 2 == 0 { "even" } else { "odd" };
    println!(
        "   {score} is {parity}; today feels {}",
        temperature_label(22)
    );

    // An array can be used directly in a `for` loop. Each trip through the loop
    // moves one Direction value into the `direction` variable. Direction is Copy,
    // so copying its tiny value is inexpensive.
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    for direction in directions {
        println!("   move: {}", direction.instruction());
    }

    // `1..=4` creates the values 1 through 4 (the `=` includes 4). `map` calls
    // `double` for each value, and `collect` gathers the results into a `Vec`, a
    // growable list. This is an example of an iterator pipeline.
    let doubled: Vec<i32> = (1..=4).map(double).collect();
    println!("   function + iterator result: {doubled:?}\n");
}

/// Calculates the longest side of a right triangle.
///
/// A function declares each parameter's type and places its return type after
/// `->`. The final expression has no semicolon, so its value is returned.
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    // `powi(2)` squares a number and `sqrt()` takes the square root. This is the
    // Pythagorean theorem: c = square_root(a squared + b squared).
    (a.powi(2) + b.powi(2)).sqrt()
}

/// Returns an integer multiplied by two.
///
/// The input is passed by value. An i32 is small and implements `Copy`, so using
/// it here copies the number instead of taking it away from the caller.
pub fn double(value: i32) -> i32 {
    value * 2
}

/// Describes a temperature using a string stored for the entire program.
///
/// `&str` is a borrowed piece of text. The `'static` lifetime means these string
/// literals are built into the program and remain valid until it exits.
pub fn temperature_label(celsius: i32) -> &'static str {
    // Each match arm covers a range. `..=0` means everything up to and including
    // zero, while `_` is a catch-all for every value not matched earlier.
    match celsius {
        ..=0 => "freezing",
        1..=15 => "cold",
        16..=25 => "pleasant",
        _ => "hot",
    }
}

// An enum lists every allowed form of a value. Unlike a free-form string, a
// Direction cannot contain a typo such as "Nrth". `derive` asks the compiler to
// generate useful trait implementations: Debug printing, cloning/copying, and
// equality comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Converts a direction into a simple instruction.
    ///
    /// `self` means the Direction on the left side of the method call. Writing
    /// `Direction::North.instruction()` passes North as `self`.
    pub fn instruction(self) -> &'static str {
        // Because every enum variant is listed, adding a fifth Direction later
        // will cause a helpful compiler error until this match is updated.
        match self {
            Self::North => "increase y",
            Self::East => "increase x",
            Self::South => "decrease y",
            Self::West => "decrease x",
        }
    }
}

// This whole module is compiled only by `cargo test`. Keeping tests close to the
// code makes it easy to see the expected behavior while learning.
#[cfg(test)]
mod tests {
    // Import every public and private name from the parent module.
    use super::*;

    // `#[test]` tells Rust's test runner to execute the following function.
    #[test]
    fn calculates_hypotenuse() {
        // `assert_eq!` passes when its two arguments are equal and reports both
        // values when they differ.
        assert_eq!(hypotenuse(3.0, 4.0), 5.0);
    }

    #[test]
    fn classifies_temperature_boundaries() {
        assert_eq!(temperature_label(0), "freezing");
        assert_eq!(temperature_label(16), "pleasant");
        assert_eq!(temperature_label(26), "hot");
    }
}
