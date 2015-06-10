use std::cell::UnsafeCell;
use marble::ringbuffer::RingBuffer;
use super::task::Task;

const MAX_TASKS: usize = 64;

static GLOBAL: GlobalDataCell = GlobalDataCell(UnsafeCell{value: None});

struct GlobalDataCell(UnsafeCell<Option<GlobalData>>);
unsafe impl Sync for GlobalDataCell {}

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
