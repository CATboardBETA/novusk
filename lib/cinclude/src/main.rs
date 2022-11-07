#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[path = "../lang.rs"]
mod lang;

fn main() {
    panic!("crash and burn");
}
