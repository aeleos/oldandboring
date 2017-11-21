use volatile::Volatile;
use core::ptr::Unique;
use core::marker::Copy;

use core::mem::size_of;

use BOOT_INFO;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Pixel {
    blue: u8,
    green: u8,
    red: u8,
}

impl Pixel {
    pub fn new(blue: u8, green: u8, red: u8) -> Pixel {
        Pixel {
            blue: blue,
            green: green,
            red: red,
        }
    }
}

struct Buffer<T>
where
    T: Copy,
{
    address: Unique<Volatile<T>>,
    location: u64,
    width: u32,
    height: u32,
    pitch: u32,
    pixelwidth: u8,
}

impl<T> Buffer<T>
where
    T: Copy,
{
    pub fn new(address: u64, width: u32, height: u32, pitch: u32, pixelwidth: u8) -> Buffer<T> {
        Buffer {
            address: unsafe { Unique::new_unchecked(address as *mut _) },
            location: address,
            width,
            height,
            pitch,
            pixelwidth,
        }
    }

    fn init(&self) {
        use memory::{paging, MEMORY_CONTROLLER};

        let mut memory_controller = MEMORY_CONTROLLER.lock();

        let framebuffer_size = (self.width * self.height * self.pixelwidth as u32) as usize;
        let start_address = self.location as usize;
        let end_address = start_address + framebuffer_size;

        // indentity map the video buffer
        for frame in memory_controller.frame_range_inclusive(start_address, end_address) {
            memory_controller.identity_map(frame, paging::WRITABLE);
        }
    }

    fn offset(&self, x: u32, y: u32) -> isize {
        assert!(self.width > x);
        assert!(self.height > y);
        ((x * self.pixelwidth as u32 + y * self.pitch) / size_of::<T>() as u32) as isize
    }

    fn byte_offset(&self, x: u32, y: u32) -> isize {
        assert!(self.width > x);
        assert!(self.height > y);
        ((x * self.pixelwidth as u32 + y * self.pitch)) as isize
    }

    unsafe fn write(&mut self, x: u32, y: u32, pixel: T) {
        let start = self.address.as_ptr();
        let location_ptr = start.offset(self.offset(x, y));
        (&mut *location_ptr).write(pixel);
    }

    unsafe fn read(&mut self, x: u32, y: u32) -> T {
        let start = self.address.as_ref() as *const Volatile<T>;
        let location_ptr = start.offset(self.offset(x, y));
        (&*location_ptr).read()
    }
}



pub fn init() {
    let fb_info = BOOT_INFO.try().unwrap().fb_info_tag().unwrap();

    debugln!("framebuffer_info: {:?}", fb_info);

    let mut buffer = Buffer::new(
        fb_info.addr,
        fb_info.width,
        fb_info.height,
        fb_info.pitch,
        fb_info.bpp / 8,
    );

    buffer.init();

    let width = fb_info.width;
    let height = fb_info.height;


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

                let color = Pixel::new(
                    q as u8,
                    (q % max_iter) as u8,
                    if q < max_iter { 0 } else { 255 },
                );
                // debugln!(
                //     "putpixel: x: {}, y: {}, color: {}, {}, {}",
                //     x,
                //     y,
                //     q as u8,
                //     (q % max_iter) as u8,
                //     if q < max_iter { 0 } else { 255 }
                // );
                buffer.write(x, y, color)
            }
        }
    }

    // for i in 0..768 {
    //     unsafe {
    //         buffer.write(i, i, Pixel::new(0xff, 0xff, 0xff));
    //     };
    // }
    // for i in 0..256 {
    //     writer.putpixel(i + 768, i, 0xffffff);
    // }
}
