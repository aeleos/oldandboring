use volatile::Volatile;

use core::fmt;
use core::ptr::Unique;

use spin::Mutex;

// static void putpixel(unsigned char* screen, int x,int y, int color) {
//     unsigned where = x*4 + y*3200;
//     screen[where] = color & 255;              // BLUE
//     screen[where + 1] = (color >> 8) & 255;   // GREEN
//     screen[where + 2] = (color >> 16) & 255;  // RED
// }

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Pixel {
    blue: u8,
    green: u8,
    red: u8,
}

struct Buffer {
    chars: [Volatile<u8>; 10000000],
}

struct Writer {
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn new(addr: u32) -> Writer {
        Writer {
            buffer: unsafe { Unique::new_unchecked(addr as *mut _) },
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }


    pub fn putpixel(&mut self, x: u32, y: u32, color: u32) {
        let location = x * 3 + y * 3072;
        self.buffer().chars[location as usize].write((color & 255) as u8);
        self.buffer().chars[(location + 1) as usize].write(((color >> 8) & 255) as u8);
        self.buffer().chars[(location + 2) as usize].write(((color >> 16) & 255) as u8);

        // self.buffer().chars[location as usize].write(Pixel {
        //     blue: (color & 255) as u8,
        //     green: ((color >> 8) & 255) as u8,
        //     red: ((color >> 16) & 255) as u8,
        // })
    }

    //     static void fillrect(unsigned char *vram, unsigned char r,
    //unsigned char g, unsigned   char b, unsigned char w, unsigned char h) {
    //     unsigned char *where = vram;
    //     int i, j;
    //
    //     for (i = 0; i < w; i++) {
    //         for (j = 0; j < h; j++) {
    //             //putpixel(vram, 64 + j, 64 + i, (r << 16) + (g << 8) + b);
    //             where[j*4] = r;
    //             where[j*4 + 1] = g;
    //             where[j*4 + 2] = b;
    //         }
    //         where+=3200;
    //     }
    // }
}

pub fn init() {
    use memory::paging;
    use MEMORY_CONTROLLER;

    // let mut active_table = &MEMORY_CONTROLLER.lock().active_table;
    let mut memory_controller = MEMORY_CONTROLLER.lock();
    // let mut frame_allocator =
    // indentity map the VGA video buffer
    for frame in memory_controller.frame_range_inclusive(0xFD000000, 0xFD000000 + 10000000) {
        memory_controller.identity_map(frame, paging::WRITABLE);
    }


    let mut writer = Writer::new(0xFD000000);
    for i in 0..768 {
        writer.putpixel(i, i, 0xffffff);
    }
    // for i in 0..256 {
    //     writer.putpixel(i + 768, i, 0xffffff);
    // }
}
