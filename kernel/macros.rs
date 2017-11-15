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
