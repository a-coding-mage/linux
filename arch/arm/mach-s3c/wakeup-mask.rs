// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2010 Ben Dooks <ben-linux@fluff.org>
//
// Support for wakeup mask interrupts on newer SoCs

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct samsung_wakeup_mask {
    pub irq: c_int,
    pub bit: u32,
}

extern "C" {
    static NO_WAKEUP_IRQ: c_int;

    fn __raw_readl(reg: *mut c_void) -> u32;
    fn __raw_writel(val: u32, reg: *mut c_void);
    fn irq_get_irq_data(irq: c_int) -> *mut irq_data;
    fn irqd_is_wakeup_set(data: *mut irq_data) -> bool;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn samsung_sync_wakemask(
    reg: *mut c_void,
    mut mask: *const samsung_wakeup_mask,
    mut nr_mask: c_int,
) {
    let mut data: *mut irq_data;
    let mut val: u32;

    val = __raw_readl(reg);

    while nr_mask > 0 {
        if (*mask).irq == NO_WAKEUP_IRQ {
            val |= (*mask).bit;
            nr_mask -= 1;
            mask = mask.add(1);
            continue;
        }

        data = irq_get_irq_data((*mask).irq);

        // bit of a liberty to read this directly from irq_data.
        if irqd_is_wakeup_set(data) {
            val &= !(*mask).bit;
        } else {
            val |= (*mask).bit;
        }

        nr_mask -= 1;
        mask = mask.add(1);
    }

    let fmt = b"wakemask %08x => %08x\n\0";
    printk(fmt.as_ptr() as *const c_char, __raw_readl(reg), val);
    __raw_writel(val, reg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
