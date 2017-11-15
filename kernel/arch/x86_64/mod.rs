mod interrupts;
pub mod vga_buffer;
pub mod sync;

pub fn init() {
    assert_has_not_been_called!("x86_64 specific initialization code should only be called once.");

    interrupts::init();
}
