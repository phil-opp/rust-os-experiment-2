#![feature(no_std, lang_items)]
#![no_std]

extern crate rlibc;

pub mod unimplemented_functions;

#[no_mangle]
pub extern fn main() {
    loop{}
}
