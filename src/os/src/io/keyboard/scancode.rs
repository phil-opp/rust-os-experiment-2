use super::KeyPress::{self, KeyPressed, KeyReleased};
use super::Key::{Control, Printable};
use super::ControlKey::*;
use super::PrintableKey::*;

pub struct ScanCode(usize);

impl ScanCode {
    pub fn new(code: usize) -> ScanCode {
        ScanCode(code)
    }
}

pub struct Parser {
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
                0x01 => Some(KeyPressed(Control(F9))),
                0x03 => Some(KeyPressed(Control(F5))),
                0x04 => Some(KeyPressed(Control(F3))),
                0x05 => Some(KeyPressed(Control(F1))),
                0x06 => Some(KeyPressed(Control(F2))),
                0x07 => Some(KeyPressed(Control(F12))),
                0x09 => Some(KeyPressed(Control(F10))),
                0x0A => Some(KeyPressed(Control(F8))),
                0x0B => Some(KeyPressed(Control(F6))),
                0x0C => Some(KeyPressed(Control(F4))),
                0x0D => Some(KeyPressed(Control(Tab))),
                0x0E => Some(KeyPressed(Printable(BackTick))),

                0x11 => Some(KeyPressed(Control(LeftAlt))),
                0x12 => Some(KeyPressed(Control(LeftShift))),
                0x14 => Some(KeyPressed(Control(LeftCtrl))),
                0x15 => Some(KeyPressed(Printable(Q))),
                0x16 => Some(KeyPressed(Printable(Number1))),
                0x1A => Some(KeyPressed(Printable(Z))),
                0x1B => Some(KeyPressed(Printable(S))),
                0x1C => Some(KeyPressed(Printable(A))),
                0x1D => Some(KeyPressed(Printable(W))),
                0x1E => Some(KeyPressed(Printable(Number2))),

                0x21 => Some(KeyPressed(Printable(C))),
                0x22 => Some(KeyPressed(Printable(X))),
                0x23 => Some(KeyPressed(Printable(D))),
                0x24 => Some(KeyPressed(Printable(E))),
                0x25 => Some(KeyPressed(Printable(Number4))),
                0x26 => Some(KeyPressed(Printable(Number3))),
                0x29 => Some(KeyPressed(Printable(Space))),
                0x2A => Some(KeyPressed(Printable(V))),
                0x2B => Some(KeyPressed(Printable(F))),
                0x2C => Some(KeyPressed(Printable(T))),
                0x2D => Some(KeyPressed(Printable(R))),
                0x2E => Some(KeyPressed(Printable(Number5))),

                0x31 => Some(KeyPressed(Printable(N))),
                0x32 => Some(KeyPressed(Printable(B))),
                0x33 => Some(KeyPressed(Printable(H))),
                0x34 => Some(KeyPressed(Printable(G))),
                0x35 => Some(KeyPressed(Printable(Y))),
                0x36 => Some(KeyPressed(Printable(Number6))),
                0x3A => Some(KeyPressed(Printable(M))),
                0x3B => Some(KeyPressed(Printable(J))),
                0x3C => Some(KeyPressed(Printable(U))),
                0x3D => Some(KeyPressed(Printable(Number7))),
                0x3E => Some(KeyPressed(Printable(Number8))),

                0x41 => Some(KeyPressed(Printable(Comma))),
                0x42 => Some(KeyPressed(Printable(K))),
                0x43 => Some(KeyPressed(Printable(I))),
                0x44 => Some(KeyPressed(Printable(O))),
                0x45 => Some(KeyPressed(Printable(Number0))),
                0x46 => Some(KeyPressed(Printable(Number9))),
                0x49 => Some(KeyPressed(Printable(Point))),
                0x4A => Some(KeyPressed(Printable(Slash))),
                0x4B => Some(KeyPressed(Printable(L))),
                0x4C => Some(KeyPressed(Printable(Semicolon))),
                0x4D => Some(KeyPressed(Printable(P))),
                0x4E => Some(KeyPressed(Printable(Minus))),

                0x52 => Some(KeyPressed(Printable(SingleQuote))),
                0x54 => Some(KeyPressed(Printable(LeftBracket))),
                0x55 => Some(KeyPressed(Printable(Equal))),
                0x58 => Some(KeyPressed(Control(CapsLock))),
                0x59 => Some(KeyPressed(Control(RightShift))),
                0x5A => Some(KeyPressed(Control(Enter))),
                0x5B => Some(KeyPressed(Printable(RightBracket))),
                0x5D => Some(KeyPressed(Printable(Backslash))),

