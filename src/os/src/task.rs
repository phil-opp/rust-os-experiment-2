use std::boxed::FnBox;

pub struct Task(Box<FnBox()>);

impl Task {
    pub fn new<F>(f: F) -> Task where F: FnOnce() + 'static + Send {
        Task(Box::new(f))
    }
}
