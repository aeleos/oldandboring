use spin::Mutex;
use cpuio::UnsafePort;

struct KeyboardHandler {
    port: UnsafePort<u8>,
    pressed: bool,
    shift_down: bool,
}

static KB_HANDLER: Mutex<KeyboardHandler> = Mutex::new(unsafe { KeyboardHandler::new() });


impl KeyboardHandler {
    const unsafe fn new() -> KeyboardHandler {
        KeyboardHandler {
            port: UnsafePort::new(0x60),
            pressed: false,
            shift_down: false,
        }
    }

    pub fn read(&mut self) -> u8 {
        unsafe { self.port.read() }
    }
}

pub fn handle_irq() {
    let mut handler = &mut KB_HANDLER.lock();

    let scancode = handler.read();

    if scancode & 0x80 != 0 {
        handler.pressed = false;
    } else {
        if !handler.pressed {
            let c = KEYMAP_US_LOWER[(scancode & (!0x80)) as usize];
            print!("{}", c);
            handler.pressed = true;
        }
    }
}

const KEYMAP_US_LOWER: [char; 90] = [
    '\0' as char,
    27 as char,
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8', /* 9 */
    '9',
    '0',
    '-',
    '=',
    '\x08', /* Backspace */
    '\t', /* Tab */
    'q',
    'w',
    'e',
    'r', /* 19 */
    't',
    'y',
    'u',
    'i',
    'o',
    'p',
    '[',
    ']',
    '\n', /* Enter key */
    '\0', /* 29   - Control */
    'a',
    's',
    'd',
    'f',
    'g',
    'h',
    'j',
    'k',
    'l',
    ';', /* 39 */
    '\'',
    '`',
    '\0', /* Left shift */
    '\\',
    'z',
    'x',
    'c',
    'v',
    'b',
    'n', /* 49 */
    'm',
    ',',
    '.',
    '/',
    '\0', /* Right shift */
    '*',
    '\0', /* Alt */
    ' ', /* Space bar */
    '\0', /* Caps lock */
    '\0', /* 59 - F1 key ... > */
    '\0',
    '\0',
    '\0',
    '\0',
    '\0',
    '\0',
    '\0',
    '\0',
    '\0', /* < ... F10 */
    '\0', /* 69 - Num lock*/
    '\0', /* Scroll Lock */
    '\0', /* Home key */
    '\0', /* Up Arrow */
    '\0', /* Page Up */
    '-',
    '\0', /* Left Arrow */
    '\0',
    '\0', /* Right Arrow */
    '+',
    '\0', /* 79 - End key*/
    '\0', /* Down Arrow */
    '\0', /* Page Down */
    '\0', /* Insert Key */
    '\0', /* Delete Key */
    '\0',
    '\0',
    '\0',
    '\0', /* F11 Key */
    '\0', /* F12 Key */
    '\0' /* All other keys are undefined */,
];
