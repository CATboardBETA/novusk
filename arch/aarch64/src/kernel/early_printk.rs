use core::fmt::{Arguments, Write};
use novuskinc::serial::SimpleUart;
use crate::include::dif::DIF;

extern "C" {
    static mut KERNEL_SIMPLEUART: SimpleUart;
}

#[no_mangle]
pub unsafe extern "C" fn _early64_printk(fmt: Arguments) {
   KERNEL_SIMPLEUART.write_fmt(fmt);
}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { unsafe { $crate::kernel::early_printk::_early64_printk(format_args!($($arg)*)); } }
}
