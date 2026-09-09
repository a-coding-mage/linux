// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) BitBox Ltd 2010
 */

// External Linux kernel types and functions supplied by the surrounding tree.
use core::ffi::c_void;

#[repr(C)]
pub struct irq_chip_generic {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct mxc_extra_irq {
    pub set_irq_fiq: Option<unsafe extern "C" fn(irq: u64, irq_type: u32) -> i32>,
}

#[repr(C)]
pub struct irq_data {
    pub hwirq: u64,
}

extern "C" {
    fn irq_get_chip_data(irq: u32) -> *mut irq_chip_generic;
    fn irq_get_irq_data(irq: u32) -> *mut irq_data;
    fn irqd_to_hwirq(d: *const irq_data) -> u64;
}

#[no_mangle]
pub unsafe extern "C" fn mxc_set_irq_fiq(irq: u32, irq_type: u32) -> i32 {
    let mut gc: *mut irq_chip_generic;
    let mut exirq: *mut mxc_extra_irq;
    let ret: i32;

    ret = -38; // -ENOSYS

    gc = irq_get_chip_data(irq);
    if !gc.is_null() && !(*gc).private.is_null() {
        exirq = (*gc).private as *mut mxc_extra_irq;
        if (*exirq).set_irq_fiq.is_some() {
            let d: *mut irq_data = irq_get_irq_data(irq);
            ret = ((*exirq).set_irq_fiq.unwrap())(irqd_to_hwirq(d), irq_type);
        }
    }

    ret
}

// EXPORT_SYMBOL(mxc_set_irq_fiq);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
