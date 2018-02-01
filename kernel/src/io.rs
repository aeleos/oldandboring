//! This module deals with all in-kernel IO.
//!
//! It handles all the IO that kernel code needs to perform.
use drivers::serial;

/// Initializes all IO devices.
pub fn init() {
    assert_has_not_been_called!("IO components should only be initialized once");
    serial::init();
    if cfg!(target_arch = "x86_64") {
        ::arch::vga_buffer::init();
    }
}

/// Prints the given line to the screen.
///
/// It uses the arguments passed to it and prints the string with the
/// formatting arguments.
/// Then a new line is started.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints the given string to the screen.
///
/// It uses the arguments passed to it and prints the string with the
/// formatting arguments.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::arch::write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::drivers::serial::print($crate::drivers::serial::Port::COM1, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => (debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! panic_debug {
    ($($arg:tt)*) => ({
        $crate::drivers::serial::print($crate::drivers::serial::Port::COM2, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! panic_debugln {
    ($fmt:expr) => (panic_debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (panic_debug!(concat!($fmt, "\n"), $($arg)*));
}
