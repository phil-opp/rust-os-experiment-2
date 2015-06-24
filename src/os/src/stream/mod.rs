pub use self::spsc::SpscSender;

mod basic;
mod spsc;

pub trait Sender<T> {
    fn send(&self, value: T);

    fn close(self);
}

pub trait Subscriber<T> {
    fn on_value(&mut self, value: T);

    fn on_close(self: Box<Self>);
}

pub trait Stream {
    type Item;

    fn subscribe(self, subscriber: Box<Subscriber<Self::Item>>);
}
