// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

// Supplied by the corresponding Linux and PIC32 headers.
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
    fn irq_of_parse_and_map(node: *mut device_node, index: c_int) -> c_uint;
    fn of_node_put(node: *mut device_node);
    fn irq_create_mapping(domain: *mut core::ffi::c_void, hwirq: c_uint) -> c_uint;
    fn of_clk_init(np: *mut core::ffi::c_void);
    fn pic32_get_pbclk(index: c_uint) -> c_ulong;
    fn timer_probe();
}

extern "C" {
    static mut mips_hpt_frequency: c_ulong;
}

static PIC32_INFRA_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"microchip,pic32mzda-infra\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

const DEFAULT_CORE_TIMER_INTERRUPT: c_uint = 0;

unsafe fn pic32_xlate_core_timer_irq() -> c_uint {
    let node: *mut device_node;
    let irq: c_uint;

    node = of_find_matching_node(ptr::null_mut(), PIC32_INFRA_MATCH.as_ptr());

    if WARN_ON!(node.is_null()) {
        return irq_create_mapping(ptr::null_mut(), DEFAULT_CORE_TIMER_INTERRUPT);
    }

    irq = irq_of_parse_and_map(node, 0);

    of_node_put(node);

    if irq == 0 {
        return irq_create_mapping(ptr::null_mut(), DEFAULT_CORE_TIMER_INTERRUPT);
    }

    return irq;
}

pub unsafe fn get_c0_compare_int() -> c_uint {
    pic32_xlate_core_timer_irq()
}

pub unsafe fn plat_time_init() {
    let rate: c_ulong = pic32_get_pbclk(7);

    of_clk_init(ptr::null_mut());

    pr_info!("CPU Clock: %ldMHz\n", rate / 1_000_000);
    mips_hpt_frequency = rate / 2;

    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