                0x66 => Some(KeyPressed(Control(Backspace))),
                0x69 => Some(KeyPressed(Printable(Keypad1))),
                0x6B => Some(KeyPressed(Printable(Keypad4))),
                0x6C => Some(KeyPressed(Printable(Keypad7))),

                0x70 => Some(KeyPressed(Printable(Keypad0))),
                0x71 => Some(KeyPressed(Printable(KeypadPoint))),
                0x72 => Some(KeyPressed(Printable(Keypad2))),
                0x73 => Some(KeyPressed(Printable(Keypad5))),
                0x74 => Some(KeyPressed(Printable(Keypad6))),
                0x75 => Some(KeyPressed(Printable(Keypad8))),
                0x76 => Some(KeyPressed(Control(Escape))),
                0x77 => Some(KeyPressed(Control(NumberLock))),
                0x78 => Some(KeyPressed(Control(F11))),
                0x79 => Some(KeyPressed(Printable(KeypadPlus))),
                0x7A => Some(KeyPressed(Printable(Keypad3))),
                0x7B => Some(KeyPressed(Printable(KeypadMinus))),
                0x7C => Some(KeyPressed(Printable(KeypadStar))),
                0x7D => Some(KeyPressed(Printable(Keypad9))),
                0x7E => Some(KeyPressed(Control(ScrollLock))),

                0x83 => Some(KeyPressed(Control(F7))),

                0xE0 | 0xE1 | 0xF0 => {self.modifier_bytes.push(code); None},

