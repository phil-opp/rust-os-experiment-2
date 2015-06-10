#![feature(std_misc, alloc, core, unique, lang_items, asm, const_fn)]

pub use init::MultibootHeader;
use std::fmt;

#[macro_use]
extern crate bitflags;

extern crate marble;

pub mod io;
pub mod stream;
pub mod allocator;
mod init;
mod thread;
pub mod interrupts;

pub unsafe fn init(multiboot: init::MultibootHeader) {
    init::init(multiboot);
}

pub unsafe fn enable_interrupts() {
    asm!("sti" :::: "volatile");
}

unsafe fn disable_interrupts() {
    asm!("cli" :::: "volatile");
}

unsafe fn out_byte(port: u16, data: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
}
unsafe fn in_byte(port: u16) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al" : "={al}"(ret) : "{dx}"(port) :: "volatile");
    ret
}
