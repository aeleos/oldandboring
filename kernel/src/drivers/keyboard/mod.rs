use cpuio::UnsafePort;
use alloc::boxed::Box;
use spin::PreemptableMutex;

pub mod ps2;

/// A pair of keys which appears on both the left and right side
/// of the keyboard, ex. left shift, right shift
#[derive(Debug, Clone, Copy)]
pub struct Keypair {
    left: bool,
    right: bool,
}

impl Keypair {
    /// Create a new default keypair
    const fn new() -> Keypair {
        Keypair {
            left: false,
            right: false,
        }
    }

    /// Is either of the keys in this pair currently pressed
    fn is_pressed(&self) -> bool {
        self.left || self.right
    }

    fn left(&self) -> bool {
        self.left
    }

    fn right(&self) -> bool {
        self.right
    }
}

pub trait Keyboard {
    fn new() -> Self;

    fn shift(&self) -> Keypair;

    fn caps_lock(&self) -> bool;

    fn key_down(&self) -> bool;

    fn scancode(&self) -> u8;

    /// Update modifiers given the current scancode
    fn update(&mut self, scancode: u8);

    fn get_ascii(&self, should_apply: bool) -> Option<u8>;

    fn should_print_key(&self) -> bool {
        !self.key_down()
    }

    /// Should uppercase letters given the current modifier state
    fn should_use_uppercase(&self) -> bool {
        self.shift().is_pressed() ^ self.caps_lock()
    }

    /// Apply all modifiers to a given ascii character
    fn apply(&self, ascii: u8) -> u8 {
        if b'a' <= ascii && ascii <= b'z' {
            if self.should_use_uppercase() {
                return ascii - b'a' + b'A';
            }
        }
        ascii
    }
}

pub struct KeyboardHandler<K: Keyboard> {
    port: UnsafePort<u8>,
    keyboard: K,
}


impl<K: Keyboard> KeyboardHandler<K> {
    unsafe fn new() -> Box<KeyboardHandler<K>> {
        Box::new(KeyboardHandler {
            port: UnsafePort::new(0x60),
            keyboard: K::new(),
        })
    }

    pub fn read(&mut self) -> u8 {
        unsafe { self.port.read() }
    }

    pub fn update(&mut self) {
        let scancode = self.read();
        self.keyboard.update(scancode);
    }

    pub fn handle_irq(&mut self) {
        self.update();
        if self.keyboard.should_print_key() {
            if let Some(ascii) = self.keyboard.get_ascii(true) {
                if ascii as char == '\r' {
                    debugln!("");
                    return;
                }
                print!("{}", ascii as char);
            }
        }
    }
}


lazy_static! {
    pub static ref KB_HANDLER: PreemptableMutex<Box<KeyboardHandler<ps2::PS2>>> = PreemptableMutex::new(unsafe {
        KeyboardHandler::new()
    });
}
