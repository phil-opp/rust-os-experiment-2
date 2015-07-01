use super::KeyPress::{self, KeyPressed, KeyReleased};
use super::Key::{Control, Printable};
use super::Input::{self, Char, ControlKeyPressed, ControlKeyReleased};
use super::PrintableKey::*;
use super::ControlKey::*;

pub struct Parser {
    left_shift: bool,
    right_shift: bool,
    caps_lock: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            left_shift: false,
            right_shift: false,
            caps_lock: false,
        }
    }

    pub fn parse(&mut self, key_press: KeyPress) -> Option<Input> {
        match key_press {
            KeyPressed(Control(key)) => {
                match key {
                    LeftShift => {self.left_shift = true},
                    RightShift => {self.right_shift = true},
                    CapsLock => {self.caps_lock = !self.caps_lock},
                    _ => {},
                };
                Some(ControlKeyPressed(key))
            },
            KeyReleased(Control(key)) => {
                match key {
                    LeftShift => {self.left_shift = false},
                    RightShift => {self.right_shift = false},
                    _ => {},
                };
                Some(ControlKeyReleased(key))
            },
            KeyReleased(Printable(_)) => None,

            KeyPressed(Printable(key)) => match key {
                Space => Some(Char(' ')),

                Keypad1 => Some(Char('1')),
                Keypad2 => Some(Char('2')),
                Keypad3 => Some(Char('3')),
                Keypad4 => Some(Char('4')),
                Keypad5 => Some(Char('5')),
                Keypad6 => Some(Char('6')),
                Keypad7 => Some(Char('7')),
                Keypad8 => Some(Char('8')),
                Keypad9 => Some(Char('9')),
                Keypad0 => Some(Char('0')),

                KeypadPlus => Some(Char('+')),
                KeypadMinus => Some(Char('-')),
                KeypadStar => Some(Char('*')),
                KeypadSlash => Some(Char('/')),
                KeypadPoint => Some(Char('.')),

                _ => match self.left_shift | self.right_shift | self.caps_lock {
                    false => match key {
                        A => Some(Char('a')),
                        B => Some(Char('b')),
                        C => Some(Char('c')),
                        D => Some(Char('d')),
                        E => Some(Char('e')),
                        F => Some(Char('f')),
                        G => Some(Char('g')),
                        H => Some(Char('h')),
                        I => Some(Char('i')),
                        J => Some(Char('j')),
                        K => Some(Char('k')),
                        L => Some(Char('l')),
                        M => Some(Char('m')),
                        N => Some(Char('n')),
                        O => Some(Char('o')),
                        P => Some(Char('p')),
                        Q => Some(Char('q')),
                        R => Some(Char('r')),
                        S => Some(Char('s')),
                        T => Some(Char('t')),
                        U => Some(Char('u')),
                        V => Some(Char('v')),
                        W => Some(Char('w')),
                        X => Some(Char('x')),
                        Y => Some(Char('z')),
                        Z => Some(Char('y')),

                        Number1 => Some(Char('1')),
                        Number2 => Some(Char('2')),
                        Number3 => Some(Char('3')),
                        Number4 => Some(Char('4')),
                        Number5 => Some(Char('5')),
                        Number6 => Some(Char('6')),
                        Number7 => Some(Char('7')),
                        Number8 => Some(Char('8')),
                        Number9 => Some(Char('9')),
                        Number0 => Some(Char('0')),

                        BackTick => Some(Char('^')),
                        Minus => Some(Char('ß')),
                        Equal => Some(Char('´')),
                        LeftBracket => Some(Char('ü')),
                        RightBracket => Some(Char('+')),
                        Backslash => Some(Char('#')),
                        Semicolon => Some(Char('ö')),
                        SingleQuote => Some(Char('ä')),
                        Comma => Some(Char(',')),
                        Point => Some(Char('.')),
                        Slash => Some(Char('-')),

                        _ => None,
                    },
                    true => match key {
                        A => Some(Char('A')),
                        B => Some(Char('B')),
                        C => Some(Char('C')),
                        D => Some(Char('D')),
                        E => Some(Char('E')),
                        F => Some(Char('F')),
                        G => Some(Char('G')),
                        H => Some(Char('H')),
                        I => Some(Char('I')),
                        J => Some(Char('J')),
                        K => Some(Char('K')),
                        L => Some(Char('L')),
                        M => Some(Char('M')),
                        N => Some(Char('N')),
                        O => Some(Char('O')),
                        P => Some(Char('P')),
                        Q => Some(Char('Q')),
                        R => Some(Char('R')),
                        S => Some(Char('S')),
                        T => Some(Char('T')),
                        U => Some(Char('U')),
                        V => Some(Char('V')),
                        W => Some(Char('W')),
                        X => Some(Char('X')),
                        Y => Some(Char('Z')),
                        Z => Some(Char('Y')),

                        Number1 => Some(Char('!')),
                        Number2 => Some(Char('"')),
                        Number3 => Some(Char('§')),
                        Number4 => Some(Char('$')),
                        Number5 => Some(Char('%')),
                        Number6 => Some(Char('&')),
                        Number7 => Some(Char('/')),
                        Number8 => Some(Char('(')),
                        Number9 => Some(Char(')')),
                        Number0 => Some(Char('=')),

                        BackTick => Some(Char('°')),
                        Minus => Some(Char('?')),
                        Equal => Some(Char('`')),
                        LeftBracket => Some(Char('Ü')),
                        RightBracket => Some(Char('*')),
                        Backslash => Some(Char('\'')),
                        Semicolon => Some(Char('Ö')),
                        SingleQuote => Some(Char('Ä')),
                        Comma => Some(Char(';')),
                        Point => Some(Char(':')),
                        Slash => Some(Char('_')),

                        _ => None,
                    },
                },
            },
        }
    }
}
