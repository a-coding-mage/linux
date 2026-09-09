// SPDX-License-Identifier: GPL-2.0-only
/*
 * First-level interrupt controller model for Hexagon.
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C dependencies: <linux/interrupt.h>, <asm/irq.h>, and <asm/hexagon_vm.h>.

#[repr(C)]
pub struct irq_data {
    pub irq: u32,
}

type IrqHandler = unsafe extern "C" fn(*mut irq_data);
type IrqWakeHandler = unsafe extern "C" fn(*mut irq_data, u32) -> i32;

#[repr(C)]
pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_mask: Option<IrqHandler>,
    pub irq_unmask: Option<IrqHandler>,
    pub irq_set_wake: Option<IrqWakeHandler>,
    pub irq_eoi: Option<IrqHandler>,
}

unsafe extern "C" {
    fn __vmintop_locdis(irq: isize);
    fn __vmintop_locen(irq: isize);
    fn __vmintop_globen(irq: isize);
    fn irq_set_chip_and_handler(irq: i32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn handle_fasteoi_irq();
    static HEXAGON_CPUINTS: i32;
}

unsafe extern "C" fn mask_irq(data: *mut irq_data) {
    __vmintop_locdis((*data).irq as isize);
}

unsafe extern "C" fn mask_irq_num(irq: u32) {
    __vmintop_locdis(irq as isize);
}

unsafe extern "C" fn unmask_irq(data: *mut irq_data) {
    __vmintop_locen((*data).irq as isize);
}

/*  This is actually all we need for handle_fasteoi_irq  */
unsafe extern "C" fn eoi_irq(data: *mut irq_data) {
    __vmintop_globen((*data).irq as isize);
}

/* Power mamangement wake call. We don't need this, however,
 * if this is absent, then an -ENXIO error is returned to the
 * msm_serial driver, and it fails to correctly initialize.
 * This is a bug in the msm_serial driver, but, for now, we
 * work around it here, by providing this bogus handler.
 * XXX FIXME!!! remove this when msm_serial is fixed.
 */
unsafe extern "C" fn set_wake(_data: *mut irq_data, _on: u32) -> i32 {
    0
}

static mut hexagon_irq_chip: irq_chip = irq_chip {
    name: b"HEXAGON\0".as_ptr() as *const core::ffi::c_char,
    irq_mask: Some(mask_irq),
    irq_unmask: Some(unmask_irq),
    irq_set_wake: Some(set_wake),
    irq_eoi: Some(eoi_irq),
};

/**
 * The hexagon core comes with a first-level interrupt controller
 * with 32 total possible interrupts.  When the core is embedded
 * into different systems/platforms, it is typically wrapped by
 * macro cells that provide one or more second-level interrupt
 * controllers that are cascaded into one or more of the first-level
 * interrupts handled here. The precise wiring of these other
 * irqs varies from platform to platform, and are set up & configured
 * in the platform-specific files.
 *
 * The first-level interrupt controller is wrapped by the VM, which
 * virtualizes the interrupt controller for us.  It provides a very
 * simple, fast & efficient API, and so the fasteoi handler is
 * appropriate for this case.
 */
pub unsafe extern "C" fn init_IRQ() {
    let mut irq: i32 = 0;

    while irq < HEXAGON_CPUINTS {
        mask_irq_num(irq as u32);
        irq_set_chip_and_handler(irq, &raw mut hexagon_irq_chip, handle_fasteoi_irq);
        irq += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
