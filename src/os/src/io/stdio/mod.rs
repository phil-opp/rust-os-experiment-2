use std::fmt::{self, Write, Result};
use stream::{Sender, SpscSender, Subscriber};
use self::vga_buffer::{ScreenWriter, Color};
use thread;

mod vga_buffer;

pub struct Stdout(SpscSender<String, StdoutSubscriber>);

impl Write for Stdout {
    fn write_str(&mut self, msg: &str) -> Result {
        self.0.send(msg.to_string());
        Ok(())
    }
}

struct StdoutSubscriber {
    screen_writer: ScreenWriter,
}

impl Subscriber<String> for StdoutSubscriber {
    fn on_value(&mut self, value: String) {
        self.screen_writer.write_str(&value);
    }
    fn on_close(self) {}
}

pub unsafe fn init() {
    let mut screen_writer = ScreenWriter::new(Color::Black, Color::White);
    //screen_writer.clear_screen();
    let stdout = Some(Stdout(SpscSender::new(StdoutSubscriber{
        screen_writer: screen_writer,
    })));
    thread::thread_local_data().borrow_mut().stdout = stdout;
}

#[no_mangle]
pub extern fn print_to_stdout(args: fmt::Arguments) {
    thread::thread_local_data().borrow_mut().stdout.as_mut().unwrap()
        .write_fmt(args);
}

#[lang = "panic_fmt"]
extern fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    let mut err_writer = unsafe {
        ScreenWriter::new(Color::White, Color::Red)
    };
    err_writer.write_fmt(format_args!("\nPANIC: `{}` in `{}` in line `{}`",
        msg, file, line));
    loop {}
}
