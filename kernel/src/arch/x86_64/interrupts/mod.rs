//! Handles interrupts on the x86_64 architecture.

pub mod lapic;
mod ioapic;

pub use self::lapic::issue_self_interrupt;
use super::sync::CLOCK;
use multitasking::scheduler::schedule_next_thread;
use sync::PreemptableMutex;
use spin::Once;
use x86_64::instructions::interrupts;
use x86_64::registers::control_regs;
use x86_64::structures::idt::{ExceptionStackFrame, Idt, PageFaultErrorCode};
use x86_64::instructions::port::{inb, outb};
use x86_64::PrivilegeLevel;
use memory::VirtualAddress;
use multitasking::{get_process, ProcessID, TCB};
use multitasking::scheduler::READY_LIST;
/// The vector for the scheduling interrupt.
pub const SCHEDULE_INTERRUPT_NUM: u8 = 0x20;

/// The vectors for the IRQs.
const IRQ_INTERRUPT_NUMS: [u8; 16] = [
    0xEC, 0xE4, 0xFF, 0x94, 0x8C, 0x84, 0x7C, 0x74, 0xD4, 0xCC, 0xC4, 0xBC, 0xB4, 0xAC, 0xA4, 0x9C
];

/// The vector for the LAPIC timer interrupt.
const TIMER_INTERRUPT_HANDLER_NUM: u8 = 0x30;

/// The handler number for the spurious interrupt.
const SPURIOUS_INTERRUPT_HANDLER_NUM: u8 = 0x2f;

/// The number of IRQ8 interrupt ticks that have passed since it was enabled.
static IRQ8_INTERRUPT_TICKS: PreemptableMutex<u64> = PreemptableMutex::new(0);

static mut KEYBOARD_INTERRUPT: Once<PreemptableMutex<KbIntInfo>> = Once::new();

// lazy_static! {
//     /// The list of all the currently running processes.
//     static ref KEYBOARD_INTERRUPT: PreemptableMutex<KbIntInfo> = PreemptableMutex::new({
//         let mut map = BTreeMap::new();
//         map.insert(0, PCB::idle_pcb());
//
//         map
//     });
// }

lazy_static! {
    /// The interrupt descriptor table used by the kernel.
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        // Exception handlers.
        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        // idt.non_maskable_interrupt.set_handler_fn(empty_handler);
        // idt.overflow.set_handler_fn(empty_handler);
        // idt.bound_range_exceeded.set_handler_fn(empty_handler);
        // idt.invalid_opcode.set_handler_fn(empty_handler);
        // idt.device_not_available.set_handler_fn(empty_handler);
        // idt.stack_segment_fault.set_handler_fn(empty_handler_with_error);
        // idt.x87_floating_point.set_handler_fn(empty_handler);
        // idt.alignment_check.set_handler_fn(empty_handler_with_error);
        // idt.machine_check.set_handler_fn(empty_handler);
        // idt.simd_floating_point.set_handler_fn(empty_handler);
        // idt.virtualization.set_handler_fn(empty_handler);
        // idt.security_exception.set_handler_fn(empty_handler_with_error);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(0);
        }

        // IRQ interrupts that are not explicitly handled.
        for i in 0..16 {
            idt[IRQ_INTERRUPT_NUMS[i] as usize].set_handler_fn(empty_handler);
        }

        // IRQ interrupts that are explicitly handled.
        idt[IRQ_INTERRUPT_NUMS[1] as usize]
            .set_handler_fn(irq1_handler).set_privilege_level(PrivilegeLevel::Ring3);
        idt[IRQ_INTERRUPT_NUMS[8] as usize].set_handler_fn(irq8_handler);

        // The schedule interrupt is invoked for every reschedule.
        idt[SCHEDULE_INTERRUPT_NUM as usize].set_handler_fn(schedule_interrupt)
            .disable_interrupts(false);

        // LAPIC specific interrupts.
        idt[SPURIOUS_INTERRUPT_HANDLER_NUM as usize].set_handler_fn(empty_handler);
        idt[TIMER_INTERRUPT_HANDLER_NUM as usize].set_handler_fn(timer_handler);

        idt
    };
}

/// Initializes interrupts on the x86_64 architecture.
pub fn init() {
    assert_has_not_been_called!("Interrupts should only be initialized once.");

    IDT.load();

    lapic::init();

    ioapic::init();

    lapic::calibrate_timer();

    lapic::set_periodic_timer(150);
}

