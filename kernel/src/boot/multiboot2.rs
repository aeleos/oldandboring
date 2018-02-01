//! Handles the multiboot2 information structure.

use memory::{FreeMemoryArea, PhysicalAddress};
use spin::Once;
use multiboot2;
use arch::vga_buffer;

static BOOT_INFO: Once<&multiboot2::BootInformation> = Once::new();

/// Initializes the multiboot module.
pub fn init(information_structure_address: usize) {
    assert_has_not_been_called!("The multiboot2 module should only be initialized once.");
    BOOT_INFO.call_once(|| unsafe { multiboot2::load(information_structure_address) });
}

/// Returns the VGA buffer information requested.
#[cfg(target_arch = "x86_64")]
pub fn get_vga_info() -> vga_buffer::Info {
    match BOOT_INFO.try().unwrap().fb_info_tag() {
        Some(framebuffer_tag) => vga_buffer::Info {
            height: framebuffer_tag.height as usize,
            width: framebuffer_tag.width as usize,
            address: to_virtual!(framebuffer_tag.addr) as usize,
            bpp: framebuffer_tag.bpp,
            pitch: framebuffer_tag.pitch as usize,
        },
        None => vga_buffer::Info {
            height: 25,
            width: 80,
            address: to_virtual!(0xb8000),
            bpp: 16,
            pitch: 160,
        },
    }
}

/// Returns the bootloader name.
pub fn get_bootloader_name() -> &'static str {
    match BOOT_INFO.try().unwrap().boot_loader_name_tag() {
        Some(boot_loader_name_tag) => boot_loader_name_tag.name(),
        None => "a multiboot compliant bootloader",
    }
}

/// Returns the module entry for the initramfs.
fn get_initramfs_module_entry() -> &'static multiboot2::ModuleTag {
    for module in BOOT_INFO.try().unwrap().module_tags() {
        if module.name() == "initramfs" {
            return module;
        }
    }

    panic!("No initramfs found.");
}

/// Returns the start address of the initramfs.
pub fn get_initramfs_start() -> PhysicalAddress {
    get_initramfs_module_entry().start_address() as usize
}

/// Returns the length of the initramfs.
pub fn get_initramfs_length() -> usize {
    let entry = get_initramfs_module_entry();

    entry.end_address() as usize - entry.start_address() as usize
}

/// Provides an iterator for the memory map.
pub struct MemoryMapIterator {
    /// Iterator for current memory.
    memory: multiboot2::MemoryAreaIter,
}

impl MemoryMapIterator {
    /// Creates a new iterator through the memory map.
    fn new() -> MemoryMapIterator {
        MemoryMapIterator {
            memory: BOOT_INFO
                .try()
                .unwrap()
                .memory_map_tag()
                .expect("missing multiboot memory map tag")
                .memory_areas(),
        }
    }
}

impl Iterator for MemoryMapIterator {
    type Item = FreeMemoryArea;

    fn next(&mut self) -> Option<FreeMemoryArea> {
        while let Some(next_area) = self.memory.next() {
            if next_area.area_type() == multiboot2::MemoryAreaType::Usable {
                return Some(FreeMemoryArea::new(
                    next_area.start_address(),
                    next_area.size(),
                ));
            }
        }
        None
    }
}

/// Returns the memory map given by the boot loader.
pub fn get_memory_map() -> MemoryMapIterator {
    MemoryMapIterator::new()
}
