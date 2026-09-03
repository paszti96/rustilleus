//! # Lesson 8: Safe concurrency
//!
//! Concurrency means making progress on more than one task during the same period.
//! A **thread** is one path of instructions inside a program. Threads can run at
//! the same time on different CPU cores, but sharing data carelessly can cause a
//! data race: two threads access the same memory and at least one changes it.
//!
//! Rust's ownership rules prevent many concurrency bugs before the program runs.
//! `Send` marks types safe to move between threads, and `Sync` marks types safe to
//! share by reference. The compiler applies these traits automatically when safe.

// `Arc`, `Mutex`, and `mpsc` solve different sharing problems. `thread` provides
// the functions that create and manage operating-system threads.
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/// Runs and prints the safe-concurrency examples.
pub fn run() {
    println!("8. SAFE CONCURRENCY");

    // Both worker threads borrow different parts of this array. The scoped-thread
    // API guarantees they finish before `numbers` goes out of scope.
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("   scoped threads sum: {}", parallel_sum(&numbers));

    // A channel acts like a one-way pipe. One or more Sender values put messages
    // in, and the Receiver takes them out. This pattern shares information by
    // transferring messages instead of sharing mutable memory.
    let (sender, receiver) = mpsc::channel();
    thread::scope(|scope| {
        for message in ["fearless", "concurrency"] {
            // Clone the sending handle for this worker. `move` transfers that
            // handle and the message reference into the thread closure.
            let sender = sender.clone();
            scope.spawn(move || {
                sender.send(message).expect("receiver is still alive");
            });
        }
    });
    // Destroy the original sender explicitly. After all cloned senders are also
    // dropped, the channel closes and the receiver's iterator knows when to stop.
    drop(sender);

    // Thread scheduling is not predictable, so messages may arrive in either
    // order. Sorting makes the lesson's printed output stable from run to run.
    let mut messages: Vec<_> = receiver.into_iter().collect();
    messages.sort_unstable();
    println!("   channel messages: {messages:?}");
    println!("   Arc<Mutex<_>> counter: {}\n", shared_counter(4));
}

/// Adds a borrowed slice using two scoped worker threads.
///
/// Ordinary spawned threads may outlive their caller, so they generally require
/// owned data. Scoped threads promise to finish before `thread::scope` returns;
/// that promise lets the workers safely borrow `values` from this function.
pub fn parallel_sum(values: &[i64]) -> i64 {
    // `split_at` creates two non-overlapping slices without copying any numbers.
    let middle = values.len() / 2;
    let (left, right) = values.split_at(middle);

    thread::scope(|scope| {
        // Each closure sums one half. `spawn` returns a handle used to wait for and
        // retrieve that worker's result.
        let left_task = scope.spawn(move || left.iter().sum::<i64>());
        let right_task = scope.spawn(move || right.iter().sum::<i64>());

        // `join` waits for a thread. A panic in a worker becomes Err, so `expect`
        // gives a clear message in that exceptional situation.
        left_task.join().expect("left worker panicked")
            + right_task.join().expect("right worker panicked")
    })
}

/// Lets several threads safely increment one shared counter.
///
/// `Arc` means "atomically reference counted." Cloning it creates another owner
/// and atomic counting keeps that ownership safe across threads. `Mutex` means
/// "mutual exclusion": only the thread holding its lock may access the inner value.
pub fn shared_counter(worker_count: usize) -> usize {
    let counter = Arc::new(Mutex::new(0_usize));
    let mut workers = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        // Clone the Arc, not the counter itself. Every worker points to the same
        // Mutex-protected value, and the value lives until the final Arc is dropped.
        let counter = Arc::clone(&counter);
        workers.push(thread::spawn(move || {
            // `lock` waits until no other thread owns the lock. The returned guard
            // unlocks automatically when it leaves scope, an example of RAII.
            let mut value = counter.lock().expect("counter mutex was poisoned");
            *value += 1;
        }));
    }

    // Joining all workers ensures every increment is finished before we read.
    for worker in workers {
        worker.join().expect("counter worker panicked");
    }

    // This final expression locks briefly, copies the usize, and returns it. The
    // semicolon is omitted because this is the function's return value.
    *counter.lock().expect("counter mutex was poisoned")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_borrowed_data_across_scoped_threads() {
        assert_eq!(parallel_sum(&[]), 0);
        assert_eq!(parallel_sum(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn mutex_prevents_lost_updates() {
        assert_eq!(shared_counter(20), 20);
    }
}
