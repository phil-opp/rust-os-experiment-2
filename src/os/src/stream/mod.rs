pub use self::spsc::{SpscSender, stream};
pub use self::mpsc::{MpscSender, mpsc_stream};

mod basic;
mod spsc;
mod mpsc;

mod spsc_old;

pub trait Subscriber<T> {
    fn on_value(&mut self, value: T);

    fn on_close(&mut self) {}

    fn on_close_boxed(mut self: Box<Self>) {
        self.on_close()
    }

    fn on_close_unboxed(mut self) where Self: Sized {
        self.on_close()
    }
}

pub trait Stream {
    type Item;

    fn subscribe(self, subscriber: Box<Subscriber<Self::Item> + Send>);
}
