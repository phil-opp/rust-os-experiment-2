use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, AtomicIsize, Ordering};
use std::sync::mpsc::spsc_queue::Queue;
use global;
use stream::{Stream, Subscriber};

pub fn stream<T>() -> (SpscSender<T>, SpscStream<T>) where T: Send + 'static {
    let inner = Arc::new(Inner{
        queue: unsafe{Queue::new(128)},
         // set count to 1 so sender won't try to wake receiver until it's set
        count: AtomicIsize::new(1),
        receiver: AtomicPtr::new(0 as *mut _),
        });
    (SpscSender{inner: inner.clone()}, SpscStream{inner: inner})
}

pub struct SpscSender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> SpscSender<T> where T: Send + 'static {
    pub fn send(&self, value: T) {
        self.inner.queue.push(value);
        match self.inner.count.fetch_add(1, Ordering::SeqCst) {
            0 => Inner::spawn_receiver(self.inner.clone()),
            n if n>0 => {},
            n => panic!("stream count {}", n),
        }
    }

    pub fn close(self) {}
}

impl<T> !Sync for SpscSender<T> {}

pub struct SpscStream<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Stream for SpscStream<T> where T: Send + 'static {
    type Item = T;

    fn subscribe<Sub>(self, subscriber: Sub) where
        Sub: Subscriber<T> + Send + 'static
    {
        let receiver_raw = Box::into_raw(Box::new(Receiver(
            Box::new(subscriber))));
        assert!(self.inner.receiver.swap(receiver_raw, Ordering::SeqCst) ==
        (0 as *mut _));

        match self.inner.count.fetch_sub(1, Ordering::SeqCst) {
            n if n<1 => unreachable!(),
            1 => {}, // queue is empty and next send will wake receiver
            _ => {
                // queue is filled -> wake receiver (send won't wake it as
                // count is >0)
                Inner::spawn_receiver(self.inner);
            }
        }
    }
}

struct Inner<T> {
    queue: Queue<T>,
    count: AtomicIsize,
    receiver: AtomicPtr<Receiver<T>>,
}

impl<T> Inner<T> where T: Send + 'static {
    fn spawn_receiver(inner: Arc<Inner<T>>) {
        match unsafe{inner.receiver.load(Ordering::SeqCst).as_mut()} {
            None => panic!("no receiver registered"),
            Some(receiver) => global::spawn(move || {
                    while let Some(value) = inner.queue.pop() {
                        receiver.0.on_value(value);
                        if inner.count.fetch_sub(1, Ordering::SeqCst) == 1 {
                            // we removed the last element
                            return;
                        }
                    }
                    unreachable!();
                }),
        }
    }
}

// TODO why is AtomicPtr not Send?
unsafe impl<T: Send> Send for Inner<T> {}

impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        match self.receiver.load(Ordering::SeqCst) {
            s if s.is_null() => {} // no receiver registered,
            s => unsafe{Box::from_raw(s)}.0.on_close_boxed(),
        }
    }
}

struct Receiver<T>(Box<Subscriber<T> + Send>);
