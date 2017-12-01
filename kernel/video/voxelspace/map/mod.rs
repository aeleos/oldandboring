pub mod color;
pub mod height;

pub fn get_color(x: u32, y: u32) -> [u8; 3] {
    color::DATA_CMAP[color::DATA[((y << 10) + x) as usize] as usize]
}

pub fn get_height(x: u32, y: u32) -> u8 {
    height::DATA_CMAP[height::DATA[((y << 10) + x) as usize] as usize][0]
}
