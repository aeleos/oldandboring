#![feature(const_fn, asm, core_intrinsics)]
#![no_std]
#![allow(dead_code)]

mod io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
