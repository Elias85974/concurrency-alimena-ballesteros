use std::thread;
use std::time::Duration;
use crate::leibniz_pi_partial;

pub struct ThreadOperations;

impl ThreadOperations {
    pub(crate) fn execute_program(&self) {
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
        });
    }

    pub(crate) fn leibniz_operation(&self, n: usize, threads: usize) -> f64 {
        let mut handles = Vec::new();
        let terms_per_thread = (n + 1) / threads;
        for i in 0..threads {
            handles.push(thread::spawn(move || {
                leibniz_pi_partial(terms_per_thread * i, terms_per_thread)
            }));
        }

        let mut leibniz_sum = 0.0;

        for h in handles {
            leibniz_sum += h.join().unwrap();
        }

        leibniz_sum
    }
}