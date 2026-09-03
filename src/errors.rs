//! Recoverable errors with `Option`, `Result`, custom error types, and `?`.

use std::{error::Error, fmt};

pub fn run() {
    println!("7. ERROR HANDLING");

    match divide(10.0, 2.0) {
        Some(value) => println!("   Option success: 10 / 2 = {value}"),
        None => println!("   Option contained no value"),
    }

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

/// `Option<T>` represents either a value (`Some`) or its absence (`None`).
pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositiveNumberError {
    EmptyInput,
    NotANumber(String),
    NotPositive(i32),
}

impl fmt::Display for PositiveNumberError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(formatter, "at least one value is required"),
            Self::NotANumber(value) => write!(formatter, "`{value}` is not a whole number"),
            Self::NotPositive(value) => {
                write!(formatter, "expected a positive number, got {value}")
            }
        }
    }
}

impl Error for PositiveNumberError {}

pub fn parse_positive(input: &str) -> Result<i32, PositiveNumberError> {
    let value = input
        .parse::<i32>()
        .map_err(|_| PositiveNumberError::NotANumber(input.to_owned()))?;

    if value <= 0 {
        Err(PositiveNumberError::NotPositive(value))
    } else {
        Ok(value)
    }
}

/// `?` returns early on an error and unwraps successful values.
pub fn average_positive(inputs: &[&str]) -> Result<f64, PositiveNumberError> {
    if inputs.is_empty() {
        return Err(PositiveNumberError::EmptyInput);
    }

    let mut total: i64 = 0;
    for input in inputs {
        total += i64::from(parse_positive(input)?);
    }
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
        assert_eq!(average_positive(&["2", "4", "6"]), Ok(4.0));
        assert_eq!(average_positive(&[]), Err(PositiveNumberError::EmptyInput));
        assert_eq!(
            average_positive(&["2", "-1", "6"]),
            Err(PositiveNumberError::NotPositive(-1))
        );
    }
}
