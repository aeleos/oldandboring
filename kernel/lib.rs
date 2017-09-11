#![feature(lang_items, const_fn, unique, asm, alloc, allocator_internals, abi_x86_interrupt,
          concat_idents)]
#![default_lib_allocator]
#![no_std]
#![allow(dead_code)]
#![feature(trace_macros)]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;
extern crate hole_list_allocator as allocator;
#[macro_use]
extern crate alloc;
extern crate bit_field;
extern crate fringe;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate once;
#[macro_use]
extern crate lazy_static;


use spin::Mutex;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;
mod cpuio;
mod drivers;

static KEYBOARD: Mutex<cpuio::Port<u8>> = Mutex::new(unsafe { cpuio::Port::new(0x60) });

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("Hello {} world", "rust");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };



    enable_nxe_bit();
    enable_write_protect_bit();

    // let information = cupid::master();
    // println!("{:#?}".information);
    // drivers::vesa::init();


    let mut memory_controller = memory::init(boot_info);
    println!("Heap and paging initialized");

    interrupts::init(&mut memory_controller);
    println!("Interrupts initialized");

    println!("Scanning PCI bus...");
    // for function in drivers::pci::init_pci() {Trait
    //     println!("{}", function);
    // }

    drivers::pci::init_pci();

    // drivers::pci::print_devices();

    use alloc::boxed::Box;
    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test2 = Box::new("hello");
    println!("{:?} {:?}", heap_test, heap_test2);

    let mut vec_test = vec![1, 2, 3, 4, 5, 6, 7];
    vec_test[3] = 42;
    for i in &vec_test {
        print!("{} ", i);
    }



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
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("  {}", fmt);
    loop {}
}
