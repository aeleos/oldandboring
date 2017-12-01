use drivers::vga::video::SCREEN;


pub fn draw() {
    let mut pr: f64;
    let mut pi: f64;
    let mut new_re: f64;
    let mut new_im: f64;
    let mut old_re: f64;
    let mut old_im: f64;
    let zoom: f64 = 1.0;
    let move_x: f64 = -0.5;
    let move_y: f64 = 0.0;
    let max_iter = 300;

    let mut buffer = SCREEN.try().unwrap().lock();

    let width = buffer.width();
    let height = buffer.height();

    unsafe {
        for y in 0..height {
            for x in 0..width {
                pr = 1.5
                    * ((x as i32 - (width as i32 / 2)) as f64 / (0.5 * zoom * width as f64)) as f64
                    + move_x;
                pi = ((y as i32 - (height as i32 / 2)) as f64 / (0.5 * zoom * height as f64)) as f64
                    + move_y;

                new_re = 0.0;
                new_im = 0.0;
                let mut q: i32 = 0;
                for i in 0..max_iter {
                    q = i;
                    old_re = new_re;
                    old_im = new_im;

                    new_re = old_re * old_re - old_im * old_im + pr;
                    new_im = 2.0 * old_re * old_im + pi;

                    if (new_re * new_re + new_im * new_im) as u64 > 4 {
                        break;
                    }
                }

                let color = buffer.pixel(
                    if q < max_iter { 0 } else { 255 },
                    q as u8,
                    (q % max_iter) as u8,
                );

                buffer.write(x, y, color)
            }
        }
    }
}
