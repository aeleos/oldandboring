//! This module contains general interrupt handlers.
//!
//! None of the contained interrupt handlers should be architecture specific.
//! They should instead
//! be called by the architecture specific interrupt handlers.

use drivers::keyboard::KB_HANDLER;

// /// The timer interrupt handler for the system.
// pub fn timer_interrupt() {
//     {
//         let mut sleeping_list = SLEEPING_LIST.lock();
//         loop {
//             if sleeping_list.peek().is_some() {
//                 if sleeping_list.peek().unwrap().get_sleep_time() <= Timestamp::get_current() {
//                     READY_LIST.lock().push(sleeping_list.pop().unwrap().0);
//                 } else {
//                     break;
//                 }
//             } else {
//                 break;
//             }
//         }
//     }
//
//     schedule();
// }

/// The keyboard interrupt handler.
pub fn keyboard_interrupt() {
    KB_HANDLER.lock().handle_irq();
}

// / The page fault handler.
// pub fn page_fault_handler(address: VirtualAddress, program_counter: VirtualAddress) {
//     unsafe { ::sync::disable_preemption() };
//     let current_thread = CURRENT_THREAD.lock();
//
//     println!(
//         "Page fault in process {} (thread {}) at address {:x} (PC: {:x})",
//         current_thread.pid,
//         current_thread.id,
//         address,
//         program_counter
//     );
//
//     println!("Page flags: {:?}", ::memory::get_page_flags(address));
//     loop {}
// }
