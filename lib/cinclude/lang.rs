// Lang items for Rust


use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {};

struct Allocator {}

// unsafe impl Send for Allocator {}
// unsafe impl Sync for Allocator {}

#[allow(clippy::unused_self)]
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        self.alloc(size, align)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        self.dealloc(ptr, size, align);
    }
}


#[allow(clippy::unused_self)]
impl Allocator {
    unsafe fn alloc(&self, _size: usize, _align: usize) -> *mut u8 {
        unimplemented!("alloc")
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _size: usize, _align: usize) {
        unimplemented!("dealloc")
    }
}

#[panic_handler]
#[allow(clippy::empty_loop)]
fn _panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
#[allow(clippy::empty_loop)]
unsafe fn _alloc_error_handler(_layout: Layout) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
