//! This module handles system calls.

use arch::schedule;
use elf;
use memory::VirtualAddress;
use multitasking::{CURRENT_THREAD, get_current_process, TCB};
use multitasking::scheduler::READY_LIST;
use sync::time::{Timestamp, Time};

/// This function accepts the syscalls and calls the corresponding handlers.
pub fn syscall_handler(num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> i64 {
    match num {
        0 => print_char(arg1 as u8 as char),
        1 => kill_process(),
        2 => return_pid(),
        3 => exec(arg1 as VirtualAddress, arg2 as usize),
        4 => sleep(arg1),
        5 => create_thread(arg1 as VirtualAddress, arg2, arg3, arg4, arg5, arg6),
        6 => kill_thread(),
        _ => unknown_syscall(num),
    }
}

fn print_char(character: char) -> i64 {
    print!("{}", character);
    0
}

fn kill_process() -> i64 {
    get_current_process().kill();

    schedule();
    0
}

fn return_pid() -> i64 {
    let pid = CURRENT_THREAD.lock().pid;
    pid as i64
}

fn exec(name_ptr: VirtualAddress, name_length: usize) -> i64 {
    let name_ptr_valid = {
        let pcb = get_current_process();

        pcb.address_space.contains_range(name_ptr, name_length)
    };

    if name_ptr_valid {
        let name = from_raw_str!(name_ptr, name_length);

        if let Ok(name) = name {
            let process_id = elf::process_from_initramfs_file(name);

            if let Ok(process_id) = process_id {
                assert!(process_id as i64 > 0, "Process ID too large.");

                process_id as i64
            } else {
                -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

fn create_thread(start_address: VirtualAddress, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> i64 {
    let pid = CURRENT_THREAD.lock().pid;
    let mut pcb = get_current_process();
    let id = pcb.find_thread_id();

    match id {
        Some(id) => {
            let thread = TCB::in_process_with_arguments(pid, id, start_address, &mut pcb, arg1, arg2, arg3, arg4, arg5);

            pcb.add_thread(id);

            READY_LIST.lock().push(thread);

            id as i64
        }
        None => {
            -1
        }
    }
}

fn kill_thread() -> i64 {
    CURRENT_THREAD.lock().kill();

    schedule();

    0
}

fn sleep(ms: u64) -> i64 {
    let mut wake_time = Timestamp::get_current();
    
    // TODO: Handle the case of overflow.
    wake_time.offset(Time::Milliseconds(ms as i64));
    
    CURRENT_THREAD.lock().state = ::multitasking::ThreadState::Sleeping(wake_time);
    schedule();
    0
}

fn unknown_syscall(num: u64) -> ! {
    if cfg!(debug) {
        panic!("The syscall {} is not known.", num);
    } else {
        get_current_process().kill_immediately();
    }
}
