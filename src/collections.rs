//! Frequently used collections from Rust's standard library.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn run() {
    println!("4. COMMON DATA STRUCTURES");

    // Vec<T>: growable, contiguous array; the default sequential collection.
    let mut values = vec![3, 1, 4];
    values.push(1);
    println!("   Vec (dynamic array): {values:?}");

    // VecDeque<T>: efficient insertion/removal at both ends; useful as a queue.
    let mut queue = VecDeque::from(["compile", "test"]);
    queue.push_back("run");
    println!("   VecDeque front: {:?}", queue.pop_front());

    // HashMap<K, V>: average O(1) key lookup. HashSet<T>: unique values.
    let counts = word_frequencies("rust is fast and rust is safe");
    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_unstable_by_key(|(word, _)| *word);
    println!("   HashMap word counts: {sorted_counts:?}");

    let unique: HashSet<_> = [2, 2, 3, 5, 5].into_iter().collect();
    println!("   HashSet has {} unique values", unique.len());

    // BTreeMap keeps keys ordered and offers O(log n) lookup.
    let ordered = BTreeMap::from([("Ada", 1815), ("Grace", 1906), ("Linus", 1969)]);
    println!(
        "   BTreeMap ordered keys: {:?}",
        ordered.keys().collect::<Vec<_>>()
    );

    // BinaryHeap is a max-heap. Reverse turns it into a min-heap.
    let mut min_heap: BinaryHeap<Reverse<i32>> = values.into_iter().map(Reverse).collect();
    println!(
        "   BinaryHeap smallest item: {:?}\n",
        min_heap.pop().map(|item| item.0)
    );
}

pub fn word_frequencies(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// A small generic LIFO stack built on `Vec<T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

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
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert!(stack.is_empty());
    }
}
