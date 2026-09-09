/* SPDX-License-Identifier: GPL-2.0 */

// Translation of <linux/types.h> types used by this header.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct dove_pmu_domain_initdata {
    pub pwr_mask: u32,
    pub rst_mask: u32,
    pub iso_mask: u32,
    pub name: *const c_char,
}

#[repr(C)]
pub struct dove_pmu_initdata {
    pub pmc_base: *mut c_void,
    pub pmu_base: *mut c_void,
    pub irq: i32,
    pub irq_domain_start: i32,
    pub domains: *const dove_pmu_domain_initdata,
}

unsafe extern "C" {
    pub fn dove_init_pmu_legacy(initdata: *const dove_pmu_initdata) -> i32;
    pub fn dove_init_pmu() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
