//! Manages multitasking in the operating system.

mod tcb;
pub mod stack;
pub mod scheduler;
mod cpu_local;
mod pcb;

pub use self::cpu_local::{CPULocal, CPULocalMut};
pub use self::pcb::{get_current_process, PCB};
pub use self::scheduler::CURRENT_THREAD;
pub use self::stack::{Stack, StackType};
pub use self::tcb::{ThreadState, TCB};
use alloc::btree_map::BTreeMap;
pub use arch::{get_cpu_id, get_cpu_num};
use memory::VirtualAddress;
use memory::address_space::AddressSpace;
use sync::PreemptableMutex;
use sync::preemptable_mutex::PreemptableMutexGuard;

/// The type of a process ID.
pub type ProcessID = usize;

/// The type of a thread ID.
type ThreadID = u16;

lazy_static! {
    /// The list of all the currently running processes.
    static ref PROCESS_LIST: PreemptableMutex<BTreeMap<ProcessID, PCB>> = PreemptableMutex::new({
        let mut map = BTreeMap::new();
        map.insert(0, PCB::idle_pcb());

        map
    });
}

/// Finds an unused process ID.
fn find_pid(list: &PreemptableMutexGuard<BTreeMap<ProcessID, PCB>>) -> ProcessID {
    let mut pid = 1;
    while list.contains_key(&pid) {
        pid += 1;
    }
    pid
}

/// Creates a new process.
pub fn create_process(address_space: AddressSpace, entry_address: VirtualAddress) -> ProcessID {
    let mut pcb = PCB::new(address_space);

    let mut process_list = PROCESS_LIST.lock();
    let id = find_pid(&process_list);

    let first_tcb = TCB::in_process(id, 0, entry_address, &mut pcb);

    scheduler::READY_LIST.lock().push(first_tcb);

    assert!(
        process_list.insert(id, pcb).is_none(),
        "Trying to use an already used PID ({}).",
        id
    );

    id
}
