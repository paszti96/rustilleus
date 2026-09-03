//! Safe concurrency with scoped threads, channels, `Arc`, and `Mutex`.

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub fn run() {
    println!("8. SAFE CONCURRENCY");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("   scoped threads sum: {}", parallel_sum(&numbers));

    // Channels transfer messages between threads. Sender ownership is cloned.
    let (sender, receiver) = mpsc::channel();
    thread::scope(|scope| {
        for message in ["fearless", "concurrency"] {
            let sender = sender.clone();
            scope.spawn(move || {
                sender.send(message).expect("receiver is still alive");
            });
        }
    });
    drop(sender); // Close the channel so iteration stops after all messages arrive.

    let mut messages: Vec<_> = receiver.into_iter().collect();
    messages.sort_unstable();
    println!("   channel messages: {messages:?}");
    println!("   Arc<Mutex<_>> counter: {}\n", shared_counter(4));
}

/// Scoped threads may safely borrow data that belongs to the calling function.
pub fn parallel_sum(values: &[i64]) -> i64 {
    let middle = values.len() / 2;
    let (left, right) = values.split_at(middle);

    thread::scope(|scope| {
        let left_task = scope.spawn(move || left.iter().sum::<i64>());
        let right_task = scope.spawn(move || right.iter().sum::<i64>());

        left_task.join().expect("left worker panicked")
            + right_task.join().expect("right worker panicked")
    })
}

/// `Arc` shares ownership between threads; `Mutex` permits one writer at a time.
pub fn shared_counter(worker_count: usize) -> usize {
    let counter = Arc::new(Mutex::new(0_usize));
    let mut workers = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        let counter = Arc::clone(&counter);
        workers.push(thread::spawn(move || {
            let mut value = counter.lock().expect("counter mutex was poisoned");
            *value += 1;
        }));
    }

    for worker in workers {
        worker.join().expect("counter worker panicked");
    }

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
