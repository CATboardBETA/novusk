use device::Device;
use crate::LoFiveBoard;

impl Device for LoFiveBoard {
    fn name(&self) -> &'static str {
        return "LoFive";
    }

    fn serial_io_init(&self) {

    }

    fn gpio_init(&self) {

    }
}
