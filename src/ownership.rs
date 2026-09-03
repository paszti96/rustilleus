//! # Lesson 2: Ownership and borrowing
//!
//! Ownership is Rust's most distinctive feature. Imagine a value as a library
//! book: one variable is responsible for returning it. Ownership may be handed to
//! another variable (a **move**) or temporarily lent out (a **borrow**).
//!
//! These rules let Rust release memory safely without a garbage collector:
//!
//! 1. Every value has one owner.
//! 2. When the owner goes out of scope, the value is dropped and cleaned up.
//! 3. At one time, code may have many read-only references or one mutable reference.
//! 4. A reference may never live longer than the value it points to.

/// Runs and prints the ownership examples.
pub fn run() {
    println!("2. OWNERSHIP AND BORROWING");

    // A `String` stores its characters in the heap, a memory area used for data
    // whose size can change. The variable itself keeps information such as the
    // text's length. Moving the String transfers responsibility for that memory.
    let original = String::from("ownership");
    let moved = original;
    // `original` cannot be used here because its value moved to `moved`. This
    // prevents two owners from trying to free the same heap memory. Calling
    // `original.clone()` instead would copy the characters into new heap memory.
    println!("   moved String: {moved}");

    // Fixed-size numbers live directly in their variables and implement `Copy`.
    // Assigning one to another duplicates the bits, so both remain usable.
    let first = 7;
    let copied = first;
    println!("   copied integers: {first} and {copied}");

    // The `&` creates a reference: permission to use a value without owning it.
    // `moved` remains valid after this function call because it was only borrowed.
    print_length(&moved);

    // `&mut` creates a mutable reference that permits changes. Rust allows only
    // one active mutable reference to a value, preventing conflicting changes.
    let mut greeting = String::from("Hello");
    add_name(&mut greeting, "Ferris");
    println!("   after mutable borrow: {greeting}");

    // TRY IT: remove the `//` marks from the next four lines, run `cargo check`,
    // and read the compiler message. Both references try to change `greeting` at
    // the same time, so Rust rejects the code before it can become a data race.
    // let first_writer = &mut greeting;
    // let second_writer = &mut greeting;
    // first_writer.push_str(" first");
    // second_writer.push_str(" second");

    // A string slice (`&str`) borrows some UTF-8 text. It stores where the text
    // begins and how long it is, but it does not allocate or copy characters.
    let text = String::from("borrow checker");
    println!("   first word slice: {}", first_word(&text));

    // Lifetimes describe relationships between borrowed references. They usually
    // do not change how long a value actually lives; they help the compiler prove
    // that the returned reference will still be valid.
    let left = "short";
    let right = "a little longer";
    println!("   lifetime example: {}\n", longest(left, right));
}

/// Prints borrowed text and its byte length without taking ownership.
///
/// `str` is Rust's text-slice type. Non-ASCII characters can use multiple bytes,
/// so `len()` reports bytes rather than the number of visible characters.
pub fn print_length(text: &str) {
    println!("   borrowed `{text}` has {} bytes", text.len());
}

/// Appends a name to a greeting through one mutable reference.
///
/// `String` can grow, while `&str` is a borrowed view. Accepting `name` as `&str`
/// makes the function work with both string literals and borrowed Strings.
pub fn add_name(greeting: &mut String, name: &str) {
    // `push_str` appends several characters; `push` appends one `char`.
    greeting.push_str(", ");
    greeting.push_str(name);
    greeting.push('!');
}

/// Returns the first whitespace-separated word without creating a new String.
///
/// Rust's lifetime-elision rules understand that the output slice comes from the
/// one input reference, so we do not need to write a lifetime name explicitly.
pub fn first_word(text: &str) -> &str {
    // `split_whitespace` is an iterator. `next()` returns `Some(word)` or `None`.
    // Empty input has no word, so `unwrap_or("")` safely returns an empty slice.
    text.split_whitespace().next().unwrap_or("")
}

/// Returns the longer of two borrowed pieces of text.
///
/// `'a` is a lifetime parameter, not a timer. It tells the compiler that both
/// inputs and the returned reference share a valid lifetime. In practical terms,
/// the caller may use the result only while both possible inputs are still alive.
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

// These tests also demonstrate that borrowed results contain the expected text
// and that a mutable borrow really changes its owner.
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
