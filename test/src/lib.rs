#![no_std]
#![feature(asm)]
#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;

fn keyboard_test(arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64) {
    let scancode: u8;

    unsafe {
        asm!("inb %dx, %al" : "={ax}"(scancode) : "{dx}"(0x60) :: "volatile");
    }
    debugln!("Key: <{}>", scancode);
    // let scancode = unsafe { ::x86_64::instructions::port::inb(0x60) };
}

#[no_mangle]
pub fn main() {
    // boringos_std::thread::new_thread(new_thread, 1, 2, 3, 4);
    // debugln!("test pid: {}", boringos_std::process::get_pid());
    boringos_std::thread::register_kb_interrupt(keyboard_test);

    loop {
        let a: f32 = 432.432432;
        let b: f32 = 6.654654;
        let c = a * b;

        debugln!("start");

        boringos_std::thread::sleep(1000);
        debugln!("Nest: a: {}, b: {}, c: {}", a, b, c);
    }
}
