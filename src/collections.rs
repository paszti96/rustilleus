//! # Lesson 4: Common data structures
//!
//! A data structure organizes values so a program can store and retrieve them.
//! Picking the right one can make code simpler and faster. Big-O notation gives
//! a rough description of how work grows: O(1) stays about constant, O(log n)
//! grows slowly, and O(n) may inspect every item.

// `Reverse` flips comparison order. The remaining types are collections from
// `std::collections`, Rust's standard collection toolbox.
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

/// Runs and prints examples of frequently used standard collections.
pub fn run() {
    println!("4. COMMON DATA STRUCTURES");

    // Vec<T>: a growable, contiguous array and Rust's default list type.
    // "Contiguous" means elements sit next to each other in memory, which makes
    // index access fast. Appending is usually O(1), though it occasionally grows.
    let mut values = vec![3, 1, 4];
    values.push(1);
    println!("   Vec (dynamic array): {values:?}");

    // VecDeque<T>: efficient insertion and removal at both ends. It is ideal for
    // a queue, where the first item added is the first item removed (FIFO).
    let mut queue = VecDeque::from(["compile", "test"]);
    queue.push_back("run");
    println!("   VecDeque front: {:?}", queue.pop_front());

    // HashMap<K, V> pairs each key with a value and usually looks up a key in O(1)
    // time. Word text is the key and its count is the value in this example.
    let counts = word_frequencies("rust is fast and rust is safe");
    // HashMap iteration order is deliberately unpredictable. Sorting these
    // references gives learners stable output without copying the words.
    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_unstable_by_key(|(word, _)| *word);
    println!("   HashMap word counts: {sorted_counts:?}");

    // HashSet<T> keeps each value only once. It is useful for membership questions
    // such as "Have we already seen this item?" and usually answers in O(1).
    let unique: HashSet<_> = [2, 2, 3, 5, 5].into_iter().collect();
    println!("   HashSet has {} unique values", unique.len());

    // BTreeMap keeps keys in sorted order and offers O(log n) lookup. Choose it
    // over HashMap when ordered traversal or range queries matter.
    let ordered = BTreeMap::from([("Ada", 1815), ("Grace", 1906), ("Linus", 1969)]);
    println!(
        "   BTreeMap ordered keys: {:?}",
        ordered.keys().collect::<Vec<_>>()
    );

    // BinaryHeap is a priority queue. By default it removes the largest item first
    // (a max-heap). Wrapping each number in Reverse makes the smallest come first.
    let mut min_heap: BinaryHeap<Reverse<i32>> = values.into_iter().map(Reverse).collect();
    println!(
        "   BinaryHeap smallest item: {:?}\n",
        min_heap.pop().map(|item| item.0)
    );
}

/// Counts how many times each whitespace-separated word appears.
///
/// The returned map borrows word slices from `text`; it does not copy the words.
/// Rust's lifetime-elision rules understand this relationship automatically.
pub fn word_frequencies(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        // `entry` looks up the word. If it is missing, `or_insert(0)` adds zero.
        // The leading `*` follows the returned mutable reference so `+= 1` changes
        // the number stored inside the map.
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// A small generic last-in-first-out stack built on `Vec<T>`.
///
/// `T` is a type parameter: one Stack might hold numbers while another holds
/// Strings. The same implementation works for both without losing type safety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates an empty Stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Places an item on top of the Stack, moving ownership into the collection.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Removes and returns the top item, or `None` when the Stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Borrows the top item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Returns true when the Stack has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Implementing Default lets callers use `Stack::default()`. Rust APIs commonly
// provide Default when there is one obvious starting value, such as an empty stack.
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        let counts = word_frequencies("red blue red");
        assert_eq!(counts.get("red"), Some(&2));
        assert_eq!(counts.get("blue"), Some(&1));
    }

    #[test]
    fn stack_is_last_in_first_out() {
        // Type inference discovers `Stack<i32>` from the numbers pushed below.
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert!(stack.is_empty());
    }
}
