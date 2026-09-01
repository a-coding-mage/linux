/* SPDX-License-Identifier: GPL-2.0 */

/* Forward declarations from the original C header. */
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* register map */
pub const ADAU7118_REG_VENDOR_ID: u32 = 0x00;
pub const ADAU7118_REG_DEVICE_ID1: u32 = 0x01;
pub const ADAU7118_REG_DEVICE_ID2: u32 = 0x02;
pub const ADAU7118_REG_REVISION_ID: u32 = 0x03;
pub const ADAU7118_REG_ENABLES: u32 = 0x04;
pub const ADAU7118_REG_DEC_RATIO_CLK_MAP: u32 = 0x05;
pub const ADAU7118_REG_HPF_CONTROL: u32 = 0x06;
pub const ADAU7118_REG_SPT_CTRL1: u32 = 0x07;
pub const ADAU7118_REG_SPT_CTRL2: u32 = 0x08;
pub const ADAU7118_REG_DRIVE_STRENGTH: u32 = 0x11;
pub const ADAU7118_REG_RESET: u32 = 0x12;

#[inline]
pub const fn ADAU7118_REG_SPT_CX(num: u32) -> u32 {
    0x09u32.wrapping_add(num)
}

unsafe extern "C" {
    pub fn adau7118_probe(dev: *mut device, map: *mut regmap, hw_mode: bool) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
