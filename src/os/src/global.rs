use std::cell::UnsafeCell;
use marble::ringbuffer::RingBuffer;
use super::task::Task;

const MAX_TASKS: usize = 64;

static GLOBAL: GlobalDataCell<GlobalData> = GlobalDataCell(UnsafeCell{value: None});

struct GlobalDataCell<T>(UnsafeCell<Option<T>>);
unsafe impl<T> Sync for GlobalDataCell<T> where T: Sync {}

pub struct GlobalData {
    pub tasks: RingBuffer<Task>,
}

pub unsafe fn init() {
    let global = GlobalData{
        tasks: RingBuffer::new(MAX_TASKS),
    };
    *GLOBAL.0.get() = Some(global);
}

pub fn data<'a>() -> &'a GlobalData {
    GLOBAL.0.value.as_ref().unwrap()
}

// TODO Future<T>
pub fn spawn<F>(f: F) where F: FnOnce() + 'static + Send {
    data().tasks.put(Task::new(f)).unwrap()
}

pub fn run_next_task() {
    data().tasks.get().unwrap().run()
}
