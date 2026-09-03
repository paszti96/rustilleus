//! Search, sorting, graph traversal, and dynamic-programming examples.

use std::collections::VecDeque;

pub fn run() {
    println!("5. ALGORITHMS");

    let values = [2, 4, 6, 8, 10, 12];
    println!("   linear search for 8: {:?}", linear_search(&values, &8));
    println!("   binary search for 10: {:?}", binary_search(&values, &10));

    let mut small = [5, 2, 4, 1, 3];
    insertion_sort(&mut small);
    println!("   insertion sort: {small:?}");

    let sorted = merge_sort(&[38, 27, 43, 3, 9, 82, 10]);
    println!("   merge sort: {sorted:?}");

    // Adjacency-list graph: 0 connects to 1 and 2, and so on.
    let graph = vec![vec![1, 2], vec![3], vec![3, 4], vec![], vec![]];
    println!(
        "   breadth-first traversal: {:?}",
        breadth_first_search(&graph, 0)
    );
    println!("   fibonacci(20): {:?}\n", fibonacci(20));
}

/// O(n) search that works for any comparable element type.
pub fn linear_search<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    items.iter().position(|item| item == target)
}

/// O(log n) search. The input must already be sorted.
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = items.len();

    while low < high {
        let middle = low + (high - low) / 2;
        match items[middle].cmp(target) {
            std::cmp::Ordering::Less => low = middle + 1,
            std::cmp::Ordering::Greater => high = middle,
            std::cmp::Ordering::Equal => return Some(middle),
        }
    }
    None
}

/// O(n^2), in-place, and useful for teaching or very small inputs.
pub fn insertion_sort<T: Ord>(items: &mut [T]) {
    for index in 1..items.len() {
        let mut current = index;
        while current > 0 && items[current] < items[current - 1] {
            items.swap(current, current - 1);
            current -= 1;
        }
    }
}

/// O(n log n), stable, and returns a newly allocated sorted vector.
pub fn merge_sort<T: Ord + Clone>(items: &[T]) -> Vec<T> {
    if items.len() <= 1 {
        return items.to_vec();
    }

    let middle = items.len() / 2;
    let left = merge_sort(&items[..middle]);
    let right = merge_sort(&items[middle..]);
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut output = Vec::with_capacity(left.len() + right.len());
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            output.push(left[left_index].clone());
            left_index += 1;
        } else {
            output.push(right[right_index].clone());
            right_index += 1;
        }
    }

    output.extend_from_slice(&left[left_index..]);
    output.extend_from_slice(&right[right_index..]);
    output
}

/// O(vertices + edges) traversal for an adjacency-list graph.
pub fn breadth_first_search(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    if start >= graph.len() {
        return Vec::new();
    }

    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::from([start]);
    let mut order = Vec::new();
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &graph[node] {
            if neighbor < graph.len() && !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    order
}

/// Iterative dynamic programming in O(n) time and O(1) space.
/// Returns `None` when the result cannot fit in a `u64`.
pub fn fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

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
        let values = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&values, &5), Some(2));
        assert_eq!(binary_search(&values, &9), Some(4));
        assert_eq!(binary_search(&values, &4), None);
    }

    #[test]
    fn sorts_empty_duplicate_and_regular_inputs() {
        let mut empty: [i32; 0] = [];
        insertion_sort(&mut empty);
        assert_eq!(empty, []);
        assert_eq!(merge_sort(&[4, 1, 4, 2]), [1, 2, 4, 4]);
    }

    #[test]
    fn traverses_each_reachable_node_once() {
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
