#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

#[cfg(not(feature = "no_lang_items"))]
#[path = "../../lang.rs"]
pub(crate) mod lang;

use libc::c_ulong;

type rsize_t = c_ulong;

#[no_mangle]
pub extern "C" fn __use_types(_: rsize_t) {
    unimplemented!()
}
