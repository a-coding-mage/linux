/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2015 Linaro Ltd.
 */

pub const IMX_CHIP_REVISION_1_0: u32 = 0x10;
pub const IMX_CHIP_REVISION_1_1: u32 = 0x11;
pub const IMX_CHIP_REVISION_1_2: u32 = 0x12;
pub const IMX_CHIP_REVISION_1_3: u32 = 0x13;
pub const IMX_CHIP_REVISION_1_4: u32 = 0x14;
pub const IMX_CHIP_REVISION_1_5: u32 = 0x15;
pub const IMX_CHIP_REVISION_2_0: u32 = 0x20;
pub const IMX_CHIP_REVISION_2_1: u32 = 0x21;
pub const IMX_CHIP_REVISION_2_2: u32 = 0x22;
pub const IMX_CHIP_REVISION_2_3: u32 = 0x23;
pub const IMX_CHIP_REVISION_3_0: u32 = 0x30;
pub const IMX_CHIP_REVISION_3_1: u32 = 0x31;
pub const IMX_CHIP_REVISION_3_2: u32 = 0x32;
pub const IMX_CHIP_REVISION_3_3: u32 = 0x33;
pub const IMX_CHIP_REVISION_UNKNOWN: u32 = 0xff;

unsafe extern "C" {
    pub fn mx25_revision() -> ::core::ffi::c_int;
    pub fn mx27_revision() -> ::core::ffi::c_int;
    pub fn mx31_revision() -> ::core::ffi::c_int;
    pub fn mx35_revision() -> ::core::ffi::c_int;
    pub fn mx51_revision() -> ::core::ffi::c_int;
    pub fn mx53_revision() -> ::core::ffi::c_int;

    pub fn imx_get_soc_revision() -> ::core::ffi::c_uint;
    pub fn imx_print_silicon_rev(cpu: *const ::core::ffi::c_char, srev: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
