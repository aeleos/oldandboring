use allocator;
pub use self::paging::{PhysicalAddress, VirtualAddress};
pub use self::paging::{remap_the_kernel, test_paging, EntryFlags, Page};
pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::stack_allocator::Stack;

use super::BOOT_INFO;

use spin::Mutex;

mod area_frame_allocator;
pub mod paging;
mod stack_allocator;

pub const PAGE_SIZE: usize = 4096;

lazy_static! {
    pub static ref MEMORY_CONTROLLER: Mutex<MemoryController> = Mutex::new(init());
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}


pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(active_table, frame_allocator, size_in_pages)
    }

    pub fn map_to(&mut self, page: Page, frame: Frame, flags: EntryFlags) {
        self.active_table
            .map_to(page, frame, flags, &mut self.frame_allocator)
    }


    pub fn map(&mut self, page: Page, flags: EntryFlags) {
        self.active_table
            .map(page, flags, &mut self.frame_allocator)
    }

    pub fn identity_map(&mut self, frame: Frame, flags: EntryFlags) {
        self.active_table
            .identity_map(frame, flags, &mut self.frame_allocator)
    }

    pub fn unmap(&mut self, page: Page) {
        self.active_table.unmap(page, &mut self.frame_allocator)
    }

    pub fn map_page_at(
        &mut self,
        virtual_address: VirtualAddress,
        physical_address: PhysicalAddress,
        flags: EntryFlags,
    ) {
        self.active_table.map_page_at(virtual_address, physical_address, flags, &mut self.frame_allocator);
    }


    pub fn frame_range_inclusive(&self, start_address: usize, end_address: usize) -> FrameIter {
        let start_frame = Frame::containing_address(start_address);
        let end_frame = Frame::containing_address(end_address);
        Frame::range_inclusive(start_frame, end_frame)
    }
}


pub fn init() -> MemoryController {
    assert_has_not_been_called!("memory::init can only be called once");

    let boot_info = BOOT_INFO.try().unwrap();

    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.end_address())
        .max()
        .unwrap();

    debugln!(
        "kernel start: 0x{:#x}, kernel end: 0x{:#x}",
        kernel_start,
        kernel_end
    );
    debugln!(
        "multiboot start: 0x{:#x}, multiboot end: 0x{:#x}",
        boot_info.start_address(),
        boot_info.end_address()
    );

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    let mut active_table = paging::remap_the_kernel(&mut frame_allocator, &boot_info);

    use allocator::{HEAP_SIZE, HEAP_START};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }

    unsafe {
        allocator::init(HEAP_START, HEAP_SIZE);
    }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}
