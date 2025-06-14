use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod blocking_queue;
mod non_blocking_queue;

use blocking_queue::BlockingQueue;
use non_blocking_queue::NonBlockingQueue;

fn get_arg(args: &[String], key: &str, default: usize) -> usize {
    args.iter()
        .position(|arg| arg == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let producers = get_arg(&args, "--producers", 2);
    let consumers = get_arg(&args, "--consumers", 2);
    let total_items = get_arg(&args, "--items", 100);

    let queue = Arc::new(BlockingQueue::new());
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();

    let mut handles = vec![];

    // Producers
    for i in 0..producers {
        let queue = Arc::clone(&queue);
        let items_per_producer = total_items / producers;
        handles.push(thread::spawn(move || {
            for j in 0..items_per_producer {
                let value = format!("P{i}-Item{j}");
                println!("Producer: {} producing {}", i, value);
                queue.enqueue(value);
            }
        }));
    }

    // Consumers
    for i in 0..consumers {
        let queue = Arc::clone(&queue);
        let counter = Arc::clone(&counter);
        let items_per_consumer = total_items / consumers;
        handles.push(thread::spawn(move || {
            for _ in 0..items_per_consumer {
                let value = queue.dequeue();
                println!("Consumer: {} consuming {}", i, value);
                let mut c = counter.lock().unwrap();
                *c += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Consumidos: {}", *counter.lock().unwrap());
    println!("Tiempo: {:.2?}", start.elapsed());
}