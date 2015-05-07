use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::sync::atomic::Ordering::Relaxed;
use std::result::Result::Ok;
use std::fmt::{Arguments, Write, Result};

const BUFFER: *mut Buffer = 0xb8000 as *mut _;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
struct VgaColorCode(u8);

impl VgaColorCode {
    fn new(foreground: Color, background: Color) -> VgaColorCode {
        VgaColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[packed]
struct Char {
    ascii_character: u8,
    color_code: VgaColorCode,
}

static WRITER: Writer = Writer {
    position: ATOMIC_USIZE_INIT,
    color_code: ATOMIC_USIZE_INIT, 
};

struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct Writer {
    position: AtomicUsize,
    color_code: AtomicUsize,
}

impl Writer {
    fn write_byte(&self, byte: u8) -> Result {
        const NEWLINE: u8 = '\n' as u8;

        match byte {
            NEWLINE => self.new_line(),
            byte => {
                let color_code = self.color_code.load(Relaxed);
                let char = Char {
                    ascii_character: byte,
                    color_code: VgaColorCode(color_code as u8),
                };

                let position = self.position.fetch_add(1, Relaxed);
                let row = (position / BUFFER_WIDTH) % BUFFER_HEIGHT;
                let col = position % BUFFER_WIDTH;

                unsafe{(*BUFFER).chars[row][col] = char};
                Ok(())
            }
        }
    }

    fn new_line(&self) -> Result {
        let buffer_size = BUFFER_WIDTH * BUFFER_HEIGHT;
        loop {
            let old_position = self.position.load(Relaxed);
            let pos = old_position % buffer_size;
            let new_position = (pos / BUFFER_WIDTH + 1) * BUFFER_WIDTH;
            let old = self.position.compare_and_swap(old_position, new_position, 
                Relaxed);
            if old == old_position {
                return Ok(());
            }
        }

    }
}

// hack to make it possible to implement Write (takes &mut self)
struct WriterRef(&'static Writer);

impl Write for WriterRef {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            try!(self.0.write_byte(byte))
        }
        Ok(())
    }
}

pub fn set_color(foreground: Color, background: Color) {
    let color_code = VgaColorCode::new(foreground, background);
    WRITER.color_code.store(color_code.0 as usize, Relaxed)
}

pub fn clear_screen() {
    let color_code = WRITER.color_code.load(Relaxed);
    let c = Char {
        ascii_character: ' ' as u8, 
        color_code: VgaColorCode(color_code as u8),
    };
    unsafe {
        (*BUFFER).chars = [[c; BUFFER_WIDTH]; BUFFER_HEIGHT]
    }
}

pub fn write(args: Arguments) -> Result {
    WriterRef(&WRITER).write_fmt(args)
}
