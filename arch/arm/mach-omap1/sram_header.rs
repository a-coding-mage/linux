/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

unsafe extern "C" {
    pub fn omap_sram_reprogram_clock(dpllctl: u32, ckctl: u32);

    pub fn omap1_sram_init() -> i32;
    pub fn omap_sram_push(funcp: *mut c_void, size: core::ffi::c_ulong) -> *mut c_void;

    // Do not use these
    pub fn omap1_sram_reprogram_clock(ckctl: u32, dpllctl: u32);
    pub static mut omap1_sram_reprogram_clock_sz: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
