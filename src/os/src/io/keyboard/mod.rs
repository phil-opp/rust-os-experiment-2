pub use self::scancode::ScanCode;

use stream::{Stream, Subscriber};

mod scancode;

pub fn init<S>(key_presses: S) where S: Stream<Item=ScanCode> {
    key_presses.subscribe(Box::new(Dummy));
}

struct Dummy;

impl Subscriber<Option<KeyPress>> for Dummy {
    fn on_value(&mut self, v: Option<KeyPress>) {
        use self::KeyPress::*;

        if let Some(KeyPressed(press)) = v {
            print!("{:?} ", press);
        }
    }
}


#[derive(Debug)]
enum KeyPress {
    KeyPressed(Key),
    KeyReleased(Key),
}

#[derive(Debug)]
enum Key {
    Escape,
    Backspace,
    Tab,
    Enter,
    Space,

    LeftCtrl,
    LeftAlt,
    LeftShift,
    LeftGui,
    RightCtrl,
    RightAlt,
    RightShift,
    RightGui,

    Comma,
    Point,
    Semicolon,
    Slash,
    Backslash,
    LeftBracket,
    RightBracket,
    Equal,
    SingleQuote,
    BackTick,
    Minus,

    Home,
    End,
    Delete,
    Insert,
    PageUp,
    PageDown,

    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,

    NumberLock,
    CapsLock,
    ScrollLock,
    PrintScreen,
    Pause,

    Number_0,
    Number_1,
    Number_2,
    Number_3,
    Number_4,
    Number_5,
    Number_6,
    Number_7,
    Number_8,
    Number_9,

    Char_a,
    Char_b,
    Char_c,
    Char_d,
    Char_e,
    Char_f,
    Char_g,
    Char_h,
    Char_i,
    Char_j,
    Char_k,
    Char_l,
    Char_m,
    Char_n,
    Char_o,
    Char_p,
    Char_q,
    Char_r,
    Char_s,
    Char_t,
    Char_u,
    Char_v,
    Char_w,
    Char_x,
    Char_y,
    Char_z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Keypad_0,
    Keypad_1,
    Keypad_2,
    Keypad_3,
    Keypad_4,
    Keypad_5,
    Keypad_6,
    Keypad_7,
    Keypad_8,
    Keypad_9,

    KeypadPlus,
    KeypadMinus,
    KeypadStar,
    KeypadSlash,
    KeypadPoint,
    KeypadEnter,

    AcpiPower,
    AcpiSleep,
    AcpiWake,

    MultimediaStop,
    MultimediaPlayPause,
    MultmediaNext,
    MultimediaPrevious,
    MultimediaHome,
    MultimediaEmail,
    MultimediaSearch,
    MultimediaRefresh,
    MultimediaForward,
    MultimediaBack,
    MultmediaMediaSelect,
    MultimediaComputer,
    MultimediaVolumeUp,
    MultimediaVolumeDown,
    MultimediaMute,
    MultimediaCalculator,
    MultimediaFavourites,
    Apps,
}
