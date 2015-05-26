use std::fmt;
use stream::{Sender, Subscriber};
use self::vga_buffer::{ScreenWriter, Color};
use thread;

mod vga_buffer;

pub unsafe fn init() {
    let mut screen_writer = ScreenWriter::new(Color::Black, Color::White);
    screen_writer.clear_screen();
    thread::thread_local_data().borrow_mut().stdout = Box::new(screen_writer);
}

#[no_mangle]
pub extern fn print_to_stdout(args: fmt::Arguments) {
    thread::thread_local_data().borrow_mut().stdout.write_fmt(args);
}
