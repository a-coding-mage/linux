/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016,2017 IBM Corporation.
 */

// Translated from xive-internal.h. Required external kernel types and symbols
// are intentionally left as dependencies supplied by other translation units.

use core::ffi::{c_char, c_void};

/*
 * A "disabled" interrupt should never fire, to catch problems
 * we set its logical number to this
 */
pub const XIVE_BAD_IRQ: u32 = 0x7fffffff;
pub const XIVE_MAX_IRQ: u32 = XIVE_BAD_IRQ - 1;

/* Each CPU carry one of these with various per-CPU state */
#[repr(C)]
pub struct xive_cpu {
    #[cfg(feature = "CONFIG_SMP")]
    /* HW irq number and data of IPI */
    pub hw_ipi: u32,
    #[cfg(feature = "CONFIG_SMP")]
    pub ipi_data: xive_irq_data,

    pub chip_id: i32,

    /* Queue datas. Only one is populated */
    pub queue: [xive_q; XIVE_MAX_QUEUES],

    /*
     * Pending mask. Each bit corresponds to a priority that
     * potentially has pending interrupts.
     */
    pub pending_prio: u8,

    /* Cache of HW CPPR */
    pub cppr: u8,
}

pub const XIVE_MAX_QUEUES: usize = 8;

/* Backend ops */
#[repr(C)]
pub struct xive_ops {
    pub populate_irq_data: Option<unsafe extern "C" fn(u32, *mut xive_irq_data) -> i32>,
    pub configure_irq: Option<unsafe extern "C" fn(u32, u32, u8, u32) -> i32>,
    pub get_irq_config: Option<unsafe extern "C" fn(u32, *mut u32, *mut u8, *mut u32) -> i32>,
    pub setup_queue: Option<unsafe extern "C" fn(u32, *mut xive_cpu, u8) -> i32>,
    pub cleanup_queue: Option<unsafe extern "C" fn(u32, *mut xive_cpu, u8)>,
    pub prepare_cpu: Option<unsafe extern "C" fn(u32, *mut xive_cpu)>,
    pub setup_cpu: Option<unsafe extern "C" fn(u32, *mut xive_cpu)>,
    pub teardown_cpu: Option<unsafe extern "C" fn(u32, *mut xive_cpu)>,
    pub match_: Option<unsafe extern "C" fn(*mut device_node) -> bool>,
    pub shutdown: Option<unsafe extern "C" fn()>,

    pub update_pending: Option<unsafe extern "C" fn(*mut xive_cpu)>,
    pub sync_source: Option<unsafe extern "C" fn(u32)>,
    pub esb_rw: Option<unsafe extern "C" fn(u32, u32, u64, bool) -> u64>,
    #[cfg(feature = "CONFIG_SMP")]
    pub get_ipi: Option<unsafe extern "C" fn(u32, *mut xive_cpu) -> i32>,
    #[cfg(feature = "CONFIG_SMP")]
    pub put_ipi: Option<unsafe extern "C" fn(u32, *mut xive_cpu)>,
    pub debug_show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32>,
    pub debug_create: Option<unsafe extern "C" fn(*mut dentry) -> i32>,
    pub name: *const c_char,
}

extern "C" {
    pub fn xive_core_init(
        np: *mut device_node,
        ops: *const xive_ops,
        area: *mut c_void,
        offset: u32,
        max_prio: u8,
    ) -> bool;
    pub fn xive_queue_page_alloc(cpu: u32, queue_shift: u32) -> *mut u32;
    pub fn xive_core_debug_init() -> i32;
}

#[inline]
pub const fn xive_alloc_order(queue_shift: u32) -> u32 {
    if queue_shift > PAGE_SHIFT {
        queue_shift - PAGE_SHIFT
    } else {
        0
    }
}

extern "C" {
    pub static mut xive_cmdline_disabled: bool;
    pub static mut xive_has_save_restore: bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
