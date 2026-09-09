/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the C header `reset-prcc.h`.
// The original include dependencies provide `reset_controller_dev`,
// `device_node`, and `CLKRST_MAX`.

use core::ffi::c_void;

/**
 * struct u8500_prcc_reset - U8500 PRCC reset controller state
 * @rcdev: reset controller device
 * @phy_base: the physical base address for each PRCC block
 * @base: the remapped PRCC bases
 */
#[repr(C)]
pub struct u8500_prcc_reset {
    pub rcdev: reset_controller_dev,
    pub phy_base: [u32; CLKRST_MAX],
    pub base: [*mut c_void; CLKRST_MAX],
}

unsafe extern "C" {
    pub fn u8500_prcc_reset_init(np: *mut device_node, ur: *mut u8500_prcc_reset);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
