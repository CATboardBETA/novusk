use novuskinc::platform::early_device_init;
use crate::SetupReturn;

pub trait BootSetup {
    fn early_device_init(&self) -> SetupReturn {
        unsafe {
            if early_device_init() == 0 {
                (Ok(()), "Early device functions initialized and drivers set")
            } else { (Err("Early device init"), "Failed to initialize early device functions and set drivers") }
        }
    }

    fn early_serial_io_init(&self) -> SetupReturn {
        return (Ok(()), "Early I/O successfully initialized");
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        return (Ok(()), "Successfully setup linker memory");
    }

    fn disable_wdt(&self) -> SetupReturn {
        return (Ok(()), "DWT successfully initialized");
    }

    fn cpuid_init(&self) -> SetupReturn {
        return (Ok(()), "Successfully got and set CPU info");
    }

    fn early_cpu_init(&self) -> SetupReturn {
        return (Ok(()), "Successfully finished early CPU initialization");
    }
}

