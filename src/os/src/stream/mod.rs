pub use self::spsc::{SpscSender, stream};
pub use self::mpsc::{MpscSender, mpsc_stream};

mod spsc;
mod mpsc;

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

    fn subscribe<Sub>(self, subscriber: Sub) where
        Sub: Subscriber<Self::Item> + Send + 'static;

    fn map<B, F>(self, f:F) -> Map<Self, F> where
        Self:Sized, F: FnMut(Self::Item) -> B,
    {
        Map{stream: self, f: f}
    }

    fn filter<P>(self, predicate: P) -> Filter<Self, P> where
        Self: Sized, P: FnMut(Self::Item) -> bool
    {
        Filter{stream: self, predicate: predicate}
    }
}

pub struct Map<S, F> {
    stream: S,
    f: F,
}

impl<B, S: Stream, F> Stream for Map<S, F> where
    F: FnMut(S::Item) -> B + Send + 'static
{
    type Item = B;

    #[inline]
    fn subscribe<Sub>(self, subscriber: Sub) where
        Sub: Subscriber<B> + Send + 'static
    {
        self.stream.subscribe(MapSubscriber {
            subscriber: subscriber,
            f: self.f,
        });
    }
}

struct MapSubscriber<Sub, F> {
    subscriber: Sub,
    f: F,
}

impl<T, B, Sub, F> Subscriber<T> for MapSubscriber<Sub, F> where
    F: FnMut(T) -> B, Sub: Subscriber<B>,
{
    fn on_value(&mut self, value: T) {
        self.subscriber.on_value((self.f)(value))
    }

    fn on_close(&mut self) {self.subscriber.on_close()}

    fn on_close_boxed(self: Box<Self>) {self.subscriber.on_close_unboxed()}

    fn on_close_unboxed(self) where Self: Sized {
        self.subscriber.on_close_unboxed()
    }
}

pub struct Filter<S, P> {
    stream: S,
    predicate: P,
}

impl<S: Stream, P> Stream for Filter<S, P> where
    P: FnMut(&S::Item) -> bool + Send + 'static
{
    type Item = S::Item;

    #[inline]
    fn subscribe<Sub>(self, subscriber: Sub) where
        Sub: Subscriber<S::Item> + Send + 'static
    {
        self.stream.subscribe(FilterSubscriber {
            subscriber: subscriber,
            predicate: self.predicate,
        });
    }
}

struct FilterSubscriber<Sub, P> {
    subscriber: Sub,
    predicate: P,
}

impl<T, Sub, P> Subscriber<T> for FilterSubscriber<Sub, P> where
    P: FnMut(&T) -> bool, Sub: Subscriber<T>,
{
    fn on_value(&mut self, value: T) {
        if (self.predicate)(&value) {
            self.subscriber.on_value(value)
        }
    }

    fn on_close(&mut self) {self.subscriber.on_close()}

    fn on_close_boxed(self: Box<Self>) {self.subscriber.on_close_unboxed()}

    fn on_close_unboxed(self) where Self: Sized {
        self.subscriber.on_close_unboxed()
    }
}
