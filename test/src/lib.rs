#![no_std]

#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;

#[no_mangle]
pub fn main() {
    loop {
        let a: f32 = 432.432432;
        let b: f32 = 6.654654;
        let c = a * b;

        boringos_std::thread::sleep(1000);
        println!("Nest: a: {}, b: {}, c: {}", a, b, c);
    }
}
