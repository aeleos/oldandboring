#![no_std]

#[macro_use]
extern crate boringos_std;
#[allow(unused_extern_crates)]
extern crate rlibc;

// use boringos_std::math;
// use core;
use boringos_std::math::{PI32, PI64, cosf32, normalizef32, radiansf32, sinf32};

#[no_mangle]
pub fn main() {
    loop {
        let a: f32 = 432.432432;
        let b: f32 = 6.654654;
        let c = a * b;
        debugln!("start");

        //
        // debugln!("normalize test: {}, {}", -1236.7, normalizef32(-1236.7));
        //

        // let mut angle = 0.0;
        // let mut sinang: f32 = sinf32(radiansf32(normalizef32(angle)));
        // let mut cosang: f32 = cosf32(radiansf32(normalizef32(angle)));
        // debugln!("angle: {}, sin: {}, cos: {}", angle, sinang, cosang);
        //
        // angle = 1.0;
        // sinang = sinf32(radiansf32(normalizef32(angle)));
        // cosang = cosf32(radiansf32(normalizef32(angle)));
        // debugln!("angle: {}, sin: {}, cos: {}", angle, sinang, cosang);
        //
        // angle = 45.0;
        // sinang = sinf32(radiansf32(normalizef32(angle)));
        // cosang = cosf32(radiansf32(normalizef32(angle)));
        // debugln!("angle: {}, sin: {}, cos: {}", angle, sinang, cosang);

        // angle = 50.0;
        // sinang = sinf32(radiansf32(normalizef32(angle)));
        // cosang = cosf32(radiansf32(normalizef32(angle)));
        // debugln!("angle: {}, sin: {}, cos: {}", angle, sinang, cosang);

        // //
        // // sinang = 0.0.normalizef32().radians().sinf32();
        // // cosang = 0.0.normalizef32().radians().cosf32();
        // // debugln!("angle: {}, sin: {}, cos: {}", 1.0, sinang, cosang);
        // //
        // // sinang = PI32.normalizef32().radians().sinf32();
        // // cosang = PI32.normalizef32().radians().cosf32();
        // // debugln!("angle: {}, sin: {}, cos: {}", 1, sinang, cosang);
        // //
        // // sinang = (0.5 * PI32).normalizef32().radians().sinf32();
        // // cosang = (0.5 * PI32).normalizef32().radians().cosf32();
        // // debugln!("angle: {}, sin: {}, cos: {}", 0.5, sinang, cosang);
        // // sinang = (0.25 * PI32).normalizef32().radians().sinf32();
        // // cosang = (0.25 * PI32).normalizef32().radians().cosf32();
        // // debugln!("angle: {}, sin: {}, cos: {}", 0.25, sinang, cosang);
        // sinang = sinf32(radiansf32(normalizef32(90.0)));
        // cosang = cosf32(radiansf32(normalizef32(90.0)));
        // debugln!("angle: {}, sin: {}, cos: {}", 2.0, sinang, cosang);

        boringos_std::thread::sleep(1000);
        debugln!("Nest: a: {}, b: {}, c: {}", a, b, c);
    }
}
