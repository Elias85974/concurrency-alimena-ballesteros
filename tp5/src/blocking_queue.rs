use std::sync::{Arc, Condvar, Mutex};

struct Node<T> {
    value: Option<T>,
    next: Option<Arc<Mutex<Node<T>>>>,
}

pub struct BlockingQueue<T> {
    head: Mutex<Arc<Mutex<Node<T>>>>,
    tail: Mutex<Arc<Mutex<Node<T>>>>,
    not_empty_condvar: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        let dummy = Arc::new(Mutex::new(Node {
            value: None,
            next: None,
        }));

        Self {
            head: Mutex::new(dummy.clone()),
            tail: Mutex::new(dummy),
            not_empty_condvar: Condvar::new(),
        }
    }

    pub fn enqueue(&self, item: T) {
        let new_node = Arc::new(Mutex::new(Node {
            value: Some(item),
            next: None,
        }));

        let mut tail_lock = self.tail.lock().unwrap();
        tail_lock.lock().unwrap().next = Some(new_node.clone());
        *tail_lock = new_node;
        self.not_empty_condvar.notify_one();
    }

    pub fn dequeue(&self) -> T {
        let mut head_lock = self.head.lock().unwrap();

        loop {
            let next_node_opt = head_lock.lock().unwrap().next.clone();

            if let Some(next_node) = next_node_opt {
                let next_node_clone = Arc::clone(&next_node);
                let mut next_node_guard = next_node.lock().unwrap();
                let value = next_node_guard.value.take().expect("Valor ausente");
                *head_lock = next_node_clone;
                return value;
            }

            head_lock = self.not_empty_condvar.wait(head_lock).unwrap();
        }
    }
}
