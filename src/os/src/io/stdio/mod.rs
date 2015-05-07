use std::fmt;
use self::vga_buffer::Color;
use stream::SpscStream;

pub mod vga_buffer;

pub fn init() {
    vga_buffer::set_color(Color::Black, Color::White);
    vga_buffer::clear_screen();
}

#[no_mangle]
pub extern fn print_to_stdout(args: fmt::Arguments) {
    vga_buffer::write(args);
}