                _ => unreachable!(),
            },
            1 => match self.modifier_bytes[0].0 {
                0xE0 => match code.0 {
                    0x10 => Some(KeyPressed(Control(MultimediaSearch))),
                    0x11 => Some(KeyPressed(Control(RightAlt))),
                    0x14 => Some(KeyPressed(Control(RightCtrl))),
                    0x15 => Some(KeyPressed(Control(MultimediaPrevious))),
                    0x18 => Some(KeyPressed(Control(MultimediaFavourites))),
                    0x1F => Some(KeyPressed(Control(LeftGui))),

                    0x20 => Some(KeyPressed(Control(MultimediaRefresh))),
                    0x21 => Some(KeyPressed(Control(MultimediaVolumeDown))),
                    0x23 => Some(KeyPressed(Control(MultimediaMute))),
                    0x27 => Some(KeyPressed(Control(RightGui))),
                    0x28 => Some(KeyPressed(Control(MultimediaStop))),
                    0x2B => Some(KeyPressed(Control(MultimediaCalculator))),
                    0x2F => Some(KeyPressed(Control(Apps))),

                    0x30 => Some(KeyPressed(Control(MultimediaForward))),
                    0x32 => Some(KeyPressed(Control(MultimediaVolumeUp))),
                    0x34 => Some(KeyPressed(Control(MultimediaPlayPause))),
                    0x37 => Some(KeyPressed(Control(AcpiPower))),
                    0x38 => Some(KeyPressed(Control(MultimediaBack))),
                    0x3A => Some(KeyPressed(Control(MultimediaHome))),
                    0x3B => Some(KeyPressed(Control(MultimediaStop))),
                    0x3F => Some(KeyPressed(Control(AcpiSleep))),

                    0x40 => Some(KeyPressed(Control(MultimediaComputer))),
                    0x48 => Some(KeyPressed(Control(MultimediaEmail))),
                    0x4A => Some(KeyPressed(Printable(KeypadSlash))),
                    0x4D => Some(KeyPressed(Control(MultmediaNext))),

                    0x50 => Some(KeyPressed(Control(MultmediaMediaSelect))),
                    0x5A => Some(KeyPressed(Control(KeypadEnter))),
                    0x5E => Some(KeyPressed(Control(AcpiWake))),

                    0x69 => Some(KeyPressed(Control(End))),
                    0x6B => Some(KeyPressed(Control(CursorLeft))),
                    0x6C => Some(KeyPressed(Control(Home))),

                    0x70 => Some(KeyPressed(Control(Insert))),
                    0x71 => Some(KeyPressed(Control(Delete))),
                    0x72 => Some(KeyPressed(Control(CursorDown))),
                    0x74 => Some(KeyPressed(Control(CursorRight))),
                    0x75 => Some(KeyPressed(Control(CursorUp))),
                    0x7A => Some(KeyPressed(Control(PageDown))),
                    0x7D => Some(KeyPressed(Control(PageUp))),

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
                    0x01 => Some(KeyReleased(Control(F9))),
                    0x03 => Some(KeyReleased(Control(F5))),
                    0x04 => Some(KeyReleased(Control(F3))),
                    0x05 => Some(KeyReleased(Control(F1))),
                    0x06 => Some(KeyReleased(Control(F2))),
                    0x07 => Some(KeyReleased(Control(F12))),
                    0x09 => Some(KeyReleased(Control(F10))),
                    0x0A => Some(KeyReleased(Control(F8))),
                    0x0B => Some(KeyReleased(Control(F6))),
                    0x0C => Some(KeyReleased(Control(F4))),
                    0x0D => Some(KeyReleased(Control(Tab))),
                    0x0E => Some(KeyReleased(Printable(BackTick))),

                    0x11 => Some(KeyReleased(Control(LeftAlt))),
                    0x12 => Some(KeyReleased(Control(LeftShift))),
                    0x14 => Some(KeyReleased(Control(LeftCtrl))),
                    0x15 => Some(KeyReleased(Printable(Q))),
                    0x16 => Some(KeyReleased(Printable(Number1))),
                    0x1A => Some(KeyReleased(Printable(Z))),
                    0x1B => Some(KeyReleased(Printable(S))),
                    0x1C => Some(KeyReleased(Printable(A))),
                    0x1D => Some(KeyReleased(Printable(W))),
                    0x1E => Some(KeyReleased(Printable(Number2))),

                    0x21 => Some(KeyReleased(Printable(C))),
                    0x22 => Some(KeyReleased(Printable(X))),
                    0x23 => Some(KeyReleased(Printable(D))),
                    0x24 => Some(KeyReleased(Printable(E))),
                    0x25 => Some(KeyReleased(Printable(Number4))),
                    0x26 => Some(KeyReleased(Printable(Number3))),
                    0x29 => Some(KeyReleased(Printable(Space))),
                    0x2A => Some(KeyReleased(Printable(V))),
                    0x2B => Some(KeyReleased(Printable(F))),
                    0x2C => Some(KeyReleased(Printable(T))),
                    0x2D => Some(KeyReleased(Printable(R))),
                    0x2E => Some(KeyReleased(Printable(Number5))),

                    0x31 => Some(KeyReleased(Printable(N))),
                    0x32 => Some(KeyReleased(Printable(B))),
                    0x33 => Some(KeyReleased(Printable(H))),
                    0x34 => Some(KeyReleased(Printable(G))),
                    0x35 => Some(KeyReleased(Printable(Y))),
                    0x36 => Some(KeyReleased(Printable(Number6))),
                    0x3A => Some(KeyReleased(Printable(M))),
                    0x3B => Some(KeyReleased(Printable(J))),
                    0x3C => Some(KeyReleased(Printable(U))),
                    0x3D => Some(KeyReleased(Printable(Number7))),
                    0x3E => Some(KeyReleased(Printable(Number8))),

                    0x41 => Some(KeyReleased(Printable(Comma))),
                    0x42 => Some(KeyReleased(Printable(K))),
                    0x43 => Some(KeyReleased(Printable(I))),
                    0x44 => Some(KeyReleased(Printable(O))),
                    0x45 => Some(KeyReleased(Printable(Number0))),
                    0x46 => Some(KeyReleased(Printable(Number9))),
                    0x49 => Some(KeyReleased(Printable(Point))),
                    0x4A => Some(KeyReleased(Printable(Slash))),
                    0x4B => Some(KeyReleased(Printable(L))),
                    0x4C => Some(KeyReleased(Printable(Semicolon))),
                    0x4D => Some(KeyReleased(Printable(P))),
                    0x4E => Some(KeyReleased(Printable(Minus))),

                    0x52 => Some(KeyReleased(Printable(SingleQuote))),
                    0x54 => Some(KeyReleased(Printable(LeftBracket))),
                    0x55 => Some(KeyReleased(Printable(Equal))),
                    0x58 => Some(KeyReleased(Control(CapsLock))),
                    0x59 => Some(KeyReleased(Control(RightShift))),
                    0x5A => Some(KeyReleased(Control(Enter))),
                    0x5B => Some(KeyReleased(Printable(RightBracket))),
                    0x5D => Some(KeyReleased(Printable(Backslash))),

                    0x66 => Some(KeyReleased(Control(Backspace))),
                    0x69 => Some(KeyReleased(Printable(Keypad1))),
                    0x6B => Some(KeyReleased(Printable(Keypad4))),
                    0x6C => Some(KeyReleased(Printable(Keypad7))),

                    0x70 => Some(KeyReleased(Printable(Keypad0))),
                    0x71 => Some(KeyReleased(Printable(KeypadPoint))),
                    0x72 => Some(KeyReleased(Printable(Keypad2))),
                    0x73 => Some(KeyReleased(Printable(Keypad5))),
                    0x74 => Some(KeyReleased(Printable(Keypad6))),
                    0x75 => Some(KeyReleased(Printable(Keypad8))),
                    0x76 => Some(KeyReleased(Control(Escape))),
                    0x77 => Some(KeyReleased(Control(NumberLock))),
                    0x78 => Some(KeyReleased(Control(F11))),
                    0x79 => Some(KeyReleased(Printable(KeypadPlus))),
                    0x7A => Some(KeyReleased(Printable(Keypad3))),
                    0x7B => Some(KeyReleased(Printable(KeypadMinus))),
                    0x7C => Some(KeyReleased(Printable(KeypadStar))),
                    0x7D => Some(KeyReleased(Printable(Keypad9))),
                    0x7E => Some(KeyReleased(Control(ScrollLock))),

                    0x83 => Some(KeyReleased(Control(F7))),

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
                        0x10 => Some(KeyReleased(Control(MultimediaSearch))),
                        0x11 => Some(KeyReleased(Control(RightAlt))),
                        0x14 => Some(KeyReleased(Control(RightCtrl))),
                        0x15 => Some(KeyReleased(Control(MultimediaPrevious))),
                        0x18 => Some(KeyReleased(Control(MultimediaFavourites))),
                        0x1F => Some(KeyReleased(Control(LeftGui))),

                        0x20 => Some(KeyReleased(Control(MultimediaRefresh))),
                        0x21 => Some(KeyReleased(Control(MultimediaVolumeDown))),
                        0x23 => Some(KeyReleased(Control(MultimediaMute))),
                        0x27 => Some(KeyReleased(Control(RightGui))),
                        0x28 => Some(KeyReleased(Control(MultimediaStop))),
                        0x2B => Some(KeyReleased(Control(MultimediaCalculator))),
                        0x2F => Some(KeyReleased(Control(Apps))),

                        0x30 => Some(KeyReleased(Control(MultimediaForward))),
                        0x32 => Some(KeyReleased(Control(MultimediaVolumeUp))),
                        0x34 => Some(KeyReleased(Control(MultimediaPlayPause))),
                        0x37 => Some(KeyReleased(Control(AcpiPower))),
                        0x38 => Some(KeyReleased(Control(MultimediaBack))),
                        0x3A => Some(KeyReleased(Control(MultimediaHome))),
                        0x3B => Some(KeyReleased(Control(MultimediaStop))),
                        0x3F => Some(KeyReleased(Control(AcpiSleep))),

                        0x40 => Some(KeyReleased(Control(MultimediaComputer))),
                        0x48 => Some(KeyReleased(Control(MultimediaEmail))),
                        0x4A => Some(KeyReleased(Printable(KeypadSlash))),
                        0x4D => Some(KeyReleased(Control(MultmediaNext))),

                        0x50 => Some(KeyReleased(Control(MultmediaMediaSelect))),
                        0x5A => Some(KeyReleased(Control(KeypadEnter))),
                        0x5E => Some(KeyReleased(Control(AcpiWake))),

                        0x69 => Some(KeyReleased(Control(End))),
                        0x6B => Some(KeyReleased(Control(CursorLeft))),
                        0x6C => Some(KeyReleased(Control(Home))),

                        0x70 => Some(KeyReleased(Control(Insert))),
                        0x71 => Some(KeyReleased(Control(Delete))),
                        0x72 => Some(KeyReleased(Control(CursorDown))),
                        0x74 => Some(KeyReleased(Control(CursorRight))),
                        0x75 => Some(KeyReleased(Control(CursorUp))),
                        0x7A => Some(KeyReleased(Control(PageDown))),
                        0x7D => Some(KeyReleased(Control(PageUp))),

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
                        Some(KeyPressed(Control(PrintScreen)))
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
                    Some(KeyReleased(Control(PrintScreen)))
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
                Some(KeyPressed(Control(Pause)))
            },
            _ => unreachable!(),
        };
        if let Some(_) = key_press {
            self.modifier_bytes.clear()
        }
        key_press
    }
}
