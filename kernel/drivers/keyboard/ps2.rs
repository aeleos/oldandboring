use drivers::keyboard::{Keyboard, KeyboardHandler, Keypair};
use spin::Mutex;
use alloc::boxed::Box;

struct PS2 {
    shift: Keypair,
    control: Keypair,
    alt: Keypair,
    caps_lock: bool,
    key_down: bool,
    scancode: u8,
}

impl Keyboard for PS2 {
    fn new() -> PS2 {
        PS2 {
            shift: Keypair::new(),
            control: Keypair::new(),
            alt: Keypair::new(),
            caps_lock: false,
            key_down: false,
            scancode: 0,
        }
    }

    fn shift(&self) -> Keypair {
        self.shift
    }

    fn caps_lock(&self) -> bool {
        self.caps_lock
    }

    fn key_down(&self) -> bool {
        self.key_down
    }

    fn scancode(&self) -> u8 {
        self.scancode
    }

    fn get_ascii(&self, should_apply: bool) -> Option<u8> {
        let idx = self.scancode as usize;
        if let Some(ascii) = match self.scancode {
            0x01...0x0E => Some(b"\x1B1234567890-=\0x02"[idx - 0x01]),
            0x0F...0x1C => Some(b"\tqwertyuiop[]\r"[idx - 0x0F]),
            0x1E...0x28 => Some(b"asdfghjkl;'"[idx - 0x1E]),
            0x2C...0x35 => Some(b"zxcvbnm,./"[idx - 0x2C]),
            0x39 => Some(b' '),
            _ => None,
        }
        {
            if should_apply {
                Some(self.apply(ascii))
            } else {
                Some(ascii)
            }
        } else {
            None
        }

    }

    /// Update modifiers given the current scancode
    fn update(&mut self, scancode: u8) {
        self.scancode = scancode;

        if scancode & 0x80 != 0 {
            self.key_down = true;
        } else {
            self.key_down = false;
        }

        match scancode {
            0x1D => self.control.left = true,
            0x2A => self.shift.left = true,
            0x36 => self.shift.right = true,
            0x38 => self.alt.left = true,
            0x3A => self.caps_lock = !self.caps_lock,
            0x9D => self.control.left = false,
            0xAA => self.shift.left = false,
            0xB6 => self.shift.right = false,
            0xB8 => self.alt.left = false,
            _ => {}
        }

    }
}


lazy_static! {
    static ref KB_HANDLER: Mutex<Box<KeyboardHandler<PS2>>> = Mutex::new(unsafe {
        KeyboardHandler::new()
    });

}

pub fn handle_irq() {
    let mut handler = &mut KB_HANDLER.lock();

    handler.update();

    if handler.keyboard.should_print_key() {
        if let Some(ascii) = handler.keyboard.get_ascii(true) {
            print!("{}", ascii as char);

        }
    }

}
