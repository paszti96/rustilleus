//! Ownership, moves, copies, borrowing, slices, and a small lifetime example.

pub fn run() {
    println!("2. OWNERSHIP AND BORROWING");

    // `String` owns heap data. Moving it transfers that ownership.
    let original = String::from("ownership");
    let moved = original;
    // `original` cannot be used here. `clone` would make a deep copy instead.
    println!("   moved String: {moved}");

    // Numbers implement `Copy`, so both variables remain usable.
    let first = 7;
    let copied = first;
    println!("   copied integers: {first} and {copied}");

    // References borrow data without taking ownership.
    print_length(&moved);

    // One mutable reference can modify the borrowed value.
    let mut greeting = String::from("Hello");
    add_name(&mut greeting, "Ferris");
    println!("   after mutable borrow: {greeting}");

    // A string slice borrows part of a string and carries no allocation.
    let text = String::from("borrow checker");
    println!("   first word slice: {}", first_word(&text));

    let left = "short";
    let right = "a little longer";
    println!("   lifetime example: {}\n", longest(left, right));
}

pub fn print_length(text: &str) {
    println!("   borrowed `{text}` has {} bytes", text.len());
}

pub fn add_name(greeting: &mut String, name: &str) {
    greeting.push_str(", ");
    greeting.push_str(name);
    greeting.push('!');
}

/// Returns a slice tied to the lifetime of the input string.
pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// The annotation says the result cannot outlive either input.
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_word_without_allocating() {
        let sentence = String::from("safe and fast");
        assert_eq!(first_word(&sentence), "safe");
    }

    #[test]
    fn mutates_through_one_mutable_reference() {
        let mut greeting = String::from("Hi");
        add_name(&mut greeting, "Ada");
        assert_eq!(greeting, "Hi, Ada!");
    }
}
