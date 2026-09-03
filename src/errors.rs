//! # Lesson 7: Error handling
//!
//! Programs meet invalid input, missing data, and failed operations. Rust makes
//! these possibilities visible in function return types instead of relying on
//! null values or exceptions:
//!
//! - `Option<T>` is `Some(T)` when a value exists or `None` when it does not.
//! - `Result<T, E>` is `Ok(T)` on success or `Err(E)` on failure.
//!
//! A panic stops the current thread and is best reserved for bugs or impossible
//! states. Expected problems, such as a user typing a word instead of a number,
//! should normally return Option or Result so the caller can decide what to do.

// Error is the standard trait for error types. `fmt` supplies formatting tools
// used to turn our custom error values into readable sentences.
use std::{error::Error, fmt};

/// Runs and prints the error-handling examples.
pub fn run() {
    println!("7. ERROR HANDLING");

    // `match` forces us to handle both shapes of Option instead of assuming that
    // division succeeded. There is no null value to cause a surprise later.
    match divide(10.0, 2.0) {
        Some(value) => println!("   Option success: 10 / 2 = {value}"),
        None => println!("   Option contained no value"),
    }

    // Each input demonstrates a successful Result or one of two custom failures.
    for input in ["21", "-3", "crab"] {
        match parse_positive(input) {
            Ok(value) => println!("   parsed `{input}` as {value}"),
            Err(error) => println!("   could not parse `{input}`: {error}"),
        }
    }

    println!(
        "   Result + ? average: {:?}\n",
        average_positive(&["10", "20", "30"])
    );
}

/// Divides two decimal numbers, returning no value for division by zero.
///
/// The type `Option<f64>` tells every caller that an f64 might not be available.
/// The caller cannot accidentally use the result as a plain number first.
pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

/// Every expected reason that positive-number parsing can fail.
///
/// Enum variants may carry useful context. NotANumber remembers the original text,
/// and NotPositive remembers the parsed integer, producing more helpful messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositiveNumberError {
    EmptyInput,
    NotANumber(String),
    NotPositive(i32),
}

// Display controls how `{error}` appears in a user-facing message.
impl fmt::Display for PositiveNumberError {
    // A Formatter is the destination being written to. `'_` asks Rust to infer
    // the reference lifetime because its exact name is unimportant here.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match on `self` and write a different sentence for each possible error.
        match self {
            Self::EmptyInput => write!(formatter, "at least one value is required"),
            Self::NotANumber(value) => write!(formatter, "`{value}` is not a whole number"),
            Self::NotPositive(value) => {
                write!(formatter, "expected a positive number, got {value}")
            }
        }
    }
}

// The standard Error trait has useful default behavior, so this implementation
// can be empty. Implementing it allows libraries and tools to recognize the enum
// as an error and combine it with other error types.
impl Error for PositiveNumberError {}

/// Parses text and accepts only whole numbers greater than zero.
pub fn parse_positive(input: &str) -> Result<i32, PositiveNumberError> {
    // `parse::<i32>()` returns Rust's standard parsing Result. `map_err` converts
    // its error into our own error type while preserving the original input.
    // `?` unwraps Ok or immediately returns Err from this function.
    let value = input
        .parse::<i32>()
        .map_err(|_| PositiveNumberError::NotANumber(input.to_owned()))?;

    // Parsing can succeed even when our business rule fails, so validate the
    // number separately and return the appropriate Result variant.
    if value <= 0 {
        Err(PositiveNumberError::NotPositive(value))
    } else {
        Ok(value)
    }
}

/// Parses several positive numbers and returns their arithmetic mean.
///
/// This demonstrates error propagation. `?` returns early with the first error;
/// otherwise it extracts each valid i32 so the calculation can continue.
pub fn average_positive(inputs: &[&str]) -> Result<f64, PositiveNumberError> {
    // Checking first avoids dividing by zero and gives the caller a precise reason.
    if inputs.is_empty() {
        return Err(PositiveNumberError::EmptyInput);
    }

    // i64 gives the total more room than each i32 input. A production system that
    // accepts extremely many values might also use checked addition here.
    let mut total: i64 = 0;
    for input in inputs {
        total += i64::from(parse_positive(input)?);
    }
    // `as f64` converts whole numbers to decimals so division keeps the fraction.
    Ok(total as f64 / inputs.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_represents_missing_division_result() {
        assert_eq!(divide(8.0, 2.0), Some(4.0));
        assert_eq!(divide(8.0, 0.0), None);
    }

    #[test]
    fn custom_errors_retain_context() {
        assert_eq!(parse_positive("5"), Ok(5));
        assert_eq!(
            parse_positive("0"),
            Err(PositiveNumberError::NotPositive(0))
        );
        assert_eq!(
            parse_positive("nope"),
            Err(PositiveNumberError::NotANumber("nope".to_owned()))
        );
    }

    #[test]
    fn question_mark_propagates_the_first_error() {
        // The -1 failure reaches this caller unchanged, proving that `?` did not
        // hide or replace the useful error information.
        assert_eq!(average_positive(&["2", "4", "6"]), Ok(4.0));
        assert_eq!(average_positive(&[]), Err(PositiveNumberError::EmptyInput));
        assert_eq!(
            average_positive(&["2", "-1", "6"]),
            Err(PositiveNumberError::NotPositive(-1))
        );
    }
}
