//! Abstracts architecture details.
//!
//! The job of this module is to have submodules for each architecture and to
//! provide interfaces to them.

macro_rules! export_arch {
    ($name: ident) => {
        pub use self::$name::early_init;
        pub use self::$name::init;
        pub use self::$name::get_cpu_id;
        pub use self::$name::get_cpu_num;
        pub use self::$name::schedule;
        pub use self::$name::enter_first_thread;
        pub use self::$name::Context;
        pub use self::$name::STACK_TYPE;

        pub use self::$name::sync::cpu_relax;
        pub use self::$name::sync::cpu_halt;
        pub use self::$name::sync::interrupts_enabled;
        pub use self::$name::sync::disable_interrupts;
        pub use self::$name::sync::enable_interrupts;
        pub use self::$name::sync::get_current_timestamp;

        pub use self::$name::memory::init as memory_init;
        pub use self::$name::memory::map_page;
        pub use self::$name::memory::map_page_at;
        pub use self::$name::memory::unmap_page;
        pub use self::$name::memory::get_kernel_start_address;
        pub use self::$name::memory::get_kernel_end_address;
        pub use self::$name::memory::get_initramfs_start;
        pub use self::$name::memory::get_initramfs_length;
        pub use self::$name::memory::get_page_flags;
        pub use self::$name::memory::PAGE_SIZE;
        pub use self::$name::memory::HEAP_MAX_SIZE;
        pub use self::$name::memory::HEAP_START;
        pub use self::$name::memory::KERNEL_STACK_AREA_BASE;
        pub use self::$name::memory::KERNEL_STACK_OFFSET;
        pub use self::$name::memory::KERNEL_STACK_MAX_SIZE;
        pub use self::$name::memory::USER_STACK_AREA_BASE;
        pub use self::$name::memory::USER_STACK_OFFSET;
        pub use self::$name::memory::USER_STACK_MAX_SIZE;
        pub use self::$name::memory::get_free_memory_size;
        pub use self::$name::memory::is_userspace_address;

        pub use self::$name::memory::new_address_space_manager;
        pub use self::$name::memory::idle_address_space_manager;

        pub use self::$name::context::switch_context;
    };
}

#[cfg(target_arch = "x86_64")]
export_arch!(x86_64);

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::vga_buffer;

use core::fmt;
#[cfg(target_arch = "x86_64")]
mod x86_64;

/// Writes the formatted arguments.
///
/// This takes arguments as dictated by `core::fmt` and prints the to the
/// screen using the printing method relevant for the current architecture.
pub fn write_fmt(args: fmt::Arguments) {
    if cfg!(target_arch = "x86_64") {
        use core::fmt::Write;
        x86_64::vga_buffer::WRITER
            .lock()
            .write_fmt(args)
            .unwrap();
    }
}

/// Sets the state of being interruptable to the given state.
///
/// # Safety
/// - Don't use this function directly, rather use the sync module.
pub unsafe fn set_interrupt_state(state: bool) {
    if state {
        enable_interrupts();
    } else {
        disable_interrupts();
    }
}
