/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: struct device is supplied by the surrounding kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn omap_rtc_power_off_program(dev: *mut device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
