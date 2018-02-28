//! This module contains general interrupt handlers.
//!
//! None of the contained interrupt handlers should be architecture specific.
//! They should instead
//! be called by the architecture specific interrupt handlers.

use arch::schedule;
use memory::VirtualAddress;
use multitasking::CURRENT_THREAD;
use multitasking::scheduler::{READY_LIST, SLEEPING_LIST};
use sync::time::Timestamp;
use x86_64::structures::idt::PageFaultErrorCode;

/// The timer interrupt handler for the system.
pub fn timer_interrupt() {
    {
        let mut sleeping_list = SLEEPING_LIST.lock();
        loop {
            if sleeping_list.peek().is_some() {
                if sleeping_list.peek().unwrap().get_sleep_time() <= Timestamp::get_current() {
                    READY_LIST.lock().push(sleeping_list.pop().unwrap().0);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    schedule();
}

/// The page fault handler.
pub fn page_fault_handler(
    address: VirtualAddress,
    stack_frame: &mut ::x86_64::structures::idt::ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    unsafe { ::sync::disable_preemption() };
    let current_thread = CURRENT_THREAD.lock();

    debugln!(
        "Page fault in process {} (thread {}) at address {:x} (PC: {:x})",
        current_thread.pid,
        current_thread.id,
        address,
        stack_frame.instruction_pointer.0
    );

    debugln!("stack_frame: {:?}", stack_frame);

    debugln!("error_code: {:?}", error_code);

    debugln!("Page flags: {:?}", ::memory::get_page_flags(address));
    loop {}
}
