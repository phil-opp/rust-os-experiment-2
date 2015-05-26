use io::stdio::Stdout;
use std::boxed::into_raw;
use std::cell::RefCell;
use std::fmt::{Write, Result};

pub struct ThreadLocalData {
    pub stdout: Option<Stdout>,
}

pub fn init() {
    let thread_local = RefCell::new(ThreadLocalData {
        stdout: None,
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
