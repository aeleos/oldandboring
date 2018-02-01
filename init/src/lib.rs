#![feature(from_ref)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![no_std]
#![feature(lang_items)]
#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;

mod video;

#[no_mangle]
pub fn main() {
    boringos_std::process::exec("/bin/test").unwrap();

    boringos_std::screen::init();
    // boringos_std::screen::test();
    // video::mandelbrot::draw();
    video::voxelspace::test();
    // let mut buffer = SCREEN.try().unwrap().lock();
    // buffer.write(100, 100, &[255, 0, 255, 0]);
    // video::voxelspace::test();
    // let mut i = 100;

    loop {
        boringos_std::thread::sleep(500);
        debugln!("Test");
    }
}
