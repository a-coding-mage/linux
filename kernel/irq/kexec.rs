// SPDX-License-Identifier: GPL-2.0

// Linux kernel dependencies are supplied by other translation units.

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct irq_desc {
    pub irq_data: irq_data,
}

extern "C" {
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn irqd_is_started(data: *const irq_data) -> bool;
    fn irq_set_irqchip_state(irq: u32, which: u32, value: bool) -> i32;
    fn irqd_irq_inprogress(data: *const irq_data) -> bool;
    fn irq_shutdown(desc: *mut irq_desc);
}

// Build-time configuration: CONFIG_GENERIC_IRQ_KEXEC_CLEAR_VM_FORWARD.
// IRQCHIP_STATE_ACTIVE is supplied by the kernel IRQ definitions.
extern "C" {
    static IRQCHIP_STATE_ACTIVE: u32;
}

pub unsafe fn machine_kexec_mask_interrupts() {
    let mut desc: *mut irq_desc;
    let mut i: u32;

    // The for_each_irq_desc macro is supplied by the kernel IRQ implementation.
    for_each_irq_desc!(i, desc, {
        let chip: *mut irq_chip;
        let mut check_eoi: i32 = 1;

        chip = irq_desc_get_chip(desc);
        if chip.is_null() || !irqd_is_started(&(*desc).irq_data) {
            continue;
        }

        // if CONFIG_GENERIC_IRQ_KEXEC_CLEAR_VM_FORWARD is enabled:
        // First try to remove the active state from an interrupt which is forwarded
        // to a VM. If the interrupt is not forwarded, try to EOI the interrupt.
        #[cfg(CONFIG_GENERIC_IRQ_KEXEC_CLEAR_VM_FORWARD)]
        {
            check_eoi = irq_set_irqchip_state(i, IRQCHIP_STATE_ACTIVE, false);
        }

        if check_eoi != 0
            && (*chip).irq_eoi.is_some()
            && irqd_irq_inprogress(&(*desc).irq_data)
        {
            ((*chip).irq_eoi.unwrap())(&mut (*desc).irq_data);
        }

        irq_shutdown(desc);
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
