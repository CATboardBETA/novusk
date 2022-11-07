// Lang items for Rust


use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {};

struct Allocator {}

// unsafe impl Send for Allocator {}
// unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        self.alloc(size, align)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        self.dealloc(ptr, size, align)
    }
}

impl Allocator {
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        unimplemented!("alloc")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize) {
        unimplemented!("dealloc")
    }
}

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
unsafe fn _alloc_error_handler(layout: Layout) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
