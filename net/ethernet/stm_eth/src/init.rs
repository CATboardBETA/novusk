use crate::StmEth;
use ethernet::EthNetDriver;

fn is_supported() -> bool {
    #[cfg(feature = "unsupported_stm32fxxx")]
    return false;

    #[cfg(not(feature = "stm32f407"))]
    return true;
}

fn stm_ethernet_init() {
    let mut eth = StmEth::new();
    let (name, author) = eth.driver.driver_info();

    if is_supported() {
        eth.init();
    } else { printk!("Driver: {} by: {} is not supported", name, author); }
}

define_ethernet_init!(stm_ethernet_init);
