use cpuio::UnsafePort;
use spin::Mutex;
static PIT: Mutex<Pit> = Mutex::new(unsafe { Pit::new() });
// Rate is 1000hz, so we want a trigger every 1000 ticks
static SEC_TIMER: Mutex<PitHandler> = Mutex::new(PitHandler::new(1000, second_trigger));

#[allow(unused_variables)]
pub fn second_trigger(val: u32) {
    // println!("We are {} seconds in", val);
    // nothing here yet
}

struct Pit {
    data: UnsafePort<u8>,
    command: UnsafePort<u8>,
    divisor: u32,
}

impl Pit {
    const unsafe fn new() -> Pit {
        Pit {
            data: UnsafePort::new(0x40),
            command: UnsafePort::new(0x43),
            divisor: 0,
        }
    }

    pub fn divisor(self) -> u32 {
        self.divisor
    }

    fn set_timer_phase(&mut self, hz: u32) {
        let divisor: u32 = 1193181 / hz;
        self.divisor = divisor;

        // 0x36 == 0011 0110
        // Bits
        // Select Channel
        // 6 & 7 == Channel 0
        // Access Mode
        // 4 & 5 == Access mode: lobyte / hybyte
        // Operating Mode
        // 1...3 == Mode 3 (square wave generator)
        // BCD / Binary mode
        // 0     == 16 bit binary
        unsafe {
            self.command.write(0x36);
            self.data.write((divisor & 0xFF) as u8);
            self.data.write((divisor >> 8) as u8);
        }
    }
}

pub struct PitHandler {
    ticks_killed: u32,
    ticks_triggered: u32,
    ticks_per_trigger: u32,
    trigger_function: fn(u32),
}

impl PitHandler {
    const fn new(ticks_per_trigger: u32, trigger_function: fn(u32)) -> PitHandler {
        PitHandler {
            ticks_killed: 0,
            ticks_triggered: 0,
            ticks_per_trigger: ticks_per_trigger,
            trigger_function: trigger_function,
        }
    }

    #[allow(unused_variables)]
    pub fn placeholder_trigger(val: u32) {}

    pub fn get_killed(&self) -> u32 {
        self.ticks_killed
    }

    pub fn update(&mut self) {
        if self.ticks_killed == self.ticks_per_trigger {
            self.ticks_triggered += 1;
            (self.trigger_function)(self.ticks_triggered);
            self.ticks_killed = 0;
        } else {
            self.ticks_killed += 1;
        }
    }
}

pub fn initialize() {
    // interrupts::register_irq_handler(0, irq_handler);
    PIT.lock().set_timer_phase(1000);
}

pub fn irq_handler() {
    SEC_TIMER.lock().update();
}
