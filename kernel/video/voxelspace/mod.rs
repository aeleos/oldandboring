pub mod map;

use drivers::vga::video::SCREEN;
use sync::time::Timestamp;


struct Camera {
    x: i32,
    y: i32,
    pub height: u8,
    angle: f32,
    horizon: f32,
    distance: f32,
}



pub fn round(num: f32) -> u32 {
    (num + 0.5) as u32
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

    fn render(&mut self, scale_height: f32) {
        let mut buffer = SCREEN.try().unwrap().lock();
        let screen_width = buffer.width();
        let screen_height = buffer.height();

        const MAPWIDTHPERIOD: usize = 1023;

        let sinang = 1.0;
        let cosang = 0.0;

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
        // let mut color;

        debugln!("{:?}", Timestamp::get_current());

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
                height = map::height::HEIGHT_MAP[mapoffset];
                height_on_screen = ((self.height - height) as f32 * invz + self.horizon) as u32;
                // debugln!("height_on_screen: {}", height_on_screen);
                pixel = buffer.pixel_from_arr(map::color::COLOR_MAP[mapoffset]);
                // color = map::color::DATA_CMAP[map::color::DATA[mapoffset]];
                // pixel = buffer.pixel(color[0], color[1], color[2]);
                unsafe {
                    buffer.vertical_line(i as u32, height_on_screen, ybuffer[i as usize], pixel);
                }
                if height_on_screen < ybuffer[i] {
                    ybuffer[i] = height_on_screen;
                }
                plx += dx;
                ply += dy;
            }

            z += dz;
            dz += 0.05;
            // debugln!("{:?}", Timestamp::get_current());
        }
        debugln!("{:?}", Timestamp::get_current());
    }
}



pub fn test() {
    let mut camera = Camera::new();
    // for i in 150..256 {
    //     camera.height = i;
    //     camera.render(240.0);
    // }
    camera.render(240.0);
}
