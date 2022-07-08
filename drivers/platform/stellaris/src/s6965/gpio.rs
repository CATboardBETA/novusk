use core::arch::asm;
use super::leds::*;

pub const GPIO_PDDR: *mut u8 = 0x400F_F094 as *mut u8;
pub const GPIO_PDOR: *mut u8 = 0x400F_F080 as *mut u8;

pub unsafe fn gpio_init() {
    led_on();
}
