mod primitive;

use volatile::Volatile;
use core::ptr::Unique;
use core::marker::Copy;
use spin::{Mutex, Once};
use core::{cmp, slice};
use core::mem::size_of;
use core::mem;
use self::primitive::{fast_copy32, fast_set32, fast_set64};
// use alloc::allocator::{Alloc, Layout};
// use alloc::heap::Heap;
//
pub static mut OFFSCREEN: [u32; 786432] = [0; 786432];
// pub static mut OFFSCREEN: &'static mut [u32] = &mut [0; 1048576];

//
// pub trait Pixel: Sized + Copy {
//     fn new(red: u8, green: u8, blue: u8) -> Self;
//
//     fn new_from_arr(colors: [u8; 3]) -> Self;
// }
//
// #[derive(Debug, Clone, Copy)]
// #[repr(C, packed)]
// pub struct RGBPixel {
//     blue: u8,
//     green: u8,
//     red: u8,
// }
//
// impl Pixel for RGBPixel {
//     fn new(red: u8, green: u8, blue: u8) -> Self {
//         RGBPixel {
//             blue: blue,
//             green: green,
//             red: red,
//         }
//     }
//
//     fn new_from_arr(colors: [u8; 3]) -> Self {
//         unsafe { mem::transmute::<[u8; 3], Self>(colors) }
//     }
// }

pub struct Pixel {
    unused_: u8,
    blue: u8,
    green: u8,
    red: u8,
}

impl Pixel {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Pixel {
            unused_: 0,
            blue: blue,
            green: green,
            red: red,
        }
    }

    fn new_from_arr(colors: [u8; 4]) -> u32 {
        unsafe { mem::transmute::<[u8; 4], u32>(colors) }
    }
}

pub struct Buffer {
    onscreen: &'static mut [u32],
    offscreen: &'static mut [u32],
    location: u64,
    width: u32,
    height: u32,
    pitch: u32,
    pixelwidth: u8,
    len: usize,
}

