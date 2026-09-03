//! # Lesson 5: Algorithms
//!
//! An algorithm is a precise series of steps for solving a problem. These examples
//! favor clarity over clever tricks. Their time complexity describes how their
//! running time grows as the input size `n` grows:
//!
//! - O(n) work grows in a straight line with the number of items.
//! - O(log n) repeatedly cuts the remaining problem roughly in half.
//! - O(n²) may compare every item with many other items.
//! - O(n log n) is typical of efficient comparison-based sorting.

// A queue is needed for breadth-first graph traversal.
use std::collections::VecDeque;

/// Runs and prints every algorithm example.
pub fn run() {
    println!("5. ALGORITHMS");

    // Binary search only works correctly because `values` is already sorted.
    let values = [2, 4, 6, 8, 10, 12];
    println!("   linear search for 8: {:?}", linear_search(&values, &8));
    println!("   binary search for 10: {:?}", binary_search(&values, &10));

    // Insertion sort changes a mutable array in place; it does not return a new one.
    let mut small = [5, 2, 4, 1, 3];
    insertion_sort(&mut small);
    println!("   insertion sort: {small:?}");

    // Merge sort borrows the input and creates a new sorted Vec.
    let sorted = merge_sort(&[38, 27, 43, 3, 9, 82, 10]);
    println!("   merge sort: {sorted:?}");

    // A graph stores nodes and connections. In this adjacency-list representation,
    // each outer index is a node and its inner Vec lists neighboring node indexes.
    // For example, node 0 connects to nodes 1 and 2.
    let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![], vec![]];
    println!(
        "   breadth-first traversal: {:?}",
        breadth_first_search(&graph, 0)
    );
    println!("   fibonacci(20): {:?}\n", fibonacci(20));
}

/// Finds a target by checking values from left to right.
///
/// This is O(n): in the worst case it inspects every item. `T: PartialEq` means
/// the generic type T must support equality comparison with `==`.
pub fn linear_search<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    // `position` stops at the first true comparison and returns its index. It
    // returns None if the iterator ends without finding the target.
    items.iter().position(|item| item == target)
}

/// Finds a target by repeatedly discarding half of a sorted slice.
///
/// This is O(log n), which is much faster than a linear scan for large inputs.
/// The input **must already be sorted**. `T: Ord` requires a complete ordering so
/// any two values can be classified as less, greater, or equal.
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    // `low` is included in the search area and `high` is excluded. Starting high
    // at len therefore covers every valid index while also handling an empty slice.
    let mut low = 0;
    let mut high = items.len();

    while low < high {
        // This form avoids the possible overflow of `(low + high) / 2`.
        let middle = low + (high - low) / 2;

        // Compare the middle item and keep only the half where target may remain.
        match items[middle].cmp(target) {
            std::cmp::Ordering::Less => low = middle + 1,
            std::cmp::Ordering::Greater => high = middle,
            std::cmp::Ordering::Equal => return Some(middle),
        }
    }
    None
}

/// Sorts a mutable slice by inserting each item into the sorted part before it.
///
/// Insertion sort is O(n²) in the worst case, but is short, works in place, and
/// can perform well on tiny or nearly sorted inputs.
pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    // At the start of each outer iteration, everything before `index` is sorted.
    for index in 1..items.len() {
        let mut current = index;

        // Move the new item left until it is no smaller than its neighbor. `swap`
        // exchanges values safely without copying or temporarily losing either one.
        while current > 0 && items[current] < items[current - 1] {
            items.swap(current, current - 1);
            current -= 1;
        }
    }
}

/// Recursively divides a slice, sorts both halves, and merges them.
///
/// Merge sort is O(n log n) and stable, meaning equal elements keep their original
/// order. This teaching version creates extra vectors, so it uses O(n) extra memory.
/// `Clone` is required because values are copied into the new output vectors.
pub fn merge_sort<T: Ord + Clone>(items: &[T]) -> Vec<T> {
    // A collection of zero or one item is already sorted. This base case also
    // stops the recursive function from calling itself forever.
    if items.len() <= 1 {
        return items.to_vec();
    }

    // Split without copying, recursively sort both borrowed halves, then combine.
    let middle = items.len() / 2;
    let left = merge_sort(&items[..middle]);
    let right = merge_sort(&items[middle..]);
    merge(&left, &right)
}

