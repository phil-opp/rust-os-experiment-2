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

pub fn init(multiboot: init::MultibootHeader) {
    init::init(multiboot);
}

#[lang = "panic_fmt"]
fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    use io::stdio::vga_buffer::*;
    set_color(Color::White, Color::Red);
    println!("\n\nPANIC");
    loop {}
}
