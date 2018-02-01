//! This module defines thread control blocks (TCBs).

use super::{PCB, PROCESS_LIST, ProcessID, Stack, ThreadID};
use super::stack::AccessType;
use arch::Context;
use core::cmp::Ordering;
use core::fmt;
use memory::{KERNEL_STACK_AREA_BASE, KERNEL_STACK_MAX_SIZE, KERNEL_STACK_OFFSET, USER_STACK_AREA_BASE, USER_STACK_MAX_SIZE, USER_STACK_OFFSET, VirtualAddress};
use sync::time::Timestamp;
use x86_64::registers::control_regs::cr3;

/// Represents the possible states a thread can have.
#[derive(Debug, PartialEq)]
pub enum ThreadState {
    /// The thread is currently running.
    Running,
    /// The thread is ready to run.
    Ready,
    /// The thread is sleeping for a specified amount of time.
    Sleeping(Timestamp),
    /// The thread is dead.
    Dead
}

/// A structure representing a thread control block (TCB).
pub struct TCB {
    /// The thread ID within the process.
    pub id: ThreadID,
    /// The ID of the process that the thread belongs to.
    pub pid: ProcessID,
    /// The stack used during kernel operations.
    pub kernel_stack: Stack,
    /// The usermode stack.
    pub user_stack: Stack,
    /// The state of the thread.
    pub state: ThreadState,
    /// The priority of the thread.
    pub priority: i32,
    /// The architecture specific context of this thread.
    pub context: Context
}

impl fmt::Debug for TCB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Thread <ID: {}, PID: {}> ({:?})", self.id, self.pid, self.state)
    }
}

impl PartialEq for TCB {
    fn eq(&self, other: &TCB) -> bool {
        // This assumes that thread IDs are unique.
        self.id == other.id
    }
}

impl Eq for TCB {}

impl Ord for TCB {
    fn cmp(&self, other: &TCB) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for TCB {
    fn partial_cmp(&self, other: &TCB) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Drop for TCB {
    fn drop(&mut self) {
        let mut process_list = PROCESS_LIST.lock();

        let drop_pcb = {
            let pcb = process_list
                .get_mut(&self.pid)
                .expect("Process of the thread doesn't exist.");

            pcb.thread_count -= 1;

            self.kernel_stack.resize(0, Some(&mut pcb.address_space));
            self.user_stack.resize(0, Some(&mut pcb.address_space));

            pcb.is_droppable()
        };

        if drop_pcb {
            process_list.remove(&self.pid);
        }
    }
}

impl TCB {
    /// Creates a new thread in the given process at the given start address.
    pub fn in_process(pid: ProcessID, id: ThreadID, pc: VirtualAddress, pcb: &mut PCB) -> TCB {
        TCB::in_process_with_arguments(pid, id, pc, pcb, 0, 0, 0, 0, 0)
    }

    /// Creates a new thread in the given process at the given start address with the given arguments.
    pub fn in_process_with_arguments(pid: ProcessID, id: ThreadID, pc: VirtualAddress, pcb: &mut PCB, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> TCB {
        let kernel_stack = Stack::new(0x4000,
                                      KERNEL_STACK_MAX_SIZE,
                                      KERNEL_STACK_AREA_BASE + KERNEL_STACK_OFFSET * (id as usize),
                                      AccessType::KernelOnly,
                                      Some(&mut pcb.address_space));

        let user_stack = Stack::new(0x2000,
                                    USER_STACK_MAX_SIZE,
                                    USER_STACK_AREA_BASE + USER_STACK_OFFSET * (id as usize),
                                    AccessType::UserAccessible,
                                    Some(&mut pcb.address_space));

        let stack_pointer = user_stack.base_stack_pointer;
        let kernel_stack_pointer = kernel_stack.base_stack_pointer;

        TCB {
            id,
            pid,
            kernel_stack,
            user_stack,
            state: ThreadState::Ready,
            priority: 1,
            context: Context::new(pc,
                                  stack_pointer,
                                  kernel_stack_pointer,
                                  &mut pcb.address_space,
                                  arg1,
                                  arg2,
                                  arg3,
                                  arg4,
                                  arg5)
        }
    }

    /// Creates a new TCB for an idle thread.
    pub fn idle_tcb(cpu_id: usize) -> TCB {
        let id = cpu_id as ThreadID;


        // NOTE: This assumes that the idle address space is currently active.
        let kernel_stack = Stack::new(0x3000,
                                    KERNEL_STACK_MAX_SIZE,
                                    KERNEL_STACK_AREA_BASE + KERNEL_STACK_OFFSET * (id as usize),
                                    AccessType::KernelOnly,
                                    None);

        let stack_pointer = kernel_stack.base_stack_pointer;

        TCB {
            id,
            pid: 0,
            kernel_stack,
            user_stack: Stack::new(0, 0, 0, AccessType::KernelOnly, None),
            state: ThreadState::Ready,
            priority: i32::min_value(),
            context: Context::idle_context(stack_pointer, cr3().0 as usize)
        }
    }

    /// Returns true if the thread state is dead.
    pub fn is_dead(&self) -> bool {
        let process_list = PROCESS_LIST.lock();
        let process = process_list.get(&self.pid).expect("Process of the thread doesn't exist.");
        
        self.state == ThreadState::Dead || process.is_dead()
    }

    /// Returns true if the thread state is running.
    pub fn is_running(&self) -> bool {
        self.state == ThreadState::Running
    }

    /// Sets the thread state to ready if applicable.
    pub fn set_ready(&mut self) {
        if !self.is_dead() {
            self.state = ThreadState::Ready;
        }
    }

    /// Sets the thread state to running.
    pub fn set_running(&mut self) {
        debug_assert!(!self.is_dead(), "Trying to run a dead thread: {:?}", self);

        self.state = ThreadState::Running;
    }

    /// Marks this thread as dead.
    ///
    /// This will cause the scheduler to not schedule it anymore and drop it.
    pub fn kill(&mut self) {
        self.state = ThreadState::Dead;
    }
}

/// A TCB that is sorted by its sleep time (shortest first).
pub struct SleepTimeSortedTCB(pub TCB);

impl SleepTimeSortedTCB {
    /// Returns the sleep time for this TCB.
    pub fn get_sleep_time(&self) -> Timestamp {
        match self.0.state {
            ThreadState::Sleeping(time) => time,
            _ => unreachable!()
        }
    }
}

impl PartialEq for SleepTimeSortedTCB {
    fn eq(&self, other: &SleepTimeSortedTCB) -> bool {
        self.get_sleep_time() == other.get_sleep_time()
    }
}

impl Eq for SleepTimeSortedTCB {}

impl Ord for SleepTimeSortedTCB {
    fn cmp(&self, other: &SleepTimeSortedTCB) -> Ordering {
        other.get_sleep_time().cmp(&self.get_sleep_time())
    }
}

impl PartialOrd for SleepTimeSortedTCB {
    fn partial_cmp(&self, other: &SleepTimeSortedTCB) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
