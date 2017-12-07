pub mod interrupts;
pub mod vga_buffer;
pub mod sync;

use raw_cpuid::CpuId;
use x86_64::instructions::{rdmsr, wrmsr};
use x86_64::registers::*;

pub fn early_init() {
    assert_has_not_been_called!("Early x86_64 specific initialization should only be called once.");
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

pub fn init() {
    assert_has_not_been_called!("x86_64 specific initialization code should only be called once.");
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
