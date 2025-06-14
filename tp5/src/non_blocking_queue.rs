use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(null_mut())
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy)
        }
    }

    pub fn enqueue(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(null_mut())
        }));
        loop {
            let cur_tail = self.tail.load(Ordering::Acquire);
            let tail_next = unsafe { (*cur_tail).next.load(Ordering::Acquire) };

            if cur_tail == self.tail.load(Ordering::Acquire) {
                if tail_next.is_null() {
                    // Intentar enlazar el nuevo nodo al final
                    if unsafe { (*cur_tail).next.compare_exchange(null_mut(), new_node, Ordering::SeqCst, Ordering::SeqCst) }.is_ok() {
                        // Actualizar el puntero `tail` al nuevo nodo
                        self.tail.compare_exchange(cur_tail, new_node, Ordering::SeqCst, Ordering::SeqCst).ok();
                        break;
                    }
                } else {
                    // Avanzar el puntero `tail` al siguiente nodo
                    self.tail.compare_exchange(cur_tail, tail_next, Ordering::SeqCst, Ordering::SeqCst).ok();
                }
            }
        }
    }

    pub fn dequeue(&self) -> T {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if next.is_null() {
                // Si la cola está vacía, volver a intentar
                continue;
            }

            if self.head.compare_exchange(head, next, Ordering::Release, Ordering::Relaxed).is_ok() {
                unsafe {
                    let value = (*next).value.take().expect("Valor ausente");
                    let _ = Box::from_raw(head); // Liberar el nodo antiguo
                    return value;
                }
            }
        }
    }
}