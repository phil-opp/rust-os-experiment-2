#![feature(lang_items, asm, const_fn)]
#![feature(optin_builtin_traits)]
#![feature(core_intrinsics, box_raw, ptr_as_ref, fnbox)]
#![feature(spsc_queue, mpsc_queue)]

pub use init::MultibootHeader;

#[macro_use]
extern crate bitflags;

extern crate marble;

pub mod io;
pub mod stream;
pub mod allocator;
mod init;
pub mod global; // TODO make private
mod thread;
mod task;
pub mod interrupts;

pub unsafe fn init(multiboot: init::MultibootHeader) {
    init::init(multiboot);
}

pub unsafe fn enable_interrupts() {
    asm!("sti" :::: "volatile");
}

#[allow(dead_code)]
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