/// Combines two already-sorted slices into one sorted vector.
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    // Reserving the exact final capacity avoids repeated heap reallocations.
    let mut output = Vec::with_capacity(left.len() + right.len());
    let (mut left_index, mut right_index) = (0, 0);

    // Repeatedly take the smaller front item. Using `<=` chooses the left item
    // first when values are equal, which is what makes merge sort stable.
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            output.push(left[left_index].clone());
            left_index += 1;
        } else {
            output.push(right[right_index].clone());
            right_index += 1;
        }
    }

    // At most one side has leftovers. Appending both remaining slices is simple;
    // one of them will always be empty.
    output.extend_from_slice(&left[left_index..]);
    output.extend_from_slice(&right[right_index..]);
    output
}

/// Visits every reachable graph node in breadth-first order.
///
/// Breadth-first search (BFS) explores all nearby nodes before moving farther
/// away. It runs in O(vertices + edges) and is useful for shortest paths in an
/// unweighted graph. The return value records the order of first visits.
pub fn breadth_first_search(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    // An invalid start cannot be indexed safely, so return an empty traversal.
    if start >= graph.len() {
        return Vec::new();
    }

    // `visited` prevents cycles from making us process a node forever. The queue
    // stores discovered nodes whose neighbors still need to be explored.
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::from([start]);
    let mut order = Vec::new();
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &graph[node] {
            // The first condition also makes this function tolerate an invalid
            // neighbor index instead of crashing while indexing `visited`.
            if neighbor < graph.len() && !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    order
}

/// Calculates the nth Fibonacci number using previous results.
///
/// This is iterative dynamic programming: each answer reuses the previous two
/// answers. It takes O(n) time and O(1) extra space. A `u64` has a maximum value,
/// so `checked_add` returns `None` instead of silently wrapping on overflow.
pub fn fibonacci(n: u32) -> Option<u64> {
    // Fibonacci starts 0, 1, 1, 2, 3... and F(0) is a special initial case.
    if n == 0 {
        return Some(0);
    }

    // Before each loop, `previous` and `current` hold two neighboring numbers.
    // The `?` immediately returns None if adding them would overflow.
    let (mut previous, mut current) = (0_u64, 1_u64);
    for _ in 1..n {
        (previous, current) = (current, previous.checked_add(current)?);
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_present_and_missing_values() {
        // Good tests cover successful searches and a missing target.
        let values = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&values, &5), Some(2));
        assert_eq!(binary_search(&values, &9), Some(4));
        assert_eq!(binary_search(&values, &4), None);
    }

    #[test]
    fn sorts_empty_duplicate_and_regular_inputs() {
        // The empty array checks an important boundary case. Repeated 4s check
        // that an algorithm does not accidentally discard duplicate values.
        let mut empty: [i32; 0] = [];
        insertion_sort(&mut empty);
        assert_eq!(empty, []);
        assert_eq!(merge_sort(&[4, 1, 4, 2]), [1, 2, 4, 4]);
    }

    #[test]
    fn traverses_each_reachable_node_once() {
        // Node 4 is disconnected, so a traversal starting at 0 must not include it.
        let graph = vec![vec![1, 2], vec![2], vec![0, 3], vec![], vec![]];
        assert_eq!(breadth_first_search(&graph, 0), [0, 1, 2, 3]);
        assert!(breadth_first_search(&graph, 99).is_empty());
    }

    #[test]
    fn computes_fibonacci_and_detects_overflow() {
        assert_eq!(fibonacci(0), Some(0));
        assert_eq!(fibonacci(10), Some(55));
        assert_eq!(fibonacci(93), Some(12_200_160_415_121_876_738));
        assert_eq!(fibonacci(94), None);
    }
}
