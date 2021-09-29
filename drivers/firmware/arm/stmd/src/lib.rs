#![no_std]

#[macro_use] extern crate novuskinc;

use stm32f4xx_hal::pac::Peripherals;

#[cfg(feature = "stm32f401")]
pub(crate) const BOARD_MODLE: i32 = 01;

pub mod io;

pub fn stm32f4_init() {
    let mut peripherals = Peripherals::take();

    unsafe { Peripherals::steal(); }

    if peripherals.is_none() {
        panic!("Can't find peripherals");
    } else { printk!("Got peripherals"); }

    printk!("Before init");
    io::io_init();
}
