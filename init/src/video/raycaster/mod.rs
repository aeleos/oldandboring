// pub mod map;
use boringos_std::screen::SCREEN;
// use sync::time::Timestamp;
use boringos_std::math::{cosf32, normalizef32, radiansf32, sinf32};
use core::mem;
use core;
use core::num::Wrapping;

//mod rustcaster;

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
        let mut xnorm: u32;
        let mut z: u8;
        let mut ydist: u32 = 0;
        let mut xdist: u32 = 0;
        for x in 0..screen_width {
            for y in 0..screen_height {
                uvx = x as f32 / screen_width as f32;
                uvy = y as f32 / screen_height as f32;

                xnorm = (uvy * 320.0) as u32;
                ynorm = (uvx * 200.0) as u32;

                z = 0;
                for i in 0..255 {
                    xdist = (((Wrapping(xnorm) - Wrapping(160u32)) * Wrapping(z as u32)
                        + Wrapping(4096u32)) >> 8)
                        .0;
                    ydist = (((Wrapping(ynorm) - Wrapping(100u32)) * Wrapping(z as u32)
                        + Wrapping(4096u32)) >> 8)
                        .0;
                    // debugln!("{}", ydist);
                    if (ydist >= 32 && ((z & 32) != 0)) || xdist >= 32 {
                        break;
                    }

                    // if ((xdist >=32u && ((z & 32u) != 0u)) || (ydist >= 32u))
                    // break;

                    z += 1;
                }
                let mut texel: u8 = (xdist ^ ydist ^ (z as u32)) as u8;
                texel %= 16;
                texel *= 16;
                let pixel = mem::transmute::<[u8; 4], u32>([texel, texel, texel, 0]);
                buffer.write(x, y, pixel);
            }
            buffer.sync();
        }
        // panic_debugln!("sinang: {}, cosang: {}", sinang, cosang);
    }
}

// uint texel = xdist ^ ydist ^ z;
//  texel %= 16u; // adjust the period of the pattern to your liking
//  float c = float(texel) / 16.0;
//  outputColor = vec4(vec3(c), 1.0);

pub fn test() {
    let mut camera = Camera::new();
    camera.height = 160;
    unsafe {
        camera.render();
    }
}
