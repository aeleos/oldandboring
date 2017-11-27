#![feature(lang_items, const_fn, unique, asm, abi_x86_interrupt, concat_idents)]
#![feature(alloc, allocator_internals, const_unique_new, const_unsafe_cell_new)]
#![feature(const_cell_new, core_intrinsics, compiler_builtins_lib, global_allocator)]
#![feature(allocator_api, const_atomic_usize_new)]
#![default_lib_allocator]
#![allow(dead_code)]
#![no_std]

#[allow(unused_extern_crates)]
extern crate rlibc;

extern crate alloc;
extern crate bit_field;
extern crate compiler_builtins;
extern crate hole_list_allocator as allocator;
extern crate math;
extern crate multiboot2;
extern crate raw_cpuid;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate once;


use spin::{Mutex, Once};

#[macro_use]
pub mod macros;

mod drivers;

mod memory;
mod arch;
mod cpuio;
mod common;
mod sync;
mod interrupts;

static KEYBOARD: Mutex<cpuio::Port<u8>> = Mutex::new(unsafe { cpuio::Port::new(0x60) });

static BOOT_INFO: Once<&multiboot2::BootInformation> = Once::new();


#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::vga_buffer::clear_screen();
    drivers::serial::init();

    debugln!("Hello {} world", "rust");

    BOOT_INFO.call_once(|| unsafe {
        multiboot2::load(multiboot_information_address)
    });

    debugln!("Multiboot information initialized");


    enable_nxe_bit();
    enable_write_protect_bit();

    lazy_static::initialize(&memory::MEMORY_CONTROLLER);
    debugln!("Heap and paging initialized");

    arch::init();
    debugln!("Interrupts initialized");

    debugln!("Scanning PCI bus...");
    drivers::pci::init_pci();

    drivers::pci::print_devices();

    // println!("Boot Info: {:?}", BOOT_INFO.try().unwrap());

    drivers::vga::video::init();

    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{rdmsr, wrmsr, IA32_EFER};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{Cr0, cr0, cr0_write};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    debugln!("\n\nPANIC in {} at line {}:", file, line);
    debugln!("  {}", fmt);
    loop {}
}

#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
