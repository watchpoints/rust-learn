#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // ✅ 用 `Arc` 让 `numbers` 在多个线程间安全共享
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // ✅ 克隆 `Arc`，确保 `child_numbers` 在各个线程中共享 `numbers`
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers
                .iter()
                .enumerate()
                .filter(|(index, _)| index % 8 == offset)
                .map(|(_, &n)| n)
                .sum();
            
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
