#![feature(const_fn)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![feature(alloc)]
#![no_std]

use spin::Mutex;
use linked_list_allocator::Heap;
use alloc::heap::{Alloc, AllocErr, Layout, Excess, CannotReallocInPlace};
use core::{ptr, cmp};


extern crate alloc;
extern crate spin;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

lazy_static! {
    static ref HEAP: Mutex<Heap> = Mutex::new(
        unsafe { Heap::new(HEAP_START, HEAP_SIZE) }
    );
}

pub struct HoleListAllocator;


unsafe impl Alloc for HoleListAllocator {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        (&*self).alloc(layout)
    }

    // #[inline]
    // unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
    //     (&*self).alloc_zeroed(layout)
    // }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        (&*self).dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(
        &mut self,
        ptr: *mut u8,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<*mut u8, AllocErr> {
        (&*self).realloc(ptr, old_layout, new_layout)
    }

    // fn oom(&mut self, err: AllocErr) -> ! {
    //     (&*self).oom(err)
    // }

    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        (&self).usable_size(layout)
    }

    // #[inline]
    // unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> {
    //     (&*self).alloc_excess(layout)
    // }
    //
    // #[inline]
    // unsafe fn realloc_excess(
    //     &mut self,
    //     ptr: *mut u8,
    //     layout: Layout,
    //     new_layout: Layout,
    // ) -> Result<Excess, AllocErr> {
    //     (&*self).realloc_excess(ptr, layout, new_layout)
    // }

    #[inline]
    unsafe fn grow_in_place(
        &mut self,
        ptr: *mut u8,
        layout: Layout,
        new_layout: Layout,
    ) -> Result<(), CannotReallocInPlace> {
        (&*self).grow_in_place(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn shrink_in_place(
        &mut self,
        ptr: *mut u8,
        layout: Layout,
        new_layout: Layout,
    ) -> Result<(), CannotReallocInPlace> {
        (&*self).shrink_in_place(ptr, layout, new_layout)
    }
}

unsafe impl<'a> Alloc for &'a HoleListAllocator {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let ptr = HEAP.lock().allocate_first_fit(layout.clone()).expect(
            "out of memory",
        );
        if ptr.is_null() {
            Err(AllocErr::Exhausted { request: layout })
        } else {
            Ok(ptr as *mut u8)
        }

    }

    // #[inline]
    // unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
    //     // let ptr = if layout.align() <= MIN_ALIGN {
    //     //     ffi::calloc(layout.size(), 1)
    //     // } else {
    //     //     let flags = align_to_flags(layout.align()) | ffi::MALLOCX_ZERO;
    //     //     ffi::mallocx(layout.size(), flags)
    //     // };
    //     // if ptr.is_null() {
    //     //     Err(AllocErr::Exhausted { request: layout })
    //     // } else {
    //     //     Ok(ptr as *mut u8)
    //     // }
    //     Ok()
    // }
    //
    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        HEAP.lock().deallocate(ptr, layout);
    }

    #[inline]
    unsafe fn realloc(
        &mut self,
        ptr: *mut u8,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<*mut u8, AllocErr> {
        use core::{ptr, cmp};

        if old_layout.align() != new_layout.align() {
            return Err(AllocErr::Unsupported { details: "cannot change align" });
        }

        let new_ptr = self.alloc(new_layout.clone()).ok().unwrap();
        if new_ptr.is_null() {
            Err(AllocErr::Exhausted { request: new_layout })
        } else {
            unsafe {
                ptr::copy(ptr, new_ptr, cmp::min(old_layout.size(), new_layout.size()));
            }
            self.dealloc(ptr, old_layout);
            Ok(new_ptr as *mut u8)
        }
    }

    // fn oom(&mut self, err: AllocErr) -> ! {
    //     // System.oom(err)
    // }

    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        (layout.size(), layout.size())
        // let flags = align_to_flags(layout.align());
        // unsafe {
        //     let max = ffi::nallocx(layout.size(), flags);
        //     (layout.size(), max)
        // }
    }

    #[inline]
    unsafe fn grow_in_place(
        &mut self,
        ptr: *mut u8,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<(), CannotReallocInPlace> {
        self.shrink_in_place(ptr, old_layout, new_layout)
    }

    #[inline]
    unsafe fn shrink_in_place(
        &mut self,
        ptr: *mut u8,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<(), CannotReallocInPlace> {
        // if old_layout.align() != new_layout.align() {
        //     return Err(CannotReallocInPlace);
        // }
        // let flags = align_to_flags(new_layout.align());
        // let size = ffi::xallocx(ptr as *mut c_void, new_layout.size(), 0, flags);
        // if size >= new_layout.size() {
        //     Err(CannotReallocInPlace)
        // } else {
        //     Ok(())
        // }
        return Err(CannotReallocInPlace);
    }
}




// Align downwards, returns the greatest  xwith aligmment
// aligns so that x <= addr. The alignment must be a
// power of 2
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("align must be power of two");
    }
}

