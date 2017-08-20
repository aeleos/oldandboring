//! Interface to our PCI devices.
//!
//! As usual, this is heavily inspired by http://wiki.osdev.org/Pci

use core::fmt;
use core::iter::Iterator;
use spin::Mutex;
use cpuio;
use self::headers::PciHeader;
use alloc::boxed::Box;



pub mod headers;

struct Pci {
    address: cpuio::Port<u32>,
    data: cpuio::Port<u32>,
}

impl Pci {
    /// Read a 32-bit aligned word from PCI Configuration Address Space.
    /// This is marked as `unsafe` because passing in out-of-range
    /// parameters probably does excitingly horrible things to the
    /// hardware.
    unsafe fn read_config_register(&mut self, bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        // The bus number occupies bits 16 - 23
        // The slot number occupies bits 11 - 15
        // The function number occupies bits 8 - 10
        // The two least signifigant bits must be 0
        let address: u32 = 0x80000000 | (bus as u32) << 16 | (slot as u32) << 11 |
            (function as u32) << 8 | (offset & 0b1111_1100) as u32;
        self.address.write(address);
        self.data.read()
    }

    /// Check for a PCI device, and return information about it if present.
    unsafe fn probe(&mut self, bus: u8, slot: u8, function: u8) -> Option<PciDeviceFunction> {

        if !headers::is_valid(self.read_config_register(bus, slot, function, 0)) {
            return None;
        }

        let mut registers: [u32; 18] = [0;18];
        for (i, reg) in registers.iter_mut().enumerate(){
            *reg =  self.read_config_register(bus, slot, function, (i as u8) * 0x4);
        }

        Some(PciDeviceFunction {
            bus: bus,
            device_id: slot,
            function: function,
            header: headers::build_header(registers),
        })
    }

}


pub struct PciDeviceFunction {
    bus: u8,
    device_id: u8,
    function: u8,
    header: Box<PciHeader>,
}

impl fmt::Display for PciDeviceFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}: {}",
            self.bus,
            self.device_id,
            self.function,
            self.header.common(),
        )
    }
}

static PCI: Mutex<Pci> = Mutex::new(Pci {
    address: unsafe { cpuio::Port::new(0xCF8) },
    data: unsafe { cpuio::Port::new(0xCFC) },
});

const MAX_BUS: u8 = 255;
const MAX_DEVICE: u8 = 31;
const MAX_FUNCTION: u8 = 7;

/// Iterator over all functions on our PCI bus.
pub struct PciDeviceFunctionIterator {
    // Invariant: The fields in this struct point at the _next_ device to
    // probe our PCI bus for.
    done: bool,
    bus: u8,
    device: u8,
    multifunction: bool,
    function: u8,
}

impl PciDeviceFunctionIterator {
    fn update_state(&mut self) {
        if self.multifunction && self.function < MAX_FUNCTION {
            self.function += 1;
        } else if self.device < MAX_DEVICE {
            self.function = 0;
            self.multifunction = false;
            self.device += 1;
        } else if self.bus < MAX_BUS {
            self.function = 0;
            self.multifunction = false;
            self.device = 0;
            self.bus += 1;
        } else {
            self.done = true;
        }
    }
}


impl Iterator for PciDeviceFunctionIterator {
    type Item = PciDeviceFunction;

    fn next(&mut self) -> Option<Self::Item> {
        // Scan until we hit the next entry.
        let mut pci = PCI.lock();
        loop {
            // Give up if we've hit the end of the bus.
            if self.done {
                return None;
            }

            // Check for something at the current bus/device/function.
            if let Some(result) = unsafe { pci.probe(self.bus, self.device, self.function) } {
                // Something was found
                // Check to see if function 0 is multifunction
                if self.function == 0 && result.header.common().is_multifunction() {
                    // It is, start enumerating functions on the device
                    self.multifunction = true;
                }

                //Update the state
                self.update_state();

                // Return our result
                return Some(result)

            } else {

                // Nothing was found, update our state and continue
                self.update_state();
            }
        }
    }
}


/// Brute-force PCI bus probing.
pub fn init_pci() -> PciDeviceFunctionIterator {
    PciDeviceFunctionIterator {
        done: false,
        bus: 0,
        device: 0,
        multifunction: false,
        function: 0,
    }
}


// Running under QEMU, and checking against http://pcidatabase.com/ , we have:
//
// 0.0: 8086 1237 Intel 82440LX/EX PCI & Memory
// 0.1: 8086 7000 Intel 82371SB PIIX3 PCI-to-ISA Bridge (Triton II)
// 0.2: 1013 00b8 Cirrus Logic CL-GD5446 64-bit VisualMedia Accelerator
// 0.3: 8086 100e Intel 02000 Intel Pro 1000/MT
