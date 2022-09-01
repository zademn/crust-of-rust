use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex}; // condvar is a way to announce to another thread that

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap(); // Guard or PoisonError (the last thread panicked)
        inner.queue.push_back(t);
        drop(inner); // Drop the lock so the next thread takes the lock
        self.shared.available.notify_one(); // Notify a thread to wake up on that specific Condvar
                                            // Note: This does not notify a specific thread, only a thread that has the specific Condvar
                                            // In our case we have only one receiver
    }
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap(); // Guard or PoisonError (the last thread panicked)

        // Make the Receiver wait for stuff
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t), // Release the Mutex
                None if inner.senders == 0 => return None,
                None => {
                    // wait gives the mutex back AND gives up the lock
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}
struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
        },
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed() {
        // Indicate to the receiver that the channel is closed
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        // Design decision
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
