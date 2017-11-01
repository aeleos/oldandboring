#![feature(lang_items, const_fn, unique, asm, abi_x86_interrupt, concat_idents)]
#![feature(alloc, allocator_internals, const_unique_new, const_unsafe_cell_new)]
#![feature(const_cell_new)]
#![default_lib_allocator]
#![allow(dead_code)]
#![no_std]

#[allow(unused_extern_crates)]
extern crate rlibc;

extern crate alloc;
extern crate bit_field;
extern crate hole_list_allocator as allocator;
extern crate multiboot2;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate once;


use spin::Mutex;

#[macro_use]
mod drivers;

mod memory;
mod interrupts;
mod cpuio;
mod common;

static KEYBOARD: Mutex<cpuio::Port<u8>> = Mutex::new(unsafe { cpuio::Port::new(0x60) });

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    drivers::vga::text::clear_screen();
    drivers::serial::init();

    serialln!("Hello {} world", "rust");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };


    enable_nxe_bit();
    enable_write_protect_bit();

    // let information = cupid::master();
    // println!("{:#?}".information);
    // drivers::vesa::init();


    let mut memory_controller = memory::init(boot_info);
    serialln!("Heap and paging initialized");

    interrupts::init(&mut memory_controller);
    println!("Interrupts initialized");

    println!("Scanning PCI bus...");

    drivers::pci::init_pci();

    drivers::pci::print_devices();

    // println!("Boot Info: {:?}", boot_info);
    drivers::vga::video::init();

    serialln!("{:?}", boot_info.fb_info_tag().expect("no vbe info"));
    // serialln!("test");
    // serialln!("{:?}", boot_info.vbe_info_tag().expect("no vbe tag").mode());

    // for module in boot_info.module_tags() {
    //     serialln!("Module: {}", module.name());
    // }


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
    serialln!("\n\nPANIC in {} at line {}:", file, line);
    serialln!("  {}", fmt);
    loop {}
}