macro_rules! irq_interrupt {
    ($(#[$attr: meta])* fn $name: ident $content: tt) => {
        $(#[$attr])*
        extern "x86-interrupt" fn $name(_: &mut ExceptionStackFrame) {
            let old_priority = lapic::get_priority();
            lapic::set_priority(0x20);
            unsafe {
                interrupts::enable();
            }

            $content

            unsafe {
                interrupts::disable();
            }
            lapic::signal_eoi();
            lapic::set_priority(old_priority);
        }
    };
}

/// The divide by zero exception handler of the kernel.
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    panic_debugln!("Divide by zero exception.");
    panic_debugln!("{:?}", stack_frame);
    loop {}
}

/// The breakpoint exception handler of the kernel.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    panic_debugln!("Breakpoint exception.");
    panic_debugln!("{:?}", stack_frame);
    loop {}
}

/// The breakpoint exception handler of the kernel.
extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    panic_debugln!("GENERAL PROTECTION FAULT");
    panic_debugln!("{:?}", stack_frame);
    panic_debugln!("Error code: 0x{:x}", error_code);
    use multitasking::{CURRENT_THREAD, TCB};
    let tcb: &::sync::PreemptableMutex<TCB> = &CURRENT_THREAD;
    panic_debugln!("Running thread: {:?}", tcb);
    loop {}
}

/// The double fault handler of the kernel.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    panic_debugln!("DOUBLE FAULT!");
    panic_debugln!("{:?}", stack_frame);
    panic_debugln!("Error code: 0x{:x}", error_code);
    use multitasking::{CURRENT_THREAD, TCB};
    let tcb: &::sync::PreemptableMutex<TCB> = &CURRENT_THREAD;
    panic_debugln!("Running thread: {:?}", tcb);
    loop {}
}

/// The page fault handler of the kernel.
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    ::interrupts::page_fault_handler(control_regs::cr2().0, stack_frame, error_code);
}

/// The software interrupt handler that invokes schedule operations.
extern "x86-interrupt" fn schedule_interrupt(_: &mut ExceptionStackFrame) {
    lapic::set_priority(0x20);
    lapic::signal_eoi();
    unsafe {
        schedule_next_thread();
        interrupts::disable();
    }
    lapic::set_priority(0x0);
}

/// An interrupt handler that does nothing.
#[allow(dead_code)]
extern "x86-interrupt" fn empty_handler(_: &mut ExceptionStackFrame) {}

#[allow(dead_code)]
extern "x86-interrupt" fn empty_handler_with_error(_: &mut ExceptionStackFrame, _: u64) {}

irq_interrupt!(
/// The handler for the lapic timer interrupt.
fn timer_handler {
    unsafe {
        CLOCK += 150;
    }
    ::interrupts::timer_interrupt();
});

irq_interrupt!(
/// The handler for IRQ8.
fn irq8_handler {
    unsafe {
        *IRQ8_INTERRUPT_TICKS.lock() += 1;

        // Read status register c of the RTC to signal the end of an interrupt.
        let nmi_bit = inb(0x70) & 0x80;
        outb(0x70, nmi_bit | 0x0c);
        inb(0x71);
    }
});

irq_interrupt!(
/// The handler for IRQ1.
fn irq1_handler {
    // let scancode = unsafe { ::x86_64::instructions::port::inb(0x60) };

    if let Some(kb_int_info_lock) =unsafe { KEYBOARD_INTERRUPT.try() }{
        let mut kb_int_info = kb_int_info_lock.lock();

        let pid = kb_int_info.pid;
        let mut pcb = get_process(pid);
        let id = pcb.find_thread_id();

        match id {
            Some(id) => {
                let thread = TCB::in_process_with_arguments(
                    pid,
                    id,
                    kb_int_info.start_address,
                    &mut pcb,
                    kb_int_info.arg1,
                    0,
                    0,
                    0,
                    0,
                );

                pcb.add_thread(id);

                READY_LIST.lock().push(thread);

                // id as i64
            }
            None => (),
        }
    }
});

struct KbIntInfo {
    pid: ProcessID,
    start_address: usize,
    arg1: u64,
}

pub fn register_kb_interrupt(pid: ProcessID, start_address: VirtualAddress, arg1: u64) {
    unsafe {
        KEYBOARD_INTERRUPT.call_once(|| {
            PreemptableMutex::new(KbIntInfo {
                pid,
                start_address,
                arg1,
            })
        });
    }
}
