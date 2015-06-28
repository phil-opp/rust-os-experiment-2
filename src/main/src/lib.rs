#![feature(lang_items)]

use os::stream::*;
use os::global;
use std::fmt::Debug;

extern crate os;

pub mod unimplemented_functions;

#[no_mangle]
pub extern fn main(multiboot: os::MultibootHeader) {
    unsafe{os::init(multiboot)};

    println!("Hello World!");
    unsafe{os::enable_interrupts()};

    {
        let (s, r) = stream();

        s.send(H(1));
        s.send(H(2));
        s.close();
    }

    let (tx, rx) = os::stream::stream();

    tx.send("test1");
    tx.send("test2");
    rx.subscribe(Dummy);
    tx.send("test3");
    tx.close();

    print!("this is a ");
    println!("test message...");
    loop{
        //for _ in 0..10000 {}
        //println!("running next task...");
        global::run_next_task()
    }
}

struct Dummy;

impl<T> Subscriber<T> for Dummy where T: Debug {
    fn on_value(&mut self, value: T) {
        println!("Dummy: {:?}", value);
    }
    fn on_close(&mut self) {
        println!("Dummy: closed");
    }
}


struct H(i32);

impl Drop for H {
    fn drop(&mut self) {
        println!("dropping H {}", self.0);
    }
}
