pub use self::scancode::ScanCode;

use stream::{Stream, Subscriber};

mod scancode;

pub fn init<S>(key_presses: S) where S: Stream<Item=ScanCode> {
    let mut parser = scancode::Parser::new();
    key_presses.filter_map(move |code| parser.parse_code(code)).subscribe(Dummy);
}

struct Dummy;

impl Subscriber<KeyPress> for Dummy {
    fn on_value(&mut self, v: KeyPress) {
        use self::KeyPress::*;

        if let KeyPressed(press) = v {
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

    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

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

    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,

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
