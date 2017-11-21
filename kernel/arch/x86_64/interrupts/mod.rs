mod gdt;
pub mod lapic;
mod ioapic;

pub use x86_64::structures::idt::{ExceptionStackFrame, HandlerFunc, Idt};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::instructions::interrupts;
use x86_64::VirtualAddress;
use x86_64::instructions::port::{inb, outb};
use super::sync::CLOCK;
use spin::{Mutex, Once};

use memory::MEMORY_CONTROLLER;

/// The vector for the scheduling interrupt.
pub const SCHEDULE_INTERRUPT_NUM: u8 = 0x20;

/// The vectors for the IRQs.
const IRQ_INTERRUPT_NUMS: [u8; 16] = [
    0xEC,
    0xE4,
    0xFF,
    0x94,
    0x8C,
    0x84,
    0x7C,
    0x74,
    0xD4,
    0xCC,
    0xC4,
    0xBC,
    0xB4,
    0xAC,
    0xA4,
    0x9C,
];

/// The vector for the LAPIC timer interrupt.
const TIMER_INTERRUPT_HANDLER_NUM: u8 = 0x30;

/// The handler number for the spurious interrupt.
const SPURIOUS_INTERRUPT_HANDLER_NUM: u8 = 0x2f;

/// The number of IRQ8 interrupt ticks that have passed since it was enabled.
static IRQ8_INTERRUPT_TICKS: Mutex<u64> = Mutex::new(0);


lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        macro_rules! register_handler {
            ($handler:path, $int_num:expr) => {{
                #[allow(unused_variables)]
                extern "x86-interrupt" fn irq_handler(stack_frame: &mut ExceptionStackFrame) {
                    let old_priority = lapic::get_priority();
                    lapic::set_priority(0x20);
                    unsafe {
                        interrupts::enable();
                    }

                    $handler();

                    unsafe {
                        interrupts::disable();
                    }
                    lapic::signal_eoi();
                    lapic::set_priority(old_priority);
                }
                idt[$int_num].set_handler_fn(irq_handler);
            }}
        }

        for i in 0..IRQ_INTERRUPT_NUMS.len() {
            idt[IRQ_INTERRUPT_NUMS[i] as usize].set_handler_fn(empty_handler);
        }

        register_handler!(::interrupts::keyboard_interrupt, IRQ_INTERRUPT_NUMS[1] as usize);
        register_handler!(irq8_handler, IRQ_INTERRUPT_NUMS[8] as usize);


        // LAPIC specific interrupts.
        idt[SPURIOUS_INTERRUPT_HANDLER_NUM as usize].set_handler_fn(empty_handler);
        register_handler!(timer_handler, TIMER_INTERRUPT_HANDLER_NUM as usize);
        idt
    };
}

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

pub fn init() {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::interrupts;

    let double_fault_stack = MEMORY_CONTROLLER
        .lock()
        .alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] =
            VirtualAddress(double_fault_stack.top());
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });

    gdt.load();

    unsafe {
        // reload code segment register
        set_cs(code_selector);

        // load TSS
        load_tss(tss_selector);
    }

    IDT.load();

    // Initialize the PIC and enable interrupts (STI)
    unsafe {
        lapic::init();

        ioapic::init();

        lapic::calibrate_timer();

        lapic::set_periodic_timer(150);

        interrupts::enable();
    }
}

fn timer_handler() {
    unsafe {
        CLOCK += 150;
    }
}

fn irq8_handler() {
    unsafe {
        // Hack to avoid locks
        if let Some(mut ticks) = IRQ8_INTERRUPT_TICKS.try_lock(){
            *ticks += 1;
        }

        // Read status register c of the RTC to signal the end of an interrupt.
        let nmi_bit = inb(0x70) & 0x80;
        outb(0x70, nmi_bit | 0x0c);
        inb(0x71);
    }
}

extern "x86-interrupt" fn irq_handler(stack_frame: &mut ExceptionStackFrame) {
    panic!(
        "IRQ with no handler was called, stack frame:\n{:#?}",
        stack_frame
    );
}

const DOUBLE_FAULT_IST_INDEX: usize = 0;

#[allow(unused_variables)]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    debugln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[allow(unused_variables)]
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    debugln!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

/// An interrupt handler that does nothing.
extern "x86-interrupt" fn empty_handler(_: &mut ExceptionStackFrame) {}
