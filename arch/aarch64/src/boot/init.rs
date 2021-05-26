use super::setup::setup;
use super::print::printk_init;
use crate::drivers::uefi_init;
use uefi::Handle;
use uefi::table::{Boot, SystemTable};
use uefi_kd::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&system_table);
    printk_init();
    uefi_init(image, system_table);
    printk!("Starting kernel...");
    kinfo!("UEFI services initialized");
    kinfo!("UEFI kernel drivers initialized");
    printk!("   UEFI version: {}.{}", UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION);
    setup()
}