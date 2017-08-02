#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![default_lib_allocator]
#![feature(allocator_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
extern crate hole_list_allocator;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate once;
#[macro_use]
extern crate lazy_static;
extern crate bit_field;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("Hello {} world", "rust");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    enable_nxe_bit();
    enable_write_protect_bit();

    let mut memory_controller = memory::init(boot_info);
    println!("Heap and paging initialized");

    interrupts::init(&mut memory_controller);
    println!("Interrupts initialized");

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    // memory::test_paging(&mut frame_allocator);

    // println!("{:?}", frame_allocator.allocate_frame());
    // println!("{:?}", frame_allocator.allocate_frame());
    // println!("{:?}", frame_allocator.allocate_frame());


    loop {}

}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

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
