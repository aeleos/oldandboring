use volatile::Volatile;
use core::ptr::Unique;
use core::marker::Copy;
use spin::{Mutex, Once};

use core::mem::size_of;
use core::mem;
use super::vga_buffer;
use boot;

pub trait Pixel: Sized + Copy {
    fn new(red: u8, green: u8, blue: u8) -> Self;

    fn new_from_arr(colors: [u8; 3]) -> Self;
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct RGBPixel {
    blue: u8,
    green: u8,
    red: u8,
}

impl Pixel for RGBPixel {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        RGBPixel {
            blue: blue,
            green: green,
            red: red,
        }
    }

    fn new_from_arr(colors: [u8; 3]) -> Self {
        unsafe { mem::transmute::<[u8; 3], Self>(colors) }
    }
}

pub struct Buffer<T: Pixel> {
    address: Unique<Volatile<T>>,
    location: u64,
    width: u32,
    height: u32,
    pitch: u32,
    pixelwidth: u8,
}

impl<T: Pixel> Buffer<T> {
    pub fn new(info: vga_buffer::Info) -> Buffer<T> {
        Buffer {
            address: unsafe { Unique::new_unchecked(info.address as *mut _) },
            location: info.address as u64,
            width: info.width as u32,
            height: info.height as u32,
            pitch: info.pitch as u32,
            pixelwidth: info.bpp / 8,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel(&self, red: u8, green: u8, blue: u8) -> T {
        T::new(red, green, blue)
    }

    pub fn pixel_from_arr(&self, colors: [u8; 3]) -> T {
        T::new_from_arr(colors)
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

    pub unsafe fn write(&mut self, x: u32, y: u32, pixel: T) {
        let start = self.address.as_ptr();
        let location_ptr = start.offset(self.offset(x, y));
        (&mut *location_ptr).write(pixel);
    }

    pub unsafe fn read(&mut self, x: u32, y: u32) -> T {
        let start = self.address.as_ref() as *const Volatile<T>;
        let location_ptr = start.offset(self.offset(x, y));
        (&*location_ptr).read()
    }

    pub unsafe fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: T) {
        let dx = x2 as i32 - x1 as i32;
        let dy = y2 as i32 - y1 as i32;

        let dxabs = i32::abs(dx);
        let dyabs = i32::abs(dy);

        let sdx = i32::signum(dx);
        let sdy = i32::signum(dy);

        let mut x = dxabs >> 1;
        let mut y = dyabs >> 1;

        let mut px = x1 as i32;
        let mut py = y1 as i32;

        self.write(px as u32, py as u32, color);

        if dxabs >= dyabs {
            for _ in 0..dxabs {
                y += dyabs;

                if y >= dxabs {
                    y -= dxabs;
                    py += sdy;
                }

                px += sdx;
                self.write(px as u32, py as u32, color);
            }
        } else {
            for _ in 0..dyabs {
                x += dxabs;
                if x >= dyabs {
                    x -= dyabs;
                    px += sdx;
                }
                py += sdy;
                self.write(px as u32, py as u32, color);
            }
        }
    }

    pub unsafe fn vertical_line(&mut self, x: u32, ytop: u32, ybottom: u32, color: T) {
        if ytop > ybottom {
            return;
        }

        let start = self.address.as_ptr();
        let mut location_ptr = start.offset(self.offset(x, ytop));
        let pitch_pixels = self.pitch as isize / size_of::<T>() as isize;
        for _ in 0..(ybottom - ytop) as isize {
            (&mut *location_ptr).write(color);
            location_ptr = location_ptr.offset(pitch_pixels);
        }
    }

    pub unsafe fn horizontal_line(&mut self, x: u32, y: u32, w: u32, color: T) {
        let start = self.address.as_ptr();
        let location_ptr = start.offset(self.offset(x, y));
        for i in 0..w as isize {
            (&mut *location_ptr.offset(i)).write(color);
        }
    }

    pub unsafe fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: T) {
        let start = self.address.as_ptr();
        let mut location_ptr = start.offset(self.offset(x, y));
        let pitch_pixels = self.pitch as isize / size_of::<T>() as isize;
        for _ in 0..w {
            for j in 0..h as isize {
                (&mut *location_ptr.offset(j)).write(color);
            }
            location_ptr = location_ptr.offset(pitch_pixels);
        }
    }
}

pub static SCREEN: Once<Mutex<Buffer<RGBPixel>>> = Once::new();

pub fn init() {
    let info = boot::get_vga_info();

    SCREEN.call_once(|| Mutex::new(Buffer::new(info)));
}

pub fn test() {
    let mut buffer = SCREEN.try().unwrap().lock();

    let width = buffer.width;
    let height = buffer.height;

    unsafe {
        buffer.draw_line(0, 0, width - 100, height - 40, Pixel::new(255, 255, 255));
        buffer.draw_line(100, 99, 200, 99, Pixel::new(255, 255, 255));
        buffer.fill_rect(100, 100, 100, 100, Pixel::new(255, 10, 32));
        buffer.vertical_line(20, 20, 500, Pixel::new(255, 255, 255));
        buffer.horizontal_line(10, height - 100, width - 50, Pixel::new(10, 10, 255));
    }
}
