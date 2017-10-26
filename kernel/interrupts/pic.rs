use cpuio::{Port, UnsafePort};
use spin::Mutex;

/// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;

/// Command sent to acknowledge an interrupt.
const CMD_END_OF_INTERRUPT: u8 = 0x20;

// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

/// Unsafe function to lock the PIC Mutex and initialize the PIC
pub unsafe fn initialize() {
    PICS.lock().initialize();
}

/// Unsafe function to lock the PIC and notify that an interrupt is finished
/// being handled
pub unsafe fn notify_isr_eoi(int_no: u8) {
    PICS.lock().notify_isr_eoi(int_no);
}

/// An individual PIC chip.  This is not exported, because we always access
/// it through `Pics` below.
struct Pic {
    /// The base offset to which our interrupts are mapped.
    offset: u8,

    /// The processor I/O port on which we send commands.
    command: UnsafePort<u8>,

    /// The processor I/O port on which we send and receive data.
    data: UnsafePort<u8>,
}

impl Pic {
    /// Are we in change of handling the specified interrupt?
    /// (Each PIC handles 8 interrupts.)
    fn does_handle_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    /// Notify us that an interrupt has been handled and that we're ready
    /// for more.
    unsafe fn notify_isr_eoi(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

/// A pair of chained PIC controllers.  This is the standard setup on x86.
pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    /// Create a new interface for the standard PIC1 and PIC2 controllers,
    /// specifying the desired interrupt offsets.
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: UnsafePort::new(0x20),
                    data: UnsafePort::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: UnsafePort::new(0xA0),
                    data: UnsafePort::new(0xA1),
                },
            ],
        }
    }

    /// Initialize both our PICs.  We initialize them together, at the same
    /// time, because it's traditional to do so, and because I/O operations
    /// might not be instantaneous on older processors.
    pub unsafe fn initialize(&mut self) {
        // We need to add a delay between writes to our PICs, especially on
        // older motherboards.  But we don't necessarily have any kind of
        // timers yet, because most of them require interrupts.  Various
        // older versions of Linux and other PC operating systems have
        // worked around this by writing garbage data to port 0x80, which
        // allegedly takes long enough to make everything work on most
        // hardware.  Here, `wait` is a closure.
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || wait_port.write(0);

        // Save our original interrupt masks, because I'm too lazy to
        // figure out reasonable values.  We'll restore these when we're
        // done.
        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Tell each PIC that we're going to send it a three-byte
        // initialization sequence on its data port.
        self.pics[0].command.write(CMD_INIT);
        wait();
        self.pics[1].command.write(CMD_INIT);
        wait();

        // Byte 1: Set up our base offsets.
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        // Byte 2: Configure chaining between PIC1 and PIC2.
        self.pics[0].data.write(4);
        wait();
        self.pics[1].data.write(2);
        wait();

        // Byte 3: Set our mode.
        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
        wait();

        // Restore our saved masks.
        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }

    /// Do we handle this interrupt?
    pub fn does_handle_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics
            .iter()
            .any(|p| p.does_handle_interrupt(interrupt_id))
    }

    pub unsafe fn notify_isr_eoi(&mut self, interrupt_id: u8) {
        if self.does_handle_interrupt(interrupt_id) {
            if self.pics[1].does_handle_interrupt(interrupt_id) {
                self.pics[1].notify_isr_eoi();
            }
            self.pics[0].notify_isr_eoi();
        }
    }
}
