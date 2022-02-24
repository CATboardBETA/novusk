use super::idt::{idt_init, set_idt};

pub(crate) static mut IRQS: X64Irqs = X64Irqs::new();

pub struct X64Irqs {
    pub enabled: bool,
}

impl X64Irqs {
    pub const fn new() -> Self {
        return X64Irqs {
            enabled: false,
        };
    }

    pub fn enable(&mut self) {
        unsafe { asm!("sti"); }
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        unsafe { asm!("cli"); }
        self.enabled = false;
    }

    pub fn disable_and_halt(&mut self) {
        self.disable();
        unsafe { asm!("hlt"); }
    }
}

pub unsafe fn start_irq_setup() {
    let mut irqs = X64Irqs::new();
    irqs.disable();

    set_idt();
    idt_init();
}
