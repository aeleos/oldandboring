
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::drivers::vga::text::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}


macro_rules! serial {
    ($($arg:tt)*) => ({
        $crate::drivers::serial::print(format_args!($($arg)*));
    });
}

macro_rules! serialln {
    ($fmt:expr) => (serial!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial!(concat!($fmt, "\n"), $($arg)*));
}
