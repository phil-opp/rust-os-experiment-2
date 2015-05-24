use std::fmt;
use stream::{Sender, Subscriber};
use self::vga_buffer::Color;
use thread;

pub struct StdoutSubscriber;

impl Subscriber<String> for StdoutSubscriber {
    fn on_value(&mut self, value: String) {
        vga_buffer::write_str(&value);
    }
    fn on_close(self) {}
}

pub mod vga_buffer;

pub fn init() {
    vga_buffer::set_color(Color::Black, Color::White);
    vga_buffer::clear_screen();
}

#[no_mangle]
pub extern fn print_to_stdout(args: fmt::Arguments) {
    let msg = format!("{}", args);
    thread::thread_local_data().borrow_mut().stdout.send(msg);
}
