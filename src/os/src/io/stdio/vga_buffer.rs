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

struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    column_position: usize,
    color_code: VgaColorCode,
    buffer: &'static mut Buffer,
}

impl ScreenWriter {
    pub unsafe fn new(foreground: Color, background: Color) -> ScreenWriter {
        ScreenWriter {
            column_position: 0,
            color_code: VgaColorCode::new(foreground, background),
            buffer: &mut *BUFFER,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        const NEWLINE: u8 = '\n' as u8;

        match byte {
            NEWLINE => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = Char {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn set_colors(&mut self, foreground: Color, background: Color) {
        self.color_code = VgaColorCode::new(foreground, background)
    }

    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT-1) {
            self.buffer.chars[row] = self.buffer.chars[row + 1]
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii_character: ' ' as u8,
            color_code: self.color_code,
        };
        self.buffer.chars[row] = [blank; BUFFER_WIDTH];
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row)
        }
    }
}


impl Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            let byte = match byte {
                0 => 'N' as u8,
                b => b,
            };
            self.write_byte(byte)
        }
        Ok(())
    }
}
