#![feature(start)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(unique)]
#![feature(from_ref)]
#![feature(ptr_internals)]
#![no_std]
#![allow(unused)]
extern crate spin;
extern crate volatile;

/// Makes a syscall with the given arguments.
macro_rules! syscall {
    ($num: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr, $arg2: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1),
            "{rsi}"($arg2)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1),
            "{rsi}"($arg2),
            "{rdx}"($arg3)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1),
            "{rsi}"($arg2),
            "{rdx}"($arg3),
            "{r10}"($arg4)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1),
            "{rsi}"($arg2),
            "{rdx}"($arg3),
            "{r10}"($arg4),
            "{r8}"($arg5)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr, $arg6: expr) => {{
        let result: u64;
        asm!("syscall" :
            "={rax}"(result) :
            "{rax}"($num),
            "{rdi}"($arg1),
            "{rsi}"($arg2),
            "{rdx}"($arg3),
            "{r10}"($arg4),
            "{r8}"($arg5),
            "{r9}"($arg6)
            : "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
            : "intel", "volatile");
        result
    }};
}

#[macro_use]
pub mod io;
pub mod process;
pub mod thread;
pub mod video;
pub mod screen;
pub mod math;
use process::exit;

extern "Rust" {
    /// The function that the program provides as a start.
    fn main();
}

/// The start of the application.
///
/// This should perform initialization and call main. After main returns, it should exit.
#[start]
#[no_mangle]
pub fn _start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        main();
    }
    exit();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
    unimplemented!();
}

/// The panic handler of the program.
///
/// This exits after printing some debug information.
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    panic_debugln!("PANIC! in file '{}' at line {}:", file, line);
    panic_debugln!("{}", fmt);
    exit();
}

//#[inline(always)]
//unsafe fn syscall(num: u64, arg1: u64,
//arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> i64 {
//let result;

//asm!("syscall"
//: "={rax}"(result) :
//"{rax}"(num),
//"{rdi}"(arg1),
//"{rsi}"(arg2),
//"{rdx}"(arg3),
//"{r10}"(arg4),
//"{r8}"(arg5),
//"{r9}"(arg6)
//: "rax", "rdi", "rsi", "rdx", "r10", "r8", "r9", "r12", "r11", "rcx"
//: "intel", "volatile");

//result
//}
