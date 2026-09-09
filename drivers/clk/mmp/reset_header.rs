/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <linux/reset-controller.h>.

pub const MMP_RESET_INVERT: ::core::ffi::c_uint = 1;

#[repr(C)]
pub struct mmp_clk_reset_cell {
    pub clk_id: ::core::ffi::c_uint,
    pub reg: *mut ::core::ffi::c_void,
    pub bits: u32,
    pub flags: ::core::ffi::c_uint,
    pub lock: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct mmp_clk_reset_unit {
    pub rcdev: reset_controller_dev,
    pub cells: *mut mmp_clk_reset_cell,
}

// The source is conditionally compiled by CONFIG_RESET_CONTROLLER.
#[cfg(feature = "CONFIG_RESET_CONTROLLER")]
extern "C" {
    pub fn mmp_clk_reset_register(
        np: *mut device_node,
        cells: *mut mmp_clk_reset_cell,
        nr_resets: ::core::ffi::c_int,
    );
}

#[cfg(not(feature = "CONFIG_RESET_CONTROLLER"))]
#[inline]
pub unsafe fn mmp_clk_reset_register(
    _np: *mut device_node,
    _cells: *mut mmp_clk_reset_cell,
    _nr_resets: ::core::ffi::c_int,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
