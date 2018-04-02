//! The x86_64 architecture.
//!
//! This module does all the architecture specific things for x86_64.

pub mod vga_buffer;
pub mod memory;
pub mod sync;
pub mod interrupts;
pub mod context;
pub mod syscalls;
pub mod gdt;
pub mod device;
// pub mod video;

pub use self::context::Context;
use self::gdt::{GDT, TSS};
use self::interrupts::SCHEDULE_INTERRUPT_NUM;
use self::interrupts::issue_self_interrupt;
use multitasking::{StackType, CURRENT_THREAD};
use raw_cpuid::CpuId;
use x86_64::VirtualAddress;
use x86_64::instructions::{rdmsr, wrmsr};
use x86_64::registers::*;
use core::fmt;

/// The stack type used for the x86_64 architecture.
pub const STACK_TYPE: StackType = StackType::FullDescending;

/// Initializes the machine state for the x86_64 architecture to a bare minimum.
pub fn early_init() {
    assert_has_not_been_called!("Early x86_64 specific initialization should only be called once.");

    let cpuid = CpuId::new();
    let mut supported = true;

    if let Some(features) = cpuid.get_feature_info() {
        supported &= features.has_apic();
    } else {
        supported = false;
    }

    if let Some(function_info) = cpuid.get_extended_function_info() {
        supported &= function_info.has_syscall_sysret();
        supported &= function_info.has_execute_disable();
    } else {
        supported = false;
    }

    if !supported {
        panic!("Your hardware unfortunately does not supported BoringOS.");
    }

    unsafe {
        // Enable syscall/sysret instructions and the NXE bit in the page table.
        wrmsr(msr::IA32_EFER, rdmsr(msr::IA32_EFER) | 1 << 11 | 1);

        // Enable global pages.
        let cr4_flags = control_regs::cr4() | control_regs::Cr4::ENABLE_GLOBAL_PAGES;
        control_regs::cr4_write(cr4_flags);

        // Enable read only pages.
        let cr0_flags = control_regs::cr0() | control_regs::Cr0::WRITE_PROTECT;
        control_regs::cr0_write(cr0_flags);
    }
}

/// Initializes the machine state for the x86_64 architecture to the final
/// state.
pub fn init() {
    assert_has_not_been_called!("x86_64 specific initialization code should only be called once.");

    unsafe {
        GDT.load();
    }

    syscalls::init();
    interrupts::init();
}

/// Returns the ID of the currently running CPU.
pub fn get_cpu_id() -> usize {
    CpuId::new()
        .get_feature_info()
        .unwrap()
        .initial_local_apic_id() as usize
}

/// Returns the number of addressable CPUs.
pub fn get_cpu_num() -> usize {
    CpuId::new()
        .get_feature_info()
        .unwrap()
        .max_logical_processor_ids() as usize
}

/// This is called once per processor to enter the first user mode thread.
///
/// # Safety
/// - This should only be called once.
pub unsafe fn enter_first_thread() -> ! {
    // video::test();
    let stack_pointer = CURRENT_THREAD
        .without_locking()
        .context
        .kernel_stack_pointer;
    TSS.as_mut().privilege_stack_table[0] = VirtualAddress(stack_pointer);
    asm!("mov rsp, $0
          ret"
          : : "r"(stack_pointer) : : "intel", "volatile");
    unreachable!();
}

/// This function starts a scheduling operation.
pub fn schedule() {
    issue_self_interrupt(SCHEDULE_INTERRUPT_NUM);
}


/// Writes the formatted arguments.
///
/// This takes arguments as dictated by `core::fmt` and prints the to the
/// screen using the printing method relevant for the current architecture.
pub fn write_fmt(args: fmt::Arguments) {
    if cfg!(target_arch = "x86_64") {
        use core::fmt::Write;
        vga_buffer::WRITER.lock().write_fmt(args).unwrap();
    }
}

/// Sets the state of being interruptable to the given state.
///
/// # Safety
/// - Don't use this function directly, rather use the sync module.
pub unsafe fn set_interrupt_state(state: bool) {
    if state {
        sync::enable_interrupts();
    } else {
        sync::disable_interrupts();
    }
}
