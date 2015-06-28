use std::cell::UnsafeCell;
use marble::ringbuffer::RingBuffer;
use stream::{mpsc_stream, MpscSender};
use io::keyboard::{self, ScanCode};
use super::task::Task;

const MAX_TASKS: usize = 64;

static GLOBAL: GlobalDataCell<GlobalData> = GlobalDataCell(
    UnsafeCell::new(None));

struct GlobalDataCell<T>(UnsafeCell<Option<T>>);
unsafe impl<T> Sync for GlobalDataCell<T> where T: Sync {}

pub struct GlobalData {
    pub tasks: RingBuffer<Task>,
    pub key_presses: MpscSender<ScanCode>,
}

pub unsafe fn init() {
    let (key_presses_sender, key_presses) = mpsc_stream();

    let global = GlobalData{
        tasks: RingBuffer::new(MAX_TASKS),
        key_presses: key_presses_sender,
    };
    *GLOBAL.0.get() = Some(global);

    keyboard::init(key_presses);
}

pub fn data<'a>() -> &'a GlobalData {
    unsafe{(*GLOBAL.0.get()).as_ref()}.unwrap()
}

// TODO Future<T>
pub fn spawn<F>(f: F) where F: FnOnce() + 'static + Send {
    data().tasks.put(Task::new(f)).unwrap()
}

pub fn run_next_task() {
    data().tasks.get().unwrap().run()
}
