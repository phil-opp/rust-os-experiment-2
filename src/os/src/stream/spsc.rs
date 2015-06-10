use std::boxed::into_raw;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::spsc_queue::Queue;
use std::ptr::Unique;
use global;
use super::{Sender, Subscriber};

const IDLE: usize = 0;          // subscriber idle (sender has ownership)
const WORKING: usize = 1;       // subscriber working
const STREAM_CLOSED: usize = 2; // sender closed stream

pub struct SpscSender<T, S> where
    S: Subscriber<T>, S: 'static + Send, T: 'static + Send
{
    subscriber: Unique<SubscriberLink<T, S>>
}

impl<T, S> SpscSender<T, S> where
    S: Subscriber<T>, S: 'static + Send, T: 'static + Send
{
    pub fn new(subscriber: S) -> SpscSender<T, S> {
        unsafe {
            SpscSender {
                subscriber: Unique::new(into_raw(Box::new(
                    SubscriberLink {
                        queue: Queue::new(0),   // unlimited cache
                        status: AtomicUsize::new(IDLE),
                        subscriber: subscriber,
                }))),
            }
        }
    }
}

impl<T, f> SpscSender<T, FnSubscriber<f>> where
    T: 'static + Send, f: FnMut(T) + Send
{
    pub fn on_value(on_value: f) -> SpscSender<T, FnSubscriber<f>> {
        Self::new(FnSubscriber(on_value))
    }
}

impl<T, S> Sender<T> for SpscSender<T, S> where
    S: Subscriber<T> + 'static + Send, T: 'static + Send
{
    fn send(&self, value: T) {
        unsafe{ self.subscriber.get().new_value(value) };
    }

    fn close(self) {} // stream will be closed on drop
}

impl<T, S> Drop for SpscSender<T, S> where
    S: Subscriber<T> + 'static + Send, T: 'static + Send
{
    fn drop(&mut self) {
        unsafe{ self.subscriber.get().close() };
    }
}

struct SubscriberLink<T, S> {
    queue: Queue<T>,
    status: AtomicUsize,
    subscriber: S,
}

impl<T, S> SubscriberLink<T, S> where
    S: Subscriber<T> + 'static + Send, T: 'static + Send
{
    fn new_value(&self, value: T) {
        self.queue.push(value);
        self.notify();
    }

    fn close(&self) {
        match self.status.swap(STREAM_CLOSED, SeqCst) {
            WORKING => {},  // subscriber is currently working and will call
                            // the on_close function when done
            IDLE => self.notify(),  // subscriber is idle -> start it again to
                                    //pop all remaining values and call on_close
            STREAM_CLOSED => panic!("closing already closed stream"),
            _ => unreachable!()
        }
    }

    fn notify(&self) {
        let old_status = self.status.compare_and_swap(IDLE, WORKING, SeqCst);
        match old_status {
            IDLE | STREAM_CLOSED => {
                // IDLE: subscriber was IDLE and is now WORKING. We have marked
                // it as WORKING and have thus exclusive access. We will set the
                // state back to IDLE at the end.
                // STREAM_CLOSED: Stream was closed and there won't be any new
                // values. But maybe there are still values in the queue. Pop
                // them and then call on_close and drop self.

                // we have exclusive access
                let s: &mut Self = unsafe{ &mut *(self as *const _ as *mut _) };

                global::spawn(move || {
                    unsafe{ s.pop_values() };
                    // set the state back to IDLE if it was WORKING before
                    let st = s.status.compare_and_swap(WORKING, IDLE, SeqCst);
                    match st {
                        WORKING => {
                            // status was updated from WORKING to IDLE
                            // are there still values in the queue?
                            if let Some(_) = s.queue.peek() {
                                s.notify();
                            }
                        },
                        STREAM_CLOSED => {
                            // status was NOT updated -> still exclusive access
                            // stream closed, sender doesn't exist anymore
                            // check for last new values
                            unsafe{ s.pop_values() };

                            // take ownership
                            let s: Box<Self> = unsafe {
                                Box::from_raw(s as *mut _)};

                            s.subscriber.on_close();
                        },
                        _ => unreachable!()
                    }
                });
            },
            WORKING => {}, // subscriber already working
            _ => unreachable!()
        }
    }

    // unsafe because there must be only one popper at any time
    unsafe fn pop_values(&mut self) {
        while let Some(value) = self.queue.pop() {
            self.subscriber.on_value(value);
        }
    }
}

pub struct FnSubscriber<f>(f);

impl<T, f> Subscriber<T> for FnSubscriber<f> where f: FnMut(T) {
    fn on_value(&mut self, value: T) {(self.0)(value)}

    fn on_close(self) {println!("closed")}
}
