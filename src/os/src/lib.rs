#![feature(std_misc, alloc, core, unique, lang_items, asm)]

pub use init::MultibootHeader;
use std::fmt;

#[macro_use]
extern crate bitflags;

pub mod io;
pub mod stream;
pub mod allocator;
mod init;
mod thread;

pub unsafe fn init(multiboot: init::MultibootHeader) {
    init::init(multiboot);
}
