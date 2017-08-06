//! CPU-level input/output instructions, including `inb`, `outb`, etc., and
//! a high level Rust wrapper.
use core::marker::{PhantomData, Sized};

use x86_64::instructions::port;


/// This trait is defined for any type which can be read or written over a
/// port.  The processor supports I/O with `u8`, `u16` and `u32`.  The
/// functions in this trait are all unsafe because they can write to
/// arbitrary ports.
pub trait InOut
where
    Self: Sized,
{
    /// Because we are using references to arrays, we need to specify that
    /// this trait can only be used with types of a fixed size

    /// Read a value from the specified port.
    unsafe fn port_in(port: u16) -> Self;

    /// Write a value to the specified port.
    unsafe fn port_out(port: u16, value: Self);

    /// Read an array of values from the specified port.
    unsafe fn port_arr_in(port: u16, value: &mut [Self]);

    /// Write an array of values to the specified port.
    unsafe fn port_arr_out(port: u16, value: &[Self]);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        port::inb(port)
    }

    unsafe fn port_out(port: u16, value: u8) {
        port::outb(port, value);
    }

    unsafe fn port_arr_in(port: u16, value: &mut [u8]) {
        port::insb(port, value)
    }

    unsafe fn port_arr_out(port: u16, value: &[u8]) {
        port::outsb(port, value)
    }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 {
        port::inw(port)
    }

    unsafe fn port_out(port: u16, value: u16) {
        port::outw(port, value);
    }

    unsafe fn port_arr_in(port: u16, value: &mut [u16]) {
        port::insw(port, value)
    }

    unsafe fn port_arr_out(port: u16, value: &[u16]) {
        port::outsw(port, value)
    }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 {
        port::inl(port)
    }

    unsafe fn port_out(port: u16, value: u32) {
        port::outl(port, value);
    }

    unsafe fn port_arr_in(port: u16, value: &mut [u32]) {
        port::insl(port, value)
    }

    unsafe fn port_arr_out(port: u16, value: &[u32]) {
        port::outsl(port, value)
    }
}

// impl InOut for &[u32] {
//     unsafe fn port_in(port: u16) -> u32 {
//         port::inl(port)
//     }
//
//     unsafe fn port_out(port: u16, value: u32) {
//         port::outl(port, value);
//     }
// }

/// An I/O port over an arbitrary type supporting the `InOut` interface.
///
/// This version of `Port` has safe `read` and `write` functions, and it's
/// appropriate for communicating with hardware that can't violate Rust's
/// safety guarantees.
#[derive(Debug)]
pub struct Port<T: InOut> {
    // Port address.
    port: u16,

    // Zero-byte placeholder.  This is only here so that we can have a
    // type parameter `T` without a compiler error.
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Create a new I/O port.
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            phantom: PhantomData,
        }
    }

    /// Read data from the port.  This is nominally safe, because you
    /// shouldn't be able to get hold of a port object unless somebody
    /// thinks it's safe to give you one.
    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    /// Write data to the port.
    pub fn write(&mut self, value: T) {
        unsafe {
            T::port_out(self.port, value);
        }
    }

    pub fn read_arr(&mut self, value: &mut [T]) {
        unsafe {
            T::port_arr_in(self.port, value);
        }
    }

    pub fn write_arr(&mut self, value: &[T]) {
        unsafe {
            T::port_arr_out(self.port, value);
        }
    }
}

/// An unsafe I/O port over an arbitrary type supporting the `InOut`
/// interface.
///
/// This version of `Port` has unsafe `read` and `write` functions, and
/// it's appropriate for speaking to hardware that can potentially corrupt
/// memory or cause undefined behavior.
#[derive(Debug)]
pub struct UnsafePort<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> UnsafePort<T> {
    /// Create a new I/O port.
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort {
            port: port,
            phantom: PhantomData,
        }
    }

    /// Read data from the port.
    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    /// Write data to the port.
    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value);
    }

    pub fn read_arr(&mut self, value: &mut [T]) {
        unsafe {
            T::port_arr_in(self.port, value);
        }
    }

    pub fn write_arr(&mut self, value: &[T]) {
        unsafe {
            T::port_arr_out(self.port, value);
        }
    }
}
