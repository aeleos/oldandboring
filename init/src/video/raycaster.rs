// pub mod map;
use boringos_std::screen::SCREEN;
// use sync::time::Timestamp;
use boringos_std::math::{cosf32, normalizef32, radiansf32, sinf32};
use core::mem;
use core;
use core::num::Wrapping;

// pub static mut YBUFFER: [u32; 1024] = [768; 1024];

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub height: u8,
    pub angle: f32,
    pub horizon: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 512,
            y: 512,
            height: 50,
            angle: 0.0,
            horizon: 150.0,
            distance: 200.0,
        }
    }

    #[inline(always)]
    pub unsafe fn render(&mut self) {
        let mut buffer = SCREEN.try().unwrap().lock();
        let screen_width = buffer.width();
        let screen_height = buffer.height();
        debugln!("width: {}, height: {}", screen_width, screen_height);
        let mut uvx: f32;
        let mut uvy: f32;
        let mut ynorm: u32;
        let mut z: u8;
        let mut ydist: u32;
        for x in 0..screen_width {
            for y in 0..screen_height {
                uvx = x as f32 / screen_width as f32;
                uvy = y as f32 / screen_height as f32;

                ynorm = (uvy * 200.0) as u32;
                z = 0;
                for i in 0..255 {
                    // debugln!("yorm: {}", ynorm);
                    ydist = (((Wrapping(ynorm) - Wrapping(100u32)) * Wrapping(z as u32)
                        + Wrapping(4096u32)) >> 8)
                        .0;
                    // debugln!("{}", ydist);
                    if ydist >= 32 {
                        break;
                    }
                    z += 1;
                }
                // debugln!("here, x: {}, y: {}, z: {}", x, y, z);
                let pixel = mem::transmute::<[u8; 4], u32>([z, z, z, 0]);
                buffer.write(x, y, pixel);
            }
            buffer.sync();
        }
        // panic_debugln!("sinang: {}, cosang: {}", sinang, cosang);
    }
}

//     out vec4 outputColor;
// uniform vec2 iResolution;
// void main(void)
// {
//     vec2 uv = gl_FragCoord.xy / iResolution.xy;
//     uint y = uint(uv.y * 200.0);
//     uint z = 0u;
//     // the distance to wall(ceiling/floor)
//     uint ydist;
//     // we'll trace a maximun of 256 steps
//     for(int i = 0; i < 256; ++i)
//     {
//         // the distance to wall is propotional to depth,
//         // and propotional to the offset of the pixel from
//         // the center of the screen(which is 100, because
//         // the height of the screen is 200). In order to be
//         // consistent with wolf128, everything uses unsigned
//         // numbers, and we keep 8-bit precision for the result.
//         ydist = ((y - 100u) * z + 4096u) >> 8;
//         // If the distance exceeds a predefined threshold
//         if (ydist >= 32u)
//             break;
//         // next itration, increase depth by 1
//         ++z;
//     }
//
//     // normalize the depth value to [0,1]
//     float c = float(z) / 256.0;
//     outputColor = vec4(vec3(c), 1.0);
// }

pub fn test() {
    let mut camera = Camera::new();
    camera.height = 160;
    unsafe {
        camera.render();
    }
}
