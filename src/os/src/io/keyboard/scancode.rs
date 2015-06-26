use super::KeyPress;
use super::KeyPress::*;
use super::Key::*;

pub struct ScanCode(usize);

impl ScanCode {
    pub fn new(code: usize) -> ScanCode {
        ScanCode(code)
    }
}

struct Parser {
    modifier_bytes: Vec<ScanCode>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            modifier_bytes: Vec::with_capacity(7),
        }
    }

    pub fn parse_code(&mut self, code: ScanCode) -> Option<KeyPress> {
        let key_press = match self.modifier_bytes.len() {
            0 => match code.0 {
                0x01 => Some(KeyPressed(F9)),
                0x03 => Some(KeyPressed(F5)),
                0x04 => Some(KeyPressed(F3)),
                0x05 => Some(KeyPressed(F1)),
                0x06 => Some(KeyPressed(F2)),
                0x07 => Some(KeyPressed(F12)),
                0x09 => Some(KeyPressed(F10)),
                0x0A => Some(KeyPressed(F8)),
                0x0B => Some(KeyPressed(F6)),
                0x0C => Some(KeyPressed(F4)),
                0x0D => Some(KeyPressed(Tab)),
                0x0E => Some(KeyPressed(BackTick)),

                0x11 => Some(KeyPressed(LeftAlt)),
                0x12 => Some(KeyPressed(LeftShift)),
                0x14 => Some(KeyPressed(LeftCtrl)),
                0x15 => Some(KeyPressed(Char_q)),
                0x16 => Some(KeyPressed(Number_1)),
                0x1A => Some(KeyPressed(Char_z)),
                0x1B => Some(KeyPressed(Char_s)),
                0x1C => Some(KeyPressed(Char_a)),
                0x1D => Some(KeyPressed(Char_w)),
                0x1E => Some(KeyPressed(Number_2)),

                0x21 => Some(KeyPressed(Char_c)),
                0x22 => Some(KeyPressed(Char_x)),
                0x23 => Some(KeyPressed(Char_d)),
                0x24 => Some(KeyPressed(Char_e)),
                0x25 => Some(KeyPressed(Number_4)),
                0x26 => Some(KeyPressed(Number_3)),
                0x29 => Some(KeyPressed(Space)),
                0x2A => Some(KeyPressed(Char_v)),
                0x2B => Some(KeyPressed(Char_f)),
                0x2C => Some(KeyPressed(Char_t)),
                0x2D => Some(KeyPressed(Char_r)),
                0x2E => Some(KeyPressed(Number_5)),

                0x31 => Some(KeyPressed(Char_n)),
                0x32 => Some(KeyPressed(Char_b)),
                0x33 => Some(KeyPressed(Char_h)),
                0x34 => Some(KeyPressed(Char_g)),
                0x35 => Some(KeyPressed(Char_y)),
                0x36 => Some(KeyPressed(Number_6)),
                0x3A => Some(KeyPressed(Char_m)),
                0x3B => Some(KeyPressed(Char_j)),
                0x3C => Some(KeyPressed(Char_u)),
                0x3D => Some(KeyPressed(Number_7)),
                0x3E => Some(KeyPressed(Number_8)),

                0x41 => Some(KeyPressed(Comma)),
                0x42 => Some(KeyPressed(Char_k)),
                0x43 => Some(KeyPressed(Char_i)),
                0x44 => Some(KeyPressed(Char_o)),
                0x45 => Some(KeyPressed(Number_0)),
                0x46 => Some(KeyPressed(Number_9)),
                0x49 => Some(KeyPressed(Point)),
                0x4A => Some(KeyPressed(Slash)),
                0x4B => Some(KeyPressed(Char_l)),
                0x4C => Some(KeyPressed(Semicolon)),
                0x4D => Some(KeyPressed(Char_p)),
                0x4E => Some(KeyPressed(Minus)),

                0x52 => Some(KeyPressed(SingleQuote)),
                0x54 => Some(KeyPressed(LeftBracket)),
                0x55 => Some(KeyPressed(Equal)),
                0x58 => Some(KeyPressed(CapsLock)),
                0x59 => Some(KeyPressed(RightShift)),
                0x5A => Some(KeyPressed(Enter)),
                0x5B => Some(KeyPressed(RightBracket)),
                0x5D => Some(KeyPressed(Backslash)),

                0x66 => Some(KeyPressed(Backspace)),
                0x69 => Some(KeyPressed(Keypad_1)),
                0x6B => Some(KeyPressed(Keypad_4)),
                0x6C => Some(KeyPressed(Keypad_7)),

                0x70 => Some(KeyPressed(Keypad_0)),
                0x71 => Some(KeyPressed(KeypadPoint)),
                0x72 => Some(KeyPressed(Keypad_2)),
                0x73 => Some(KeyPressed(Keypad_5)),
                0x74 => Some(KeyPressed(Keypad_6)),
                0x75 => Some(KeyPressed(Keypad_8)),
                0x76 => Some(KeyPressed(Escape)),
                0x77 => Some(KeyPressed(NumberLock)),
                0x78 => Some(KeyPressed(F11)),
                0x79 => Some(KeyPressed(KeypadPlus)),
                0x7A => Some(KeyPressed(Keypad_3)),
                0x7B => Some(KeyPressed(KeypadMinus)),
                0x7C => Some(KeyPressed(KeypadStar)),
                0x7D => Some(KeyPressed(Keypad_9)),
                0x7E => Some(KeyPressed(ScrollLock)),

                0x83 => Some(KeyPressed(F7)),

                0xE0 | 0xE1 | 0xF0 => {self.modifier_bytes.push(code); None},

                _ => unreachable!(),
            },
            1 => match self.modifier_bytes[0].0 {
                0xE0 => match code.0 {
                    0x10 => Some(KeyPressed(MultimediaSearch)),
                    0x11 => Some(KeyPressed(RightAlt)),
                    0x14 => Some(KeyPressed(RightCtrl)),
                    0x15 => Some(KeyPressed(MultimediaPrevious)),
                    0x18 => Some(KeyPressed(MultimediaFavourites)),
                    0x1F => Some(KeyPressed(LeftGui)),

                    0x20 => Some(KeyPressed(MultimediaRefresh)),
                    0x21 => Some(KeyPressed(MultimediaVolumeDown)),
                    0x23 => Some(KeyPressed(MultimediaMute)),
                    0x27 => Some(KeyPressed(RightGui)),
                    0x28 => Some(KeyPressed(MultimediaStop)),
                    0x2B => Some(KeyPressed(MultimediaCalculator)),
                    0x2F => Some(KeyPressed(Apps)),

                    0x30 => Some(KeyPressed(MultimediaForward)),
                    0x32 => Some(KeyPressed(MultimediaVolumeUp)),
                    0x34 => Some(KeyPressed(MultimediaPlayPause)),
                    0x37 => Some(KeyPressed(AcpiPower)),
                    0x38 => Some(KeyPressed(MultimediaBack)),
                    0x3A => Some(KeyPressed(MultimediaHome)),
                    0x3B => Some(KeyPressed(MultimediaStop)),
                    0x3F => Some(KeyPressed(AcpiSleep)),

                    0x40 => Some(KeyPressed(MultimediaComputer)),
                    0x48 => Some(KeyPressed(MultimediaEmail)),
                    0x4A => Some(KeyPressed(KeypadSlash)),
                    0x4D => Some(KeyPressed(MultmediaNext)),

                    0x50 => Some(KeyPressed(MultmediaMediaSelect)),
                    0x5A => Some(KeyPressed(KeypadEnter)),
                    0x5E => Some(KeyPressed(AcpiWake)),

                    0x69 => Some(KeyPressed(End)),
                    0x6B => Some(KeyPressed(CursorLeft)),
                    0x6C => Some(KeyPressed(Home)),

                    0x70 => Some(KeyPressed(Insert)),
                    0x71 => Some(KeyPressed(Delete)),
                    0x72 => Some(KeyPressed(CursorDown)),
                    0x74 => Some(KeyPressed(CursorRight)),
                    0x75 => Some(KeyPressed(CursorUp)),
                    0x7A => Some(KeyPressed(PageDown)),
                    0x7D => Some(KeyPressed(PageUp)),

                    0x12 | 0xF0 => {
                        self.modifier_bytes.push(code); None},

                    _ => unreachable!(),
                },
                0xE1 => {
                    assert!(code.0 == 0x14);
                    self.modifier_bytes.push(code);
                    None
                },
                0xF0 => match code.0 {
                    0x01 => Some(KeyReleased(F9)),
                    0x03 => Some(KeyReleased(F5)),
                    0x04 => Some(KeyReleased(F3)),
                    0x05 => Some(KeyReleased(F1)),
                    0x06 => Some(KeyReleased(F2)),
                    0x07 => Some(KeyReleased(F12)),
                    0x09 => Some(KeyReleased(F10)),
                    0x0A => Some(KeyReleased(F8)),
                    0x0B => Some(KeyReleased(F6)),
                    0x0C => Some(KeyReleased(F4)),
                    0x0D => Some(KeyReleased(Tab)),
                    0x0E => Some(KeyReleased(BackTick)),

                    0x11 => Some(KeyReleased(LeftAlt)),
                    0x12 => Some(KeyReleased(LeftShift)),
                    0x14 => Some(KeyReleased(LeftCtrl)),
                    0x15 => Some(KeyReleased(Char_q)),
                    0x16 => Some(KeyReleased(Number_1)),
                    0x1A => Some(KeyReleased(Char_z)),
                    0x1B => Some(KeyReleased(Char_s)),
                    0x1C => Some(KeyReleased(Char_a)),
                    0x1D => Some(KeyReleased(Char_w)),
                    0x1E => Some(KeyReleased(Number_2)),

                    0x21 => Some(KeyReleased(Char_c)),
                    0x22 => Some(KeyReleased(Char_x)),
                    0x23 => Some(KeyReleased(Char_d)),
                    0x24 => Some(KeyReleased(Char_e)),
                    0x25 => Some(KeyReleased(Number_4)),
                    0x26 => Some(KeyReleased(Number_3)),
                    0x29 => Some(KeyReleased(Space)),
                    0x2A => Some(KeyReleased(Char_v)),
                    0x2B => Some(KeyReleased(Char_f)),
                    0x2C => Some(KeyReleased(Char_t)),
                    0x2D => Some(KeyReleased(Char_r)),
                    0x2E => Some(KeyReleased(Number_5)),

                    0x31 => Some(KeyReleased(Char_n)),
                    0x32 => Some(KeyReleased(Char_b)),
                    0x33 => Some(KeyReleased(Char_h)),
                    0x34 => Some(KeyReleased(Char_g)),
                    0x35 => Some(KeyReleased(Char_y)),
                    0x36 => Some(KeyReleased(Number_6)),
                    0x3A => Some(KeyReleased(Char_m)),
                    0x3B => Some(KeyReleased(Char_j)),
                    0x3C => Some(KeyReleased(Char_u)),
                    0x3D => Some(KeyReleased(Number_7)),
                    0x3E => Some(KeyReleased(Number_8)),

                    0x41 => Some(KeyReleased(Comma)),
                    0x42 => Some(KeyReleased(Char_k)),
                    0x43 => Some(KeyReleased(Char_i)),
                    0x44 => Some(KeyReleased(Char_o)),
                    0x45 => Some(KeyReleased(Number_0)),
                    0x46 => Some(KeyReleased(Number_9)),
                    0x49 => Some(KeyReleased(Point)),
                    0x4A => Some(KeyReleased(Slash)),
                    0x4B => Some(KeyReleased(Char_l)),
                    0x4C => Some(KeyReleased(Semicolon)),
                    0x4D => Some(KeyReleased(Char_p)),
                    0x4E => Some(KeyReleased(Minus)),

                    0x52 => Some(KeyReleased(SingleQuote)),
                    0x54 => Some(KeyReleased(LeftBracket)),
                    0x55 => Some(KeyReleased(Equal)),
                    0x58 => Some(KeyReleased(CapsLock)),
                    0x59 => Some(KeyReleased(RightShift)),
                    0x5A => Some(KeyReleased(Enter)),
                    0x5B => Some(KeyReleased(RightBracket)),
                    0x5D => Some(KeyReleased(Backslash)),

                    0x66 => Some(KeyReleased(Backspace)),
                    0x69 => Some(KeyReleased(Keypad_1)),
                    0x6B => Some(KeyReleased(Keypad_4)),
                    0x6C => Some(KeyReleased(Keypad_7)),

                    0x70 => Some(KeyReleased(Keypad_0)),
                    0x71 => Some(KeyReleased(KeypadPoint)),
                    0x72 => Some(KeyReleased(Keypad_2)),
                    0x73 => Some(KeyReleased(Keypad_5)),
                    0x74 => Some(KeyReleased(Keypad_6)),
                    0x75 => Some(KeyReleased(Keypad_8)),
                    0x76 => Some(KeyReleased(Escape)),
                    0x77 => Some(KeyReleased(NumberLock)),
                    0x78 => Some(KeyReleased(F11)),
                    0x79 => Some(KeyReleased(KeypadPlus)),
                    0x7A => Some(KeyReleased(Keypad_3)),
                    0x7B => Some(KeyReleased(KeypadMinus)),
                    0x7C => Some(KeyReleased(KeypadStar)),
                    0x7D => Some(KeyReleased(Keypad_9)),
                    0x7E => Some(KeyReleased(ScrollLock)),

                    0x83 => Some(KeyReleased(F7)),

                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            2 => match self.modifier_bytes[0].0 {
                0xE0 => match self.modifier_bytes[1].0 {
                    0x12 => match code.0 {
                        0xE0 => {self.modifier_bytes.push(code); None},
                        _ => unreachable!(),
                    },
                    0xF0 => match code.0 {
                        0x10 => Some(KeyReleased(MultimediaSearch)),
                        0x11 => Some(KeyReleased(RightAlt)),
                        0x14 => Some(KeyReleased(RightCtrl)),
                        0x15 => Some(KeyReleased(MultimediaPrevious)),
                        0x18 => Some(KeyReleased(MultimediaFavourites)),
                        0x1F => Some(KeyReleased(LeftGui)),

                        0x20 => Some(KeyReleased(MultimediaRefresh)),
                        0x21 => Some(KeyReleased(MultimediaVolumeDown)),
                        0x23 => Some(KeyReleased(MultimediaMute)),
                        0x27 => Some(KeyReleased(RightGui)),
                        0x28 => Some(KeyReleased(MultimediaStop)),
                        0x2B => Some(KeyReleased(MultimediaCalculator)),
                        0x2F => Some(KeyReleased(Apps)),

                        0x30 => Some(KeyReleased(MultimediaForward)),
                        0x32 => Some(KeyReleased(MultimediaVolumeUp)),
                        0x34 => Some(KeyReleased(MultimediaPlayPause)),
                        0x37 => Some(KeyReleased(AcpiPower)),
                        0x38 => Some(KeyReleased(MultimediaBack)),
                        0x3A => Some(KeyReleased(MultimediaHome)),
                        0x3B => Some(KeyReleased(MultimediaStop)),
                        0x3F => Some(KeyReleased(AcpiSleep)),

                        0x40 => Some(KeyReleased(MultimediaComputer)),
                        0x48 => Some(KeyReleased(MultimediaEmail)),
                        0x4A => Some(KeyReleased(KeypadSlash)),
                        0x4D => Some(KeyReleased(MultmediaNext)),

                        0x50 => Some(KeyReleased(MultmediaMediaSelect)),
                        0x5A => Some(KeyReleased(KeypadEnter)),
                        0x5E => Some(KeyReleased(AcpiWake)),

                        0x69 => Some(KeyReleased(End)),
                        0x6B => Some(KeyReleased(CursorLeft)),
                        0x6C => Some(KeyReleased(Home)),

                        0x70 => Some(KeyReleased(Insert)),
                        0x71 => Some(KeyReleased(Delete)),
                        0x72 => Some(KeyReleased(CursorDown)),
                        0x74 => Some(KeyReleased(CursorRight)),
                        0x75 => Some(KeyReleased(CursorUp)),
                        0x7A => Some(KeyReleased(PageDown)),
                        0x7D => Some(KeyReleased(PageUp)),

                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                0xE1 => {
                    assert!(self.modifier_bytes[1].0 == 0x14);
                    assert!(code.0 == 0x77);
                    self.modifier_bytes.push(code);
                    None
                },
                _ => unreachable!(),
            },
            3 => match self.modifier_bytes[0].0 {
                0xE0 => match self.modifier_bytes[1].0 {
                    0x12 => {
                        assert!(self.modifier_bytes[2].0 == 0xE0);
                        assert!(code.0 == 0x7C);
                        Some(KeyPressed(PrintScreen))
                    },
                    0xF0 => {
                        assert!(self.modifier_bytes[2].0 == 0x7C);
                        assert!(code.0 == 0xE0);
                        self.modifier_bytes.push(code);
                        None
                    },
                    _ => unreachable!(),
                },
                0xE1 => {
                    assert!(self.modifier_bytes[1].0 == 0x14);
                    assert!(self.modifier_bytes[2].0 == 0x77);
                    assert!(code.0 == 0xE1);
                    self.modifier_bytes.push(code);
                    None
                },
                _ => unreachable!(),
            },
            4 => match self.modifier_bytes[0].0 {
                0xE0 => {
                    assert!(self.modifier_bytes[1].0 == 0xF0);
                    assert!(self.modifier_bytes[2].0 == 0x7C);
                    assert!(self.modifier_bytes[3].0 == 0xE0);
                    assert!(code.0 == 0xF0);
                    self.modifier_bytes.push(code);
                    None
                },
                0xE1 => {
                    assert!(self.modifier_bytes[1].0 == 0x14);
                    assert!(self.modifier_bytes[2].0 == 0x77);
                    assert!(self.modifier_bytes[3].0 == 0xE1);
                    assert!(code.0 == 0xF0);
                    self.modifier_bytes.push(code);
                    None
                },
                _ => unreachable!(),
            },
            5 => match self.modifier_bytes[0].0 {
                0xE0 => {
                    assert!(self.modifier_bytes[1].0 == 0xF0);
                    assert!(self.modifier_bytes[2].0 == 0x7C);
                    assert!(self.modifier_bytes[3].0 == 0xE0);
                    assert!(self.modifier_bytes[4].0 == 0xF0);
                    assert!(code.0 == 0x12);
                    Some(KeyReleased(PrintScreen))
                },
                0xE1 => {
                    assert!(self.modifier_bytes[1].0 == 0x14);
                    assert!(self.modifier_bytes[2].0 == 0x77);
                    assert!(self.modifier_bytes[3].0 == 0xE1);
                    assert!(self.modifier_bytes[4].0 == 0xF0);
                    assert!(code.0 == 0x14);
                    self.modifier_bytes.push(code);
                    None
                },
                _ => unreachable!(),
            },
            6 => {
                assert!(self.modifier_bytes[0].0 == 0xE1);
                assert!(self.modifier_bytes[1].0 == 0x14);
                assert!(self.modifier_bytes[2].0 == 0x77);
                assert!(self.modifier_bytes[3].0 == 0xE1);
                assert!(self.modifier_bytes[4].0 == 0xF0);
                assert!(self.modifier_bytes[5].0 == 0x14);
                assert!(code.0 == 0xF0);
                self.modifier_bytes.push(code);
                None
            },
            7 => {
                assert!(self.modifier_bytes[0].0 == 0xE1);
                assert!(self.modifier_bytes[1].0 == 0x14);
                assert!(self.modifier_bytes[2].0 == 0x77);
                assert!(self.modifier_bytes[3].0 == 0xE1);
                assert!(self.modifier_bytes[4].0 == 0xF0);
                assert!(self.modifier_bytes[5].0 == 0x14);
                assert!(self.modifier_bytes[6].0 == 0xF0);
                assert!(code.0 == 0x77);
                Some(KeyPressed(Pause))
            },
            _ => unreachable!(),
        };
        if let Some(_) = key_press {
            self.modifier_bytes.clear()
        }
        key_press
    }
}
