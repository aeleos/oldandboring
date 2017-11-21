mod interrupts;
pub mod vga_buffer;
pub mod sync;

use raw_cpuid::CpuId;

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
