pub use self::spsc::SpscSender;

mod spsc;

pub trait Sender<T> {
    fn send(&self, value: T);
    fn close(self);
}

pub trait Subscriber<T> {
    fn on_value(&mut self, value: T);
    fn on_close(self);
}

pub trait Stream<T> {
    fn subscribe(self, subscriber: Subscriber<T>);
}