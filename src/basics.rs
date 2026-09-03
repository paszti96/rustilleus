//! Variables, built-in types, functions, expressions, loops, enums, and matching.

/// Runs the examples in this lesson.
pub fn run() {
    println!("1. BASICS");

    // Variables are immutable unless `mut` is present.
    let language = "Rust";
    let release_year = 2015;
    let mut lessons_completed = 0;
    lessons_completed += 1;

    // Shadowing creates a new variable and may change its type.
    let spaces = "   ";
    let spaces = spaces.len();

    // Common scalar types: integer, floating point, boolean, and character.
    let score: i32 = 42;
    let ratio: f64 = 1.5;
    let is_fast: bool = true;
    let crab: char = '🦀';

    // Tuples can mix types; arrays have a fixed length and one element type.
    let point: (i32, i32) = (3, 4);
    let (x, y) = point;
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

    // `if` and `match` are expressions: they return values.
    let parity = if score % 2 == 0 { "even" } else { "odd" };
    println!(
        "   {score} is {parity}; today feels {}",
        temperature_label(22)
    );

    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    for direction in directions {
        println!("   move: {}", direction.instruction());
    }

    let doubled: Vec<i32> = (1..=4).map(double).collect();
    println!("   function + iterator result: {doubled:?}\n");
}

/// A function declares parameter and return types. The final expression has no semicolon.
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub fn double(value: i32) -> i32 {
    value * 2
}

pub fn temperature_label(celsius: i32) -> &'static str {
    match celsius {
        ..=0 => "freezing",
        1..=15 => "cold",
        16..=25 => "pleasant",
        _ => "hot",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn instruction(self) -> &'static str {
        match self {
            Self::North => "increase y",
            Self::East => "increase x",
            Self::South => "decrease y",
            Self::West => "decrease x",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_hypotenuse() {
        assert_eq!(hypotenuse(3.0, 4.0), 5.0);
    }

    #[test]
    fn classifies_temperature_boundaries() {
        assert_eq!(temperature_label(0), "freezing");
        assert_eq!(temperature_label(16), "pleasant");
        assert_eq!(temperature_label(26), "hot");
    }
}
