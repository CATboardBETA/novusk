#![no_std]

#[macro_use] extern crate novuskinc;
use rpi::RpiGpio;

pub fn sf_rpilcd_35_init() {
    let mut gpio = RpiGpio::new();
}

pub fn sf_rpilcd_35_end() {

}

// module_init!(sf_rpilcd_35_init);
// module_end!(sf_rpilcd_35_end);
