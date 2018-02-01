pub mod map;
use boringos_std::screen::SCREEN;
// use sync::time::Timestamp;
use boringos_std::math::{cosf32, normalizef32, radiansf32, sinf32};
use core::mem;
use core;

// pub static mut YBUFFER: [u32; 1024] = [768; 1024];

struct Camera {
    x: i32,
    y: i32,
    pub height: u8,
    angle: f32,
    horizon: f32,
    distance: f32,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            x: 512,
            y: 512,
            height: 150,
            angle: 0.0,
            horizon: 100.0,
            distance: 300.0,
        }
    }

    unsafe fn render(&mut self, scale_height: f32) {
        let mut buffer = SCREEN.try().unwrap().lock();
        let screen_width = buffer.width();
        let screen_height = buffer.height();
        // debugln!("width: {}, height: {}", screen_width, screen_height);

        let sinang: f32 = sinf32(radiansf32(normalizef32(self.angle)));
        let cosang: f32 = cosf32(radiansf32(normalizef32(self.angle)));
        // panic_debugln!("sinang: {}, cosang: {}", sinang, cosang);

        let (mut plx, mut ply, mut prx, mut pry): (f32, f32, f32, f32);
        let (mut dx, mut dy): (f32, f32);

        let mut ybuffer = [screen_height; 1024];

        let mut mapoffset: usize;
        let mut invz: f32;

        let mut dz = 1.0;
        let mut z = 1.0;

        let mut pixel;
        let mut height: u8;
        let mut height_on_screen: u32;
        buffer.fill_rect(0, 0, 1023, 511, 0x007EC0EE);
        // let mut color;

        // debugln!("{:?}", Timestamp::get_current());

        while z < self.distance {
            plx = -cosang * z - sinang * z;
            ply = sinang * z - cosang * z;
            prx = cosang * z - sinang * z;
            pry = -sinang * z - cosang * z;

            dx = (prx - plx) / screen_width as f32;
            dy = (pry - ply) / screen_width as f32;

            plx += self.x as f32;
            ply += self.y as f32;

            invz = 1.0 / z * scale_height;
            for i in 0..screen_width as usize {
                // debugln!("dx: {}, dy: {}", dx, dy);
                // debugln!("plx: {}, ply: {}", round(plx), round(ply));
                mapoffset = ((ply as usize) << 10) + plx as usize;
                height = map::D1::D1[mapoffset];
                height_on_screen = ((self.height - height) as f32 * invz + self.horizon) as u32;
                pixel = mem::transmute::<[u8; 4], u32>(map::C1W::C1W[mapoffset]);
                buffer.vertical_line(i as u32, height_on_screen, ybuffer[i], pixel);
                if height_on_screen < ybuffer[i] {
                    ybuffer[i] = height_on_screen;
                }
                plx += dx;
                ply += dy;
            }

            z += dz;
            dz += 0.05;
        }
        buffer.sync();
        // debugln!("{:?}", Timestamp::get_current());
    }
    // pub fn draw_angle(&mut self, i: u32) {
    //     let mut angle_rad = core::f32::consts::PI * (i as f32) / 180.0;
    //     let mut sin = math::sin32(angle_rad);
    //     let mut cos = math::cos32(angle_rad);
    //     debugln!(
    //         "angle: {}, sin: {}, cos: {}",
    //         (angle_rad / core::f32::consts::PI) as u32,
    //         sin as u32,
    //         cos as u32
    //     );
    //
    //     self.angle = angle_rad;
    //     self.render(240.0);
    // }
}

pub fn test() {
    let mut camera = Camera::new();
    camera.height = 160;
    unsafe {
        camera.render(240.0);
        // camera.angle = 1.0;
        // camera.render(240.0);
        // camera.angle = 2.0;

        // camera.x = 128;
        for i in -180..180 {
            camera.angle = i as f32;
            camera.render(240.0);
        }
        // camera.angle = core::f32::consts::PI * (110 as f32) / 180.0;
        //
        // debugln!(
        //     "angle: {}, sin: {}, cos: {}",
        //     110,
        //     sinf32(radiansf32(normalizef32(110.0))),
        //     cosf32(radiansf32(normalizef32(110.0)))
        // );
        // camera.render(240.0);
        // camera.angle = core::f32::consts::PI * (111 as f32) / 180.0;
        // debugln!(
        //     "angle: {}, sin: {}, cos: {}",
        //     111,
        //     math::sin32(core::f32::consts::PI * (111 as f32) / 180.0),
        //     math::cos32(core::f32::consts::PI * (111 as f32) / 180.0)
        // );
        // camera.render(240.0);
        // camera.angle = core::f32::consts::PI * (112 as f32) / 180.0;
        // debugln!(
        //     "angle: {}, sin: {}, cos: {}",
        //     112,
        //     math::sin32(core::f32::consts::PI * (112 as f32) / 180.0),
        //     math::cos32(core::f32::consts::PI * (112 as f32) / 180.0)
        // );
        // camera.render(240.0);
        // camera.draw_angle(110);
        for i in 512..1024 {
            camera.y = i;
            camera.render(240.0);
            // camera.sync();
        }
    }
    // camera.render(240.0);
}
