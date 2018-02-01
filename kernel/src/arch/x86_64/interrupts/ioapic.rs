//! Deals with configuring the I/O APIC.

use super::IRQ_INTERRUPT_NUMS;
use core::fmt;
use memory::{map_page_at, PageFlags, PhysicalAddress, VirtualAddress};
use x86_64::instructions::port::outb;

/// The physical base address of the memory mapped I/O APIC.
const IO_APIC_BASE: PhysicalAddress = 0xfec00000;

/// Initializes the I/O APIC.
pub fn init() {
    assert_has_not_been_called!("The I/O APIC should only be initialized once.");

    map_page_at(
        to_virtual!(IO_APIC_BASE),
        IO_APIC_BASE,
        PageFlags::READABLE | PageFlags::WRITABLE | PageFlags::NO_CACHE,
    );

    // Disable the 8259 PIC.
    unsafe {
        outb(0x21, 0xff);
        outb(0xa1, 0xff);
    }

    for i in 0..16 {
        let mut irq = IORedirectionEntry::new();
        irq.set_vector(IRQ_INTERRUPT_NUMS[i]);
        set_irq(i as u8, irq);
    }

    // Deactivate irq2.
    let mut irq2 = IORedirectionEntry::new();
    irq2.set_inactive();
    set_irq(2, irq2);

    // Reroute interrupts to the IOAPIC.
    unsafe {
        outb(0x22, 0x70);
        outb(0x23, 0x01);
    }
}

/// Writes an I/O APIC register.
fn set_register(reg: u8, value: u32) {
    unsafe {
        *(get_ioapic_base() as *mut u32) = reg as u32;
        *((get_ioapic_base() + 0x10) as *mut u32) = value;
    }
}

/// Sets the given IRQ number to the specified value.
fn set_irq(number: u8, value: IORedirectionEntry) {
    assert!(number < 24);

    let reg = 0x10 + number * 2;

    // Disable the entry, before setting the destination.
    set_register(reg, IORedirectionEntryFlags::MASK.bits() as u32);

    set_register(reg + 1, (value.0 >> 32) as u32);
    set_register(reg, value.0 as u32);
}

/// Returns the base address for the I/O APIC.
fn get_ioapic_base() -> VirtualAddress {
    to_virtual!(IO_APIC_BASE)
}

/// Represents an entry in the I/O APIC redirection table.
#[repr(C)]
struct IORedirectionEntry(u64);

bitflags! {
    struct IORedirectionEntryFlags: u64 {
        /// Corresponds to the interrupt vector in the IVT.
        const VECTOR = 0xff;
        /// The delivery mode of the interrupt.
        const DELIVERY_MODE = 0b111 << 8;
        /// Delivers the interrupt to the specified vector.
        const FIXED_DELIVERY_MODE = 0b000 << 8;
        /// Delivers the interrupt to the processor with the lowest priority.
        const LOWEST_PRIORITY_DELIVERY_MODE = 0b001 << 8;
        /// Delivers an SMI interrupt.
        const SMI_DELIVERY_MODE = 0b010 << 8;
        /// Delivers an NMI interrupt.
        const NMI_DELIVERY_MODE = 0b100 << 8;
        /// For external interrupts.
        const EXTINT_DELIVERY_MODE = 0b111 << 8;
        /// Delivers an INIT request.
        const INIT_DELIVERY_MODE = 0b101 << 8;
        /// Specifies how the destination field is to be interpreted.
        const DESTINATION_MODE = 1 << 11;
        /// The specified destination references a physical processor ID.
        const PHYSICAL_DESTINATION_MODE = 0 << 11;
        /// The specified destination references a logical processor ID.
        const LOGICAL_DESTINATION_MODE = 1 << 11;
        /// The delivery status of the interrupt.
        ///
        /// Read only.
        const DELIVERY_STATUS = 1 << 12;
        /// Specifies when the pin is active.
        const PIN_POLARITY = 1 << 13;
        /// The pin is active when high.
        const HIGH_ACTIVE_PIN_POLARITY = 0 << 13;
        /// The pin is active when low.
        const LOW_ACTIVE_PIN_POLARITY = 1 << 13;
        /// Indicates if the interrupt is being serviced.
        ///
        /// Read only.
        const REMOTRE_IRR = 1 << 14;
        /// Specifies the trigger mode for the interrupt.
        const TRIGGER_MODE = 1 << 15;
        /// For edge sensitive interrupts.
        const EDGE_SENSITIVE = 0 << 15;
        /// For level sensitive interrupts.
        const LEVEL_SENSITIVE = 1 << 15;
        /// Masks the interrupt.
        const MASK = 1 << 16;
        /// The destination processor for this interrupt.
        const DESTINATION = 0xff << 56;
    }
}

impl IORedirectionEntry {
    /// Creates a new LVT register.
    fn new() -> IORedirectionEntry {
        let mut register = IORedirectionEntry(0);
        register.set_active();
        register.set_delivery_mode(IORedirectionEntryFlags::FIXED_DELIVERY_MODE);
        register.set_trigger_mode(IORedirectionEntryFlags::EDGE_SENSITIVE);
        register.set_polarity(IORedirectionEntryFlags::HIGH_ACTIVE_PIN_POLARITY);
        // 0xff sends the interrupt to all processors.
        // TODO: Don't use this ID here.
        register.set_destination(
            IORedirectionEntryFlags::PHYSICAL_DESTINATION_MODE,
            ::multitasking::get_cpu_id() as u8,
        );

        register
    }

    /// Sets the vector of this interrupt.
    fn set_vector(&mut self, num: u8) {
        self.0 &= !IORedirectionEntryFlags::VECTOR.bits();
        self.0 |= num as u64;
    }

    /// Sets the delivery mode for this interrupt.
    fn set_delivery_mode(&mut self, mode: IORedirectionEntryFlags) {
        self.0 &= !IORedirectionEntryFlags::DELIVERY_MODE.bits();
        self.0 |= mode.bits();
    }

    /// Sets the trigger mode for this interrupt.
    fn set_trigger_mode(&mut self, mode: IORedirectionEntryFlags) {
        self.0 &= !IORedirectionEntryFlags::TRIGGER_MODE.bits();
        self.0 |= mode.bits();
    }

    /// Sets the polarity for this interrupt.
    fn set_polarity(&mut self, polarity: IORedirectionEntryFlags) {
        self.0 &= !IORedirectionEntryFlags::PIN_POLARITY.bits();
        self.0 |= polarity.bits();
    }

    /// Deactivates this interrupt.
    fn set_inactive(&mut self) {
        self.0 |= IORedirectionEntryFlags::MASK.bits();
    }

    /// Activates this interrupt.
    fn set_active(&mut self) {
        self.0 &= !IORedirectionEntryFlags::MASK.bits();
    }

    /// Sets the destination for this interrupt.
    fn set_destination(&mut self, mode: IORedirectionEntryFlags, dest: u8) {
        // Set the destination mode.
        self.0 &= !IORedirectionEntryFlags::DESTINATION_MODE.bits();
        self.0 |= mode.bits();

        // Set the actual destination.
        self.0 &= !IORedirectionEntryFlags::DESTINATION.bits();
        self.0 |= (dest as u64) << 56;
    }
}

impl fmt::Debug for IORedirectionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vector: {:x}, Active: {}",
            self.0 & IORedirectionEntryFlags::VECTOR.bits(),
            self.0 & IORedirectionEntryFlags::MASK.bits() == 0
        )
    }
}
