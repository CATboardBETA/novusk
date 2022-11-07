#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(lang_items)]
#![feature(sync_unsafe_cell)]

use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::SyncUnsafeCell;
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::SeqCst;

fn main() {
    unsafe { novuskinc::syscalls::sys_cwrite(0x46 as *const u8, 1) }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

struct Allocator {
    arena: SyncUnsafeCell<[u8; 128*1024]>,
    remaining: AtomicUsize,
}


unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > 4096 {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        (self.arena.get() as *mut u8).add(allocated)
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator {
    arena: SyncUnsafeCell::new([0x55; 128*1024]),
    remaining: AtomicUsize::new(128*1024),
};

#[lang = "start"]
fn start<T>(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}