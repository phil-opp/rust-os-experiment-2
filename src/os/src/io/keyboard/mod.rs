use stream::{Stream, Subscriber};

pub struct KeyCode(pub usize);

pub fn init<S>(key_presses: S) where S: Stream<Item=KeyCode> {
    key_presses.subscribe(Box::new(Dummy));
}

pub struct Dummy;

impl Subscriber<KeyCode> for Dummy {
    fn on_value(&mut self, v: KeyCode) {
        let c = match v.0 {
            0x02 => '1',
            0x03 => '2',
            0x04 => '3',
            0x05 => '4',
            0x06 => '5',
            0x07 => '6',
            0x08 => '7',
            0x09 => '8',
            0x0A => '9',
            0x0B => '0',
            _ => '~',
        };
        print!("{}", c);
    }
    fn on_close(self: Box<Self>) {}
}
