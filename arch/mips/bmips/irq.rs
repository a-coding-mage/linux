// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2014 Broadcom Corporation
 * Author: Kevin Cernekee <cernekee@gmail.com>
 */

use core::ffi::{c_char, c_void};

// External kernel declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

extern "C" {
    fn of_find_matching_node(
        from: *mut device_node,
        matches: *const of_device_id,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn irqchip_init();
    fn mips_cpu_irq_of_init();
    static mut bmips_tp1_irqs: ::core::ffi::c_int;
}

const CP0_LEGACY_COMPARE_IRQ: u32 = 7;

static SMP_INTC_DT_MATCH: [of_device_id; 3] = [
    of_device_id {
        compatible: b"brcm,bcm7038-l1-intc\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"brcm,bcm6345-l1-intc\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

pub unsafe fn get_c0_compare_int() -> u32 {
    CP0_LEGACY_COMPARE_IRQ
}

pub unsafe fn arch_init_irq() {
    let dn: *mut device_node;

    /* Only these controllers support SMP IRQ affinity */
    dn = of_find_matching_node(core::ptr::null_mut(), SMP_INTC_DT_MATCH.as_ptr());
    if !dn.is_null() {
        of_node_put(dn);
    } else {
        bmips_tp1_irqs = 0;
    }

    irqchip_init();
}

// IRQCHIP_DECLARE(mips_cpu_intc, "mti,cpu-interrupt-controller",
//                 mips_cpu_irq_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
