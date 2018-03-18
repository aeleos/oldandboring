#![feature(from_ref)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![feature(lang_items)]
#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;
extern crate spin;

use spin::{Mutex, Once};
mod video;

// use video::voxelspace::Camera;
use boringos_std::math::*;

// static CAMERA: Once<Mutex<Camera>> = Once::new();
//
// fn keyboard_test(arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64) {
//     let mut changed: bool = false;
//     let scancode: u8;
//     unsafe {
//         asm!("inb %dx, %al" : "={ax}"(scancode) : "{dx}"(0x60) :: "volatile");
//     }
//
//     if let Some(camera_lock) = CAMERA.try() {
//         let mut camera = camera_lock.lock();
//
//         if scancode == 75 {
//             changed = true;
//             camera.angle += 5.0;
//         }
//
//         if scancode == 77 {
//             changed = true;
//             camera.angle -= 5.0;
//         }
//
//         // camera.x -= input.forwardbackward * Math.sin(camera.angle) * (current-time)*0.03;
//         // camera.y -= input.forwardbackward * Math.cos(camera.angle) * (current-time)*0.03;
//
//         if scancode == 72 {
//             camera.x -= (5.0 * sinf32(radiansf32(normalizef32(camera.angle)))) as i32;
//             camera.y -= (5.0 * cosf32(radiansf32(normalizef32(camera.angle)))) as i32;
//             changed = true;
//         }
//
//         if scancode == 80 {
//             camera.x += (5.0 * sinf32(radiansf32(normalizef32(camera.angle)))) as i32;
//             camera.y += (5.0 * cosf32(radiansf32(normalizef32(camera.angle)))) as i32;
//             changed = true;
//         }
//
//         if changed {
//             unsafe {
//                 camera.render(240.0);
//             }
//         }
//     }
//
//     debugln!("Key: <{}>", scancode);
//     // let scancode = unsafe { ::x86_64::instructions::port::inb(0x60) };
// }

#[no_mangle]
pub fn main() {
    // boringos_std::thread::new_thread(new_thread, 1, 2, 3, 4);

    boringos_std::process::exec("/bin/test").unwrap();

    //

    // debugln!("here");
    // boringos_std::process::register_kb_interrupt(keyboard_test);

    boringos_std::screen::init();
    video::raycaster::test();
    // CAMERA.call_once(|| Mutex::new(Camera::new()));
    // CAMERA.try().unwrap().lock().height = 160;
    // unsafe {
    //     CAMERA.try().unwrap().lock().render(240.0);
    // }
    //
    // boringos_std::thread::register_kb_interrupt(keyboard_test);

    // // boringos_std::screen::test();
    // // video::mandelbrot::draw();
    // video::voxelspace::test();
    // let mut buffer = SCREEN.try().unwrap().lock();
    // buffer.write(100, 100, &[255, 0, 255, 0]);
    // video::voxelspace::test();
    // let mut i = 100;

    loop {
        boringos_std::thread::sleep(500);
        // if let Some(kb) = IRQ8_INTERRUPT_TICKS.lock() {
        // debugln!("{}", *IRQ8_INTERRUPT_TICKS.lock());
        // }
        debugln!("Test");
    }
}
