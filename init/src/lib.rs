#![no_std]

#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;

#[no_mangle]
pub fn main() {
    boringos_std::process::exec("/bin/test").unwrap();

    loop {
        boringos_std::thread::sleep(500);
        println!("Test");
    }
}
