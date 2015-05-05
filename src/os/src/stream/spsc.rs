use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::spsc_queue::Queue;
use super::{Sender, Subscriber};

pub struct SpscSender<T, S> {
    subscriber: Box<SubscriberLink<T, S>>
}

impl<T, S> SpscSender<T, S> where S: Subscriber<T> {
    pub fn new(subscriber: S) -> SpscSender<T, S> {
        SpscSender {
            subscriber: Box::new(SubscriberLink{
                queue: unsafe{ Queue::new(0) },
                status: AtomicUsize::new(0),
                subscriber: subscriber,
            }),
        }
    }
}

impl<T, S> Sender<T> for SpscSender<T, S> where S: Subscriber<T> {
    fn send(&self, value: T) {
        self.subscriber.new_value(value);
    }

    fn close(self) {
        self.subscriber.close();
    }
}

struct SubscriberLink<T, S> {
    queue: Queue<T>,
    // 0 = subscriber idle (sender has ownership)
    // 1 = subscriber working
    // 2 = sender closed stream
    status: AtomicUsize,
    subscriber: S,
}

impl<T, S> SubscriberLink<T, S> where S: Subscriber<T> {
    fn new_value(&self, value: T) {
        self.queue.push(value);
        self.notify();
    }

    fn close(self: Box<Self>) {
        match self.status.swap(2, Ordering::SeqCst) {
            1 => {},    // subscriber is currently working and will call
                        // the on_close function when done
            0 => self.notify(), // subscriber is idle -> start it again to 
                                //pop all remaining values and call on_close
            _ => unreachable!()                
        }
        // either way, we won't drop self
        unsafe{ mem::forget(self) }
    }

    fn notify(&self) {
        let old_status = self.status.compare_and_swap(0, 1, Ordering::SeqCst);
        match old_status {
            0 | 2 => { 
                // if ==0 : subscriber was idle and is now working. We have marked
                // it as working (1) have thus temporary ownership. We will set 
                // the state back to idle (0) at the end and forget self to
                // avoid starting the destructor.
                // if ==2: Stream was closed and there won't be any new values. 
                // But maybe there are still values in the queue. Pop them and
                // then call on_close and drop self.

                let mut s: Box<Self> = unsafe {
                    Box::from_raw(self as *const _ as *mut _)};

                // TODO spawn task
                {
                    unsafe{ s.pop_values() };
                    // set the state back to idle (0) if it was working (1) before
                    let st = s.status.compare_and_swap(1, 0, Ordering::SeqCst);
                    match st {
                        1 => {
                            // are there still values in the queue?
                            if let Some(_) = s.queue.peek() {
                                s.notify();
                            }
                            unsafe{ mem::forget(s) } // give up ownership
                        },
                        2 => { // stream closed, sender doesn't exist anymore
                            // check again for new values
                            unsafe{ s.pop_values() };
                            s.subscriber.on_close();
                        }, 
                        _ => unreachable!()
                    }
                }
            },
            1 => {}, // subscriber already working
            _ => unreachable!()
        }
    }

    unsafe fn pop_values(&mut self) {
        while let Some(value) = self.queue.pop() {
            self.subscriber.on_value(value);
        }                        
    }

}
