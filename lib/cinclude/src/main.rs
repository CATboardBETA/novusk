#![no_std]
#![feature(alloc_error_handler)]
#![feature(lang_items)]

extern crate alloc;

#[path = "../lang.rs"]
mod lang;

fn main() {
    panic!("crash and burn");
}
