#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]


use wee_alloc::WeeAlloc;

fn main() {
    unsafe { novuskinc::syscalls::sys_write(0, 47, 1) }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[alloc_error_handler]
extern "C" fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = WeeAlloc::INIT;

#[lang = "start"]
extern "C" fn start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
{
    main();
    0
}