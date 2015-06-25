use std::boxed;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, AtomicIsize, Ordering};
use std::sync::mpsc::spsc_queue::Queue;
use super::{Stream, Subscriber};

pub fn stream<T>() -> (Sender<T>, BasicStream<T>) {
    let inner = Arc::new(Inner {
        receiver: AtomicPtr::new(0 as *mut _),
    });
    (Sender{inner: inner.clone()}, BasicStream{inner: inner})
}

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        match self.inner.receiver.load(Ordering::SeqCst) {
            r if r == (0 as *mut _) => {}, // no receiver registered
            r => {
                let receiver = unsafe{&mut *r};
                receiver.0.on_value(value);
            }
        }
    }
    pub fn close(self) {
        match self.inner.receiver.swap(0 as *mut _, Ordering::SeqCst) {
            r if r == (0 as *mut _) => loop{},
            r => unsafe{Box::from_raw(r)}.0.on_close(),
        }
    }
}

pub struct BasicStream<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Stream for BasicStream<T> {
    type Item = T;

    fn subscribe(self, subscriber: Box<Subscriber<T> + Send>) {
        let receiver_raw = boxed::into_raw(Box::new(Receiver(subscriber)));
        match self.inner.receiver.swap(receiver_raw, Ordering::SeqCst) {
            r if r == (0 as *mut _) => {},
            r => mem::drop(unsafe{Box::from_raw(r)}),
        }
    }
}

impl<T> BasicStream<T> {
        pub fn unsubscribe(&self) -> Option<Box<Subscriber<T>>> {
            match self.inner.receiver.swap(0 as *mut _, Ordering::SeqCst) {
                r if r == (0 as *mut _) => None,
                r => Some(unsafe{Box::from_raw(r)}.0),
            }
        }
}

struct Inner<T> {
    receiver: AtomicPtr<Receiver<T>>,
}

struct Receiver<T>(Box<Subscriber<T>>);
