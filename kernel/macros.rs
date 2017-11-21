#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::arch::write_fmt(format_args!($($arg)*));
    });
}
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::drivers::serial::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => (debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug!(concat!($fmt, "\n"), $($arg)*));
}

/// Converts to a virtual address.
///
/// Converts a given physical address within the kernel part of memory to its
/// corresponding
/// virtual address.
#[macro_export]
#[cfg(target_arch = "x86_64")]
macro_rules! to_virtual {
    ($address: expr) => {{
        const KERNEL_OFFSET: usize = 0xffff800000000000;
        $address as usize + KERNEL_OFFSET
    }};
}

/// Returns true for a valid virtual address.
#[macro_export]
macro_rules! valid_address {
    ($address: expr) => {{
        if cfg!(arch = "x86_64") {
            use arch::x86_64::memory::{VIRTUAL_LOW_MAX_ADDRESS, VIRTUAL_HIGH_MIN_ADDRESS};
            (VIRTUAL_LOW_MAX_ADDRESS >= $address || $address >= VIRTUAL_HIGH_MIN_ADDRESS)
        } else {
            true
        }
    }};
}
