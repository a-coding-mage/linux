/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Firmware Assisted dump header file.
 *
 * Copyright 2011 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Opaque type supplied by the PowerPC register definitions. */
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/* CONFIG_FA_DUMP */
#[cfg(feature = "CONFIG_FA_DUMP")]
extern "C" {
    pub static mut crashing_cpu: c_int;

    pub fn is_fadump_memory_area(addr: u64, size: c_ulong) -> c_int;
    pub fn setup_fadump() -> c_int;
    pub fn is_fadump_active() -> c_int;
    pub fn should_fadump_crash() -> c_int;
    pub fn crash_fadump(regs: *mut pt_regs, string: *const c_char);
    pub fn fadump_cleanup();
    pub fn fadump_setup_param_area();
    pub fn fadump_append_bootargs();
}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn is_fadump_active() -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn should_fadump_crash() -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn crash_fadump(_regs: *mut pt_regs, _string: *const c_char) {}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn fadump_cleanup() {}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn fadump_setup_param_area() {}

#[cfg(not(feature = "CONFIG_FA_DUMP"))]
#[inline]
pub fn fadump_append_bootargs() {}

/* CONFIG_FA_DUMP || CONFIG_PRESERVE_FA_DUMP */
#[cfg(any(
    feature = "CONFIG_FA_DUMP",
    feature = "CONFIG_PRESERVE_FA_DUMP"
))]
extern "C" {
    pub fn early_init_dt_scan_fw_dump(
        node: c_ulong,
        uname: *const c_char,
        depth: c_int,
        data: *mut c_void,
    ) -> c_int;
    pub fn fadump_reserve_mem() -> c_int;
}

/* CONFIG_FA_DUMP && CONFIG_CMA */
#[cfg(all(feature = "CONFIG_FA_DUMP", feature = "CONFIG_CMA"))]
extern "C" {
    pub fn fadump_cma_init();
}

#[cfg(not(all(feature = "CONFIG_FA_DUMP", feature = "CONFIG_CMA")))]
#[inline]
pub fn fadump_cma_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
