/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_void;

extern "C" {
    pub fn pxa25x_clocks_init(regs: *mut c_void) -> core::ffi::c_int;
    pub fn pxa27x_clocks_init(regs: *mut c_void) -> core::ffi::c_int;
    pub fn pxa3xx_clocks_init(
        regs: *mut c_void,
        oscc_reg: *mut c_void,
    ) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_PXA3xx")]
extern "C" {
    pub fn pxa3xx_get_clk_frequency_khz(clock: core::ffi::c_int) -> core::ffi::c_uint;
    pub fn pxa3xx_clk_update_accr(
        disable: u32,
        enable: u32,
        xclkcfg: u32,
        mask: u32,
    );
}

#[cfg(not(feature = "CONFIG_PXA3xx"))]
#[inline]
pub fn pxa3xx_get_clk_frequency_khz(_x: core::ffi::c_int) -> core::ffi::c_uint {
    0
}

#[cfg(not(feature = "CONFIG_PXA3xx"))]
#[inline]
pub fn pxa3xx_clk_update_accr(
    _disable: u32,
    _enable: u32,
    _xclkcfg: u32,
    _mask: u32,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
