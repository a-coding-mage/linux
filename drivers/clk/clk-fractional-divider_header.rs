/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_ulong;

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static clk_fractional_divider_ops: clk_ops;

    pub fn clk_fractional_divider_general_approximation(
        hw: *mut clk_hw,
        rate: c_ulong,
        parent_rate: *mut c_ulong,
        m: *mut c_ulong,
        n: *mut c_ulong,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
