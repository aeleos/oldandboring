//! This module defines IO functions.

use core::fmt;
use core::fmt::Write;

/// The number of the print char syscall.
const PRINT_CHAR_SYSCALL: u64 = 0;
const SERIAL_CHAR_SYSCALL: u64 = 7;
const PANIC_SERIAL_CHAR_SYSCALL: u64 = 8;

/// A dummy struct to implement fmt::Write on.
struct StdOut;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            print_char(character);
        }
        Ok(())
    }
}

struct SerialOut;

impl fmt::Write for SerialOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            serial_char(character);
        }
        Ok(())
    }
}

struct PanicSerialOut;

impl fmt::Write for PanicSerialOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for character in s.chars() {
            panic_serial_char(character);
        }
        Ok(())
    }
}

/// Prints a line to the standard output.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::print(format_args!($($arg)*));
    });
}

/// Prints a line to the standard output.
#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => (debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints to the standard output.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::io::serial(format_args!($($arg)*));
    });
}

/// Prints a line to the standard output.
#[macro_export]
macro_rules! panic_debugln {
    ($fmt:expr) => (panic_debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (panic_debug!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints to the standard output.
#[macro_export]
macro_rules! panic_debug {
    ($($arg:tt)*) => ({
        $crate::io::panic_serial(format_args!($($arg)*));
    });
}

/// Prints the given format arguments.
pub fn print(args: fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

/// Prints a character to the screen.
fn print_char(character: char) {
    unsafe {
        syscall!(PRINT_CHAR_SYSCALL, character as u64);
    }
}

pub fn serial(args: fmt::Arguments) {
    SerialOut.write_fmt(args).unwrap();
}

fn serial_char(character: char) {
    unsafe {
        syscall!(SERIAL_CHAR_SYSCALL, character as u64);
    }
}

pub fn panic_serial(args: fmt::Arguments) {
    PanicSerialOut.write_fmt(args).unwrap();
}

fn panic_serial_char(character: char) {
    unsafe {
        syscall!(PANIC_SERIAL_CHAR_SYSCALL, character as u64);
    }
}
