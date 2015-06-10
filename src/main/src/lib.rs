#![feature(lang_items)]

use os::stream::*;
use os::global;

extern crate os;

pub mod unimplemented_functions;

#[no_mangle]
pub extern fn main(multiboot: os::MultibootHeader) {
    unsafe{os::init(multiboot)};

    println!("Hello World!");
    unsafe{os::enable_interrupts()};

    let s = SpscSender::on_value(|msg| println!("msg: {}", msg));

    s.send("test1");
    s.send("test2");
    s.close();

    print!("this is a ");
    println!("test message...");
    loop{
        //for _ in 0..10000 {}
        //println!("running next task...");
        global::run_next_task()
    }
}
