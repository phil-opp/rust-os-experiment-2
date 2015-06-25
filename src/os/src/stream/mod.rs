pub use self::spsc::{SpscSender, stream};
pub use self::mpsc::{MpscSender, mpsc_stream};

mod basic;
mod spsc;
mod mpsc;

mod spsc_old;

pub trait Subscriber<T> {
    fn on_value(&mut self, value: T);

    fn on_close(self: Box<Self>);
}

pub trait Stream {
    type Item;

    fn subscribe(self, subscriber: Box<Subscriber<Self::Item> + Send>);
}
