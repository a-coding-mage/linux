/* SPDX-License-Identifier: GPL-2.0 */

// Translated from __ASM_SH_MIGOR_H.

pub const PORT_MSELCRA: u32 = 0xa405_0180;
pub const PORT_MSELCRB: u32 = 0xa405_0182;
pub const BSC_CS4BCR: u32 = 0xfec1_0010;
pub const BSC_CS6ABCR: u32 = 0xfec1_001c;
pub const BSC_CS4WCR: u32 = 0xfec1_0030;

// Dependency supplied by <video/sh_mobile_lcdc.h>.
use core::ffi::c_void;

#[repr(C)]
pub struct sh_mobile_lcdc_sys_bus_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn migor_lcd_qvga_setup(
        sys_ops_handle: *mut c_void,
        sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