impl Buffer {
    pub fn new(info: Info) -> Buffer {
        Buffer {
            onscreen: unsafe {
                slice::from_raw_parts_mut(info.address as *mut u32, info.width * info.height)
            },
            offscreen: unsafe { &mut OFFSCREEN },
            location: info.address as u64,
            width: info.width as u32,
            height: info.height as u32,
            pitch: info.pitch as u32,
            pixelwidth: info.bpp / 8,
            len: info.width * info.height as usize,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn offset(&self, x: u32, y: u32) -> isize {
        (x + y * 1024) as isize
    }

    pub unsafe fn write(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        fast_set32(
            self.offscreen.as_mut_ptr().offset(self.offset(x, y)) as *mut u32,
            color,
            1,
        );
    }

    pub unsafe fn write_buf(&mut self, x: u32, y: u32, buf: &[u32]) {
        if x >= self.width || y >= self.height {
            return;
        }

        let seek = self.offset(x, y);
        let size = cmp::max(
            0,
            cmp::min(self.offscreen.len() as isize - seek, (buf.len()) as isize),
        ) as usize;
        if size > 0 {
            unsafe {
                fast_copy32(
                    self.offscreen.as_mut_ptr().offset(seek) as *mut u32,
                    buf.as_ptr(),
                    size,
                );
            }
        }
    }

    pub unsafe fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        if x1 >= self.width || y1 >= self.height || x2 >= self.width || y2 >= self.height {
            return;
        }

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

    pub unsafe fn vertical_line(&mut self, x: u32, ytop: u32, ybottom: u32, color: u32) {
        if ytop >= ybottom || x >= self.width || ytop >= self.height {
            return;
        }

        // let start = self.onscreen.as_mut_ptr();
        // let mut location_ptr = start.offset(self.offset(x, ytop));
        let mut onscreen_ptr = self.offscreen.as_mut_ptr().offset(self.offset(x, ytop)) as usize;

        let offset = self.pitch as usize;
        for _ in 0..(ybottom - ytop) as isize {
            fast_set32(onscreen_ptr as *mut u32, color, 1);
            onscreen_ptr += offset;
        }
        // fast_set32(onscreen_ptr as *mut u32, color, 1);
    }

    pub unsafe fn horizontal_line(&mut self, xtop: u32, xbottom: u32, y: u32, color: u32) {
        if xtop >= xbottom || xtop >= self.width || y >= self.height {
            return;
        }

        let start = self.offscreen.as_mut_ptr();
        let mut location_ptr = start.offset(self.offset(xtop, y));
        fast_set32(location_ptr as *mut u32, color, (xtop - xbottom) as usize);
    }

    pub unsafe fn fill_rect(
        &mut self,
        mut x1: u32,
        mut y1: u32,
        mut x2: u32,
        mut y2: u32,
        color: u32,
    ) {
        if x1 >= self.width || y1 >= self.height || x2 >= self.width || y2 >= self.height {
            return;
        }

        if (x1 > x2) {
            let temp = x1;
            x1 = x2;
            x2 = temp;
        }

        if (y1 > y2) {
            let temp = y1;
            y1 = y2;
            y2 = temp;
        }

        let mut onscreen_ptr = self.offscreen.as_mut_ptr().offset(self.offset(x1, y1)) as usize;

        let offset = self.pitch as usize;
        for _ in 0..(y2 - y1) {
            fast_set32(onscreen_ptr as *mut u32, color, (x2 - x1) as usize);
            onscreen_ptr += offset;
        }
    }

    pub unsafe fn sync(&mut self) {
        // let start_y = cmp::min(self.height, y);
        // let end_y = cmp::min(self.height, y + h);
        //
        // let start_x = cmp::min(self.width, x);
        // let len = (cmp::min(self.width, x + w) - start_x) * 4;
        let mut offscreen_ptr = self.offscreen.as_mut_ptr() as usize;
        let mut onscreen_ptr = self.onscreen.as_mut_ptr() as usize;

        fast_copy32(
            onscreen_ptr as *mut u32,
            offscreen_ptr as *const u32,
            self.len as usize,
        );

        // let stride = self.width * 4;
        //
        // let offset = y * stride + start_x * 4;
        // offscreen_ptr += offset;
        // onscreen_ptr += offset;
        //
        // let mut rows = end_y - start_y;
        // while rows > 0 {
        //     unsafe {
        //         fast_copy(onscreen_ptr as *mut u8, offscreen_ptr as *const u8, len);
        //     }
        //     offscreen_ptr += stride;
        //     onscreen_ptr += stride;
        //     rows -= 1;
        // }
    }
}

pub static SCREEN: Once<Mutex<Buffer>> = Once::new();

pub struct Info {
    pub height: usize,
    pub width: usize,
    pub address: usize,
    pub bpp: u8,
    pub pitch: usize,
}

pub fn init() {
    let info = Info {
        height: 768,
        width: 1024,
        address: 0xffff8000fd000000,
        bpp: 32,
        pitch: 4096,
    };

    SCREEN.call_once(|| Mutex::new(Buffer::new(info)));
}

pub fn test() {
    let mut buffer = SCREEN.try().unwrap().lock();

    let width = buffer.width;
    let height = buffer.height;

    // buffer.write(100, 100, &[255, 255, 255, 0, 255, 255, 255, 0]);
    // buffer.write(101, 100, &[255, 255, 255, 0]);
    // for i in 0..100 {
    //     buffer.write(i, 100, &[0x00ffffff]);
    // }

    unsafe {
        buffer.write_buf(100, 100, &[255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0]);
        buffer.draw_line(0, 0, width - 100, height - 20, 0x00ffffff);
        buffer.draw_line(100, 99, 200, 99, 0x00ffffff);
        buffer.fill_rect(100, 100, 100, 100, 0x00ff0a1f);
        buffer.vertical_line(20, 20, 600, 0x000affff);
        buffer.horizontal_line(10, height - 100, width - 50, 0x00ffffff);
    }
}
