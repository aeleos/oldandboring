use cpuio::UnsafePort;
use core::fmt;
use sync::PreemptableMutex;

pub enum Port {
    COM1,
    COM2,
}

struct Serial {
    data: UnsafePort<u8>,
    interrupt: UnsafePort<u8>,
    fifo_control: UnsafePort<u8>,
    line_control: UnsafePort<u8>,
    modem_control: UnsafePort<u8>,
    line_status: UnsafePort<u8>,
}

#[allow(dead_code)]
impl Serial {
    pub fn new(address: u16) -> Serial {
        unsafe {
            Serial {
                data: UnsafePort::new(address),
                interrupt: UnsafePort::new(address + 1),
                fifo_control: UnsafePort::new(address + 2),
                line_control: UnsafePort::new(address + 3),
                modem_control: UnsafePort::new(address + 4),
                line_status: UnsafePort::new(address + 5),
            }
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.interrupt.write(0x00); // Disable all interrupts
            self.line_control.write(0x80); // Enable DLAB (set baud rate divisor)
            self.data.write(0x03); // Set divisor to 3 (lo byte) 38400 baud
            self.interrupt.write(0x00); //              (hi byte)
            self.line_control.write(0x03); //8 bits, no parity, one stop bit
            self.fifo_control.write(0xC7); // Enable FIFO, clear them, with 14-byte threshold
            self.modem_control.write(0x0B); // IRQs enabled, RTS/DSR set
        }
    }

    pub fn read_waiting(&mut self) -> u8 {
        unsafe { self.line_status.read() & 1 }
    }

    pub fn read(&mut self) -> u8 {
        unsafe {
            while self.read_waiting() == 0 {}
            self.data.read()
        }
    }

    pub fn transmit_empty(&mut self) -> u8 {
        unsafe { (self.line_status.read() & 0x20) }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while self.transmit_empty() == 0 {}
            self.data.write(byte);
        }
    }
}

impl fmt::Write for Serial {
    #[allow(dead_code)]

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

lazy_static! {
    static ref COM1: PreemptableMutex<Serial> = PreemptableMutex::new(Serial::new(0x3F8));
    static ref COM2: PreemptableMutex<Serial> = PreemptableMutex::new(Serial::new(0x2F8));
}

pub fn print(port: Port, args: fmt::Arguments) {
    use core::fmt::Write;
    match port {
        Port::COM1 => COM1.lock().write_fmt(args).unwrap(),
        Port::COM2 => COM2.lock().write_fmt(args).unwrap(),
    }
}

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();
}