// Align upwards.
// Returns the smallest x with alignment `align`
// so that x >= addr. The alignment must be a power of 2
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}


#[no_mangle]
pub unsafe extern "C" fn __rdl_alloc(size: usize, align: usize, err: *mut u8) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size, align);
    match HoleListAllocator.alloc(layout) {
        Ok(p) => p,
        Err(e) => {
            ptr::write(err as *mut AllocErr, e);
            0 as *mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_oom(err: *const u8) -> ! {
    HoleListAllocator.oom((*(err as *const AllocErr)).clone())
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_dealloc(ptr: *mut u8, size: usize, align: usize) {
    HoleListAllocator.dealloc(ptr, Layout::from_size_align_unchecked(size, align))
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_usable_size(layout: *const u8, min: *mut usize, max: *mut usize) {
    let pair = HoleListAllocator.usable_size(&*(layout as *const Layout));
    *min = pair.0;
    *max = pair.1;
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_realloc(
    ptr: *mut u8,
    old_size: usize,
    old_align: usize,
    new_size: usize,
    new_align: usize,
    err: *mut u8,
) -> *mut u8 {
    let old_layout = Layout::from_size_align_unchecked(old_size, old_align);
    let new_layout = Layout::from_size_align_unchecked(new_size, new_align);
    match HoleListAllocator.realloc(ptr, old_layout, new_layout) {
        Ok(p) => p,
        Err(e) => {
            ptr::write(err as *mut AllocErr, e);
            0 as *mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_alloc_zeroed(size: usize, align: usize, err: *mut u8) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size, align);
    match HoleListAllocator.alloc_zeroed(layout) {
        Ok(p) => p,
        Err(e) => {
            ptr::write(err as *mut AllocErr, e);
            0 as *mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_alloc_excess(
    size: usize,
    align: usize,
    excess: *mut usize,
    err: *mut u8,
) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size, align);
    match HoleListAllocator.alloc_excess(layout) {
        Ok(p) => {
            *excess = p.1;
            p.0
        }
        Err(e) => {
            ptr::write(err as *mut AllocErr, e);
            0 as *mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_realloc_excess(
    ptr: *mut u8,
    old_size: usize,
    old_align: usize,
    new_size: usize,
    new_align: usize,
    excess: *mut usize,
    err: *mut u8,
) -> *mut u8 {
    let old_layout = Layout::from_size_align_unchecked(old_size, old_align);
    let new_layout = Layout::from_size_align_unchecked(new_size, new_align);
    match HoleListAllocator.realloc_excess(ptr, old_layout, new_layout) {
        Ok(p) => {
            *excess = p.1;
            p.0
        }
        Err(e) => {
            ptr::write(err as *mut AllocErr, e);
            0 as *mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_grow_in_place(
    ptr: *mut u8,
    old_size: usize,
    old_align: usize,
    new_size: usize,
    new_align: usize,
) -> u8 {
    let old_layout = Layout::from_size_align_unchecked(old_size, old_align);
    let new_layout = Layout::from_size_align_unchecked(new_size, new_align);
    match HoleListAllocator.grow_in_place(ptr, old_layout, new_layout) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rdl_shrink_in_place(
    ptr: *mut u8,
    old_size: usize,
    old_align: usize,
    new_size: usize,
    new_align: usize,
) -> u8 {
    let old_layout = Layout::from_size_align_unchecked(old_size, old_align);
    let new_layout = Layout::from_size_align_unchecked(new_size, new_align);
    match HoleListAllocator.shrink_in_place(ptr, old_layout, new_layout) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}
