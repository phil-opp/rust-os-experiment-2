use stream::SpscSender;
use std::boxed::into_raw;
use std::cell::RefCell;
use std::fmt::{Write, Result};

pub struct ThreadLocalData {
    pub stdout: Box<Write>,
}

struct Dummy;

impl Write for Dummy {
    fn write_str(&mut self, _msg: &str) -> Result {
        Ok(())
    }
}

pub fn init() {
    let thread_local = RefCell::new(ThreadLocalData {
        stdout: Box::new(Dummy),
    });

    unsafe {
        let address = into_raw(Box::new(thread_local)) as usize;
        asm!("mov fs:0, $0" :: "r"(address) :: "intel", "volatile");
    }
}

pub fn thread_local_data<'a>() -> &'a RefCell<ThreadLocalData> {
    let address: usize;
    unsafe {
        asm!("mov $0, fs:0" : "=r"(address) ::: "intel");
        &*(address as *const RefCell<ThreadLocalData>)
    }
}
