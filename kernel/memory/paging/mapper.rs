use super::{VirtualAddress, PhysicalAddress, Page, ENTRY_COUNT};
use super::entry::*;
use super::table::{self, Table, Level4};
use memory::{PAGE_SIZE, Frame, FrameAllocator};
use core::ptr::Unique;


pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper { p4: Unique::new_unchecked(table::P4) }
    }

    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    // Translates a virtual address to the corresponding
    // phyiscal address
    // Returns `None` if the address is not mapped
    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p3 = self.p4().next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // 1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p2_index() * ENTRY_COUNT +
                                page.p1_index(),
                        });
                    }
                }

                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    // 2MiB page?
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(HUGE_PAGE) {
                            assert!(start_frame.number % ENTRY_COUNT == 0);
                            return Some(Frame { number: start_frame.number + page.p1_index() });
                        }
                    }
                }

                None
            })
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(huge_page)
    }


    // Maps the page to the frame with the provided flags.
    // The `PRESRNT` flag is added by default.
    // It needs a `FrameAllocator` as it might
    // Need to create new page tables.
    pub fn map_to<A: FrameAllocator>(
        &mut self,
        page: Page,
        frame: Frame,
        flags: EntryFlags,
        allocator: &mut A,
    ) {
        let p4 = self.p4_mut();
        let p3 = p4.next_table_create(page.p4_index(), allocator);
        let p2 = p3.next_table_create(page.p3_index(), allocator);
        let p1 = p2.next_table_create(page.p2_index(), allocator);
        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    // Maps the page to some free frame with the provided flags.
    // The free frame is allocated from the given `FrameAllocator`
    pub fn map<A: FrameAllocator>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A) {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)

    }

    pub fn map_page_at<A: FrameAllocator>(
        &mut self,
        virtual_address: VirtualAddress,
        physical_address: PhysicalAddress,
        flags: EntryFlags,
        allocator: &mut A,
    ) {
        let frame = Frame::containing_address(physical_address);
        let page = Page::containing_address(virtual_address);
        self.map_to(page, frame, flags, allocator);
    }


    // Identity map the given frame with the provided flags
    // The `FrameAllocator` is used to create new page
    // tables if needed
    pub fn identity_map<A: FrameAllocator>(
        &mut self,
        frame: Frame,
        flags: EntryFlags,
        allocator: &mut A,
    ) {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator);
    }

    // Unmaps the given page and adds all freed frames
    // to the given `FrameAllocator`
    #[allow(unused_variables)]
    pub fn unmap<A: FrameAllocator>(&mut self, page: Page, allocator: &mut A) {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();

        use x86_64::instructions::tlb;
        use x86_64::VirtualAddress;
        tlb::flush(VirtualAddress(page.start_address()));


        // TODO free p(1,2,3) table if empty
        // allocator.deallocate_frame(frame);
    }
}